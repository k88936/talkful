use anyhow::{anyhow, Context, Result};
use enigo::{Enigo, Keyboard, Settings};
use std::sync::{mpsc, Mutex};
use tokio::sync::oneshot;

use crate::asr::sherpa_asr_service::SherpaASRService;
use crate::asr::ASRService;
use crate::record::cpal_record_service::CPALRecordService;
use crate::record::{RecordResult, RecordService, RecordSignal};

const REFINE_PROMPT: &str = r#"
You are an expert editorial assistant. Your task is to transform raw speech-to-text transcripts into polished, professional written text.

Apply the following rules strictly:

1. CLEANSE: Remove all filler words (e.g., "um", "uh", "like", "you know"), stuttering, and false starts.
2. DEDUPLICATE: Remove unnecessary repetitions of words or phrases.
3. RESOLVE CORRECTIONS: If the speaker self-corrects (e.g., "go to the... no, wait, send the email"), keep only the final intended meaning ("send the email"). Discard the abandoned thought.
4. FORMAT: Structure the output logically. Use proper paragraph breaks for topic shifts, and correct punctuation/capitalization.
5. CLARIFY: Rephrase awkward or ambiguous phrasing for clarity and conciseness without changing the original intent or tone.

Output ONLY the final polished text. Do not include explanations, notes, or markdown code blocks.
"#;

struct ActiveRecording {
    stop_tx: oneshot::Sender<RecordSignal>,
    recorded_rx: oneshot::Receiver<Result<RecordResult>>,
}

struct ProcessingRequest {
    sample: RecordResult,
    response_tx: oneshot::Sender<Result<String>>,
}

pub struct RuntimePipeline {
    active_recording: Mutex<Option<ActiveRecording>>,
    process_tx: mpsc::Sender<ProcessingRequest>,
}

impl Default for RuntimePipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimePipeline {
    pub fn new() -> Self {
        let (process_tx, process_rx) = mpsc::channel::<ProcessingRequest>();
        std::thread::spawn(move || processing_worker(process_rx));
        Self {
            active_recording: Mutex::new(None),
            process_tx,
        }
    }

    pub fn start_recording(&self) -> Result<()> {
        let mut slot = self
            .active_recording
            .lock()
            .map_err(|_| anyhow!("runtime pipeline state poisoned"))?;
        if slot.is_some() {
            return Err(anyhow!("recording is already active"));
        }

        let (stop_tx, stop_rx) = oneshot::channel();
        let (recorded_tx, recorded_rx) = oneshot::channel();
        tauri::async_runtime::spawn(async move {
            let recorder = CPALRecordService::new();
            let recorded = recorder
                .record(stop_rx)
                .await
                .context("failed to record audio from input device");
            let _ = recorded_tx.send(recorded);
        });

        *slot = Some(ActiveRecording {
            stop_tx,
            recorded_rx,
        });
        Ok(())
    }

    pub fn stop_recording_and_process(&self) -> Result<oneshot::Receiver<Result<String>>> {
        let session = {
            let mut slot = self
                .active_recording
                .lock()
                .map_err(|_| anyhow!("runtime pipeline state poisoned"))?;
            slot.take()
                .ok_or_else(|| anyhow!("recording stop requested without active session"))?
        };

        session
            .stop_tx
            .send(RecordSignal::Stop)
            .map_err(|_| anyhow!("failed to send stop signal to recorder"))?;

        let process_tx = self.process_tx.clone();
        let (result_tx, result_rx) = oneshot::channel();
        tauri::async_runtime::spawn(async move {
            let processed = collect_then_process(session.recorded_rx, process_tx).await;
            let _ = result_tx.send(processed);
        });

        Ok(result_rx)
    }
}

async fn collect_then_process(
    recorded_rx: oneshot::Receiver<Result<RecordResult>>,
    process_tx: mpsc::Sender<ProcessingRequest>,
) -> Result<String> {
    let recorded = recorded_rx
        .await
        .context("recording task stopped before returning audio")??;
    let (response_tx, response_rx) = oneshot::channel();
    process_tx
        .send(ProcessingRequest {
            sample: recorded,
            response_tx,
        })
        .map_err(|_| anyhow!("processing worker stopped"))?;
    response_rx
        .await
        .context("processing worker dropped response channel")?
}

fn processing_worker(process_rx: mpsc::Receiver<ProcessingRequest>) {
    let initialized = initialize_services();
    match initialized {
        Ok(mut asr) => {
            for request in process_rx {
                let processed = transcribe_refine_inject(&mut asr, request.sample);
                let _ = request.response_tx.send(processed);
            }
        }
        Err(err) => {
            let init_err = format!("{err:#}");
            for request in process_rx {
                let _ = request.response_tx.send(Err(anyhow!(
                    "failed to initialize processing worker: {init_err}"
                )));
            }
        }
    }
}

fn initialize_services() -> Result<SherpaASRService> {
    let asr = SherpaASRService::new(None, None).context("failed to initialize ASR service")?;
    Ok(asr)
}

fn transcribe_refine_inject(asr: &mut SherpaASRService, recorded: RecordResult) -> Result<String> {
    let recognized = asr.asr(recorded).context("failed to transcribe audio")?;
    let refined = recognized;
    // let refined = refiner
    //     .refine(&recognized, REFINE_PROMPT)
    //     .context("failed to refine transcribed text")?;
    inject_text(&refined).context("failed to inject text to active window")?;
    Ok(refined)
}

fn inject_text(text: &str) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default()).context("failed to initialize enigo")?;
    enigo
        .text(text)
        .context("failed to type text with keyboard injection")?;
    Ok(())
}

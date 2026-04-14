pub mod sherpa_asr_processor;

use crate::config::get_base_path;
use crate::record::RecordResult;
use anyhow::Context;
use std::path::PathBuf;
use std::sync::{mpsc, Mutex};
use tokio::sync::oneshot;

fn get_model_base_path() -> PathBuf {
    get_base_path().join("models")
}
pub trait ASRProcessor {
    const TARGET_SAMPLE_RATE_HZ: u32 = 16_000;
    fn asr(&mut self, sample: RecordResult) -> String;
}

pub struct ASRService {
    request_tx: Mutex<mpsc::Sender<AsrRequest>>,
}

struct AsrRequest {
    sample: RecordResult,
    response_tx: oneshot::Sender<anyhow::Result<String>>,
}

impl ASRService {
    pub fn new<T>(
        build_processor: impl FnOnce() -> anyhow::Result<T> + Send + 'static,
    ) -> anyhow::Result<Self>
    where
        T: ASRProcessor + 'static,
    {
        let (request_tx, request_rx) = mpsc::channel::<AsrRequest>();
        let (init_tx, init_rx) = mpsc::sync_channel::<anyhow::Result<()>>(1);

        std::thread::Builder::new()
            .name("sherpa-asr-worker".to_string())
            .spawn(move || run_worker(request_rx, init_tx, build_processor))
            .context("failed to spawn sherpa-asr worker thread")?;

        init_rx
            .recv()
            .context("sherpa-asr worker exited before initialization")??;

        Ok(Self {
            request_tx: Mutex::new(request_tx),
        })
    }

    pub async fn transcribe(&self, sample: RecordResult) -> anyhow::Result<String> {
        let (response_tx, response_rx) = oneshot::channel();
        {
            let request_tx = self
                .request_tx
                .lock()
                .expect("sherpa-asr request sender poisoned");
            request_tx
                .send(AsrRequest {
                    sample,
                    response_tx,
                })
                .context("failed to send transcription request to sherpa-asr worker")?;
        }

        response_rx
            .await
            .context("sherpa-asr worker dropped transcription response")?
    }
}

fn run_worker<T>(
    request_rx: mpsc::Receiver<AsrRequest>,
    init_tx: mpsc::SyncSender<anyhow::Result<()>>,
    build_processor: impl FnOnce() -> anyhow::Result<T>,
) where
    T: ASRProcessor + 'static,
{
    let mut service = match build_processor() {
        Ok(service) => {
            init_tx
                .send(Ok(()))
                .expect("sherpa-asr init receiver dropped");
            service
        }
        Err(error) => {
            init_tx
                .send(Err(error))
                .expect("sherpa-asr init receiver dropped");
            return;
        }
    };

    while let Ok(request) = request_rx.recv() {
        let response = Ok(service.asr(request.sample));
        request
            .response_tx
            .send(response)
            .expect("sherpa-asr response receiver dropped");
    }
}

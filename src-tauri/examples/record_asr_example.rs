use anyhow::{Result};
use std::thread;
use std::time::Duration;
use talkful_lib::asr::ASRService;
use talkful_lib::asr::sherpa_asr_service::SherpaASRService;
use talkful_lib::record::cpal_record_service::CPALRecordService;
use talkful_lib::record::{RecordService, RecordSignal};

const RECORD_DURATION_SECONDS: u64 = 32;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {

    println!(
        "recording from the default input device for {} seconds",
        RECORD_DURATION_SECONDS
    );

    let (signal_tx, signal_rx) = tokio::sync::oneshot::channel();
    let duration = Duration::from_secs(RECORD_DURATION_SECONDS);
    let mut ASR = SherpaASRService::new(None, None)?;
    thread::spawn(move || {
        thread::sleep(duration);
        signal_tx
            .send(RecordSignal::Stop)
            .expect("recording task dropped before stop signal");
    });

    let recorder = CPALRecordService::new();
    let recorded = recorder.record(signal_rx).await?;
    let result = ASR.asr(recorded)?;
    println!("recognized: {result}");

    Ok(())
}

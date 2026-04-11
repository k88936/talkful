use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;
use talkful_lib::record::cpal_record_service::CPALRecordService;
use talkful_lib::record::{RecordService, RecordSignal};

const OUTPUT_FILE_NAME: &str = "/tmp/record_example.wav";
const RECORD_DURATION_SECONDS: u64 = 8;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let output_path = Path::new(OUTPUT_FILE_NAME);

    println!(
        "recording from the default input device for {} seconds",
        RECORD_DURATION_SECONDS
    );

    let (signal_tx, signal_rx) = tokio::sync::oneshot::channel();
    spawn_stop_timer(signal_tx, Duration::from_secs(RECORD_DURATION_SECONDS));

    let recorder = CPALRecordService::new();
    let recorded = recorder.record(signal_rx).await?;
    write_pcm16_mono_wav(&output_path, recorded.sample_rate, &recorded.samples)?;

    println!("saved wav file to {}", output_path.display());
    Ok(())
}

fn spawn_stop_timer(signal_tx: tokio::sync::oneshot::Sender<RecordSignal>, duration: Duration) {
    thread::spawn(move || {
        thread::sleep(duration);
        signal_tx
            .send(RecordSignal::Stop)
            .expect("recording task dropped before stop signal");
    });
}

const PCM16_BITS_PER_SAMPLE: u16 = 16;
const PCM16_BYTES_PER_SAMPLE: u32 = 2;
const MONO_CHANNELS: u16 = 1;

pub fn write_pcm16_mono_wav(
    path: impl AsRef<Path>,
    sample_rate_hz: u32,
    samples: &[f32],
) -> Result<()> {
    let path = path.as_ref();
    let sample_count = u32::try_from(samples.len()).context("too many samples for wav output")?;
    let data_chunk_size = sample_count
        .checked_mul(PCM16_BYTES_PER_SAMPLE)
        .context("wav data chunk size overflow")?;
    let riff_chunk_size = 36_u32
        .checked_add(data_chunk_size)
        .context("wav riff chunk size overflow")?;
    let byte_rate = sample_rate_hz
        .checked_mul(PCM16_BYTES_PER_SAMPLE)
        .context("wav byte rate overflow")?;
    let block_align = MONO_CHANNELS * (PCM16_BITS_PER_SAMPLE / 8);

    let mut file = File::create(path)
        .with_context(|| format!("failed to create wav file at {}", path.display()))?;

    file.write_all(b"RIFF")?;
    file.write_all(&riff_chunk_size.to_le_bytes())?;
    file.write_all(b"WAVE")?;
    file.write_all(b"fmt ")?;
    file.write_all(&16_u32.to_le_bytes())?;
    file.write_all(&1_u16.to_le_bytes())?;
    file.write_all(&MONO_CHANNELS.to_le_bytes())?;
    file.write_all(&sample_rate_hz.to_le_bytes())?;
    file.write_all(&byte_rate.to_le_bytes())?;
    file.write_all(&block_align.to_le_bytes())?;
    file.write_all(&PCM16_BITS_PER_SAMPLE.to_le_bytes())?;
    file.write_all(b"data")?;
    file.write_all(&data_chunk_size.to_le_bytes())?;

    for sample in samples {
        file.write_all(&quantize_pcm16(*sample).to_le_bytes())?;
    }

    Ok(())
}

fn quantize_pcm16(sample: f32) -> i16 {
    let clamped = sample.clamp(-1.0, 1.0);
    (clamped * i16::MAX as f32).round() as i16
}

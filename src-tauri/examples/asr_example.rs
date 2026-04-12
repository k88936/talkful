use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;
use talkful_lib::asr::sherpa_asr_service::SherpaASRService;
use talkful_lib::asr::ASRService;
use talkful_lib::record::RecordResult;

const INPUT_FILE_NAME: &str = "/tmp/record_example.wav";
const PCM16_BITS_PER_SAMPLE: u16 = 16;
const MONO_CHANNELS: u16 = 1;
const PCM_AUDIO_FORMAT: u16 = 1;

fn main() -> Result<()> {
    let sample = read_pcm16_mono_wav(INPUT_FILE_NAME)?;
    let mut asr = SherpaASRService::new()?;
    let result = asr.asr(sample);
    println!("recognized: {result}");
    Ok(())
}

fn read_pcm16_mono_wav(path: impl AsRef<Path>) -> Result<RecordResult> {
    let path = path.as_ref();
    let bytes =
        fs::read(path).with_context(|| format!("failed to read wav file at {}", path.display()))?;

    if bytes.len() < 12 {
        bail!("wav file too small");
    }
    if &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        bail!("unsupported wav header (expected RIFF/WAVE)");
    }

    let mut offset = 12usize;
    let mut sample_rate_hz = None;
    let mut pcm_bytes = None;

    while offset + 8 <= bytes.len() {
        let chunk_id = &bytes[offset..offset + 4];
        let chunk_size = le_u32(&bytes[offset + 4..offset + 8])? as usize;
        offset += 8;

        let chunk_end = offset
            .checked_add(chunk_size)
            .context("wav chunk size overflow")?;
        if chunk_end > bytes.len() {
            bail!("wav chunk extends beyond file size");
        }

        match chunk_id {
            b"fmt " => {
                if chunk_size < 16 {
                    bail!("wav fmt chunk is too small");
                }
                let fmt = &bytes[offset..chunk_end];
                let audio_format = le_u16(&fmt[0..2])?;
                let channels = le_u16(&fmt[2..4])?;
                let sample_rate = le_u32(&fmt[4..8])?;
                let bits_per_sample = le_u16(&fmt[14..16])?;

                if audio_format != PCM_AUDIO_FORMAT {
                    bail!("unsupported wav format: only PCM is supported");
                }
                if channels != MONO_CHANNELS {
                    bail!("unsupported wav channel count: only mono is supported");
                }
                if bits_per_sample != PCM16_BITS_PER_SAMPLE {
                    bail!("unsupported wav bit depth: only 16-bit PCM is supported");
                }
                sample_rate_hz = Some(sample_rate);
            }
            b"data" => {
                pcm_bytes = Some(&bytes[offset..chunk_end]);
            }
            _ => {}
        }

        // RIFF chunks are word-aligned, so odd-sized chunks include one padding byte.
        offset = chunk_end + (chunk_size % 2);
    }

    let sample_rate = sample_rate_hz.context("missing wav fmt chunk")?;
    let pcm = pcm_bytes.context("missing wav data chunk")?;
    if pcm.len() % 2 != 0 {
        bail!("invalid pcm16 data size");
    }

    let samples = pcm
        .chunks_exact(2)
        .map(|pair| {
            let sample = i16::from_le_bytes([pair[0], pair[1]]) as f32 / i16::MAX as f32;
            sample.clamp(-1.0, 1.0)
        })
        .collect();

    Ok(RecordResult {
        sample_rate,
        samples,
    })
}

fn le_u16(bytes: &[u8]) -> Result<u16> {
    Ok(u16::from_le_bytes(
        bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("expected 2 bytes"))?,
    ))
}

fn le_u32(bytes: &[u8]) -> Result<u32> {
    Ok(u32::from_le_bytes(
        bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("expected 4 bytes"))?,
    ))
}

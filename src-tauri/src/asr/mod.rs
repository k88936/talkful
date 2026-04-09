pub mod sherpa_asr_service;

use crate::record::RecordResult;
use anyhow::{Error, Result};
use rubato::audioadapter_buffers::direct::InterleavedSlice;
use rubato::{Async, FixedAsync, PolynomialDegree, Resampler};
use std::result;

pub trait ASRService {
    const TARGET_SAMPLE_RATE_HZ: u32 = 16_000;
    fn asr(&mut self, sample: RecordResult) -> Result<String, Error>;
}

fn resample_audio(audio: &[f32], from_rate: u32, to_rate: u32) -> result::Result<Vec<f32>, Error> {
    if from_rate == to_rate {
        return Ok(audio.to_vec());
    }

    let ratio = to_rate as f64 / from_rate as f64;
    let chunk_size = 1024;

    let mut resampler = Async::<f32>::new_poly(
        ratio,
        1.0,
        PolynomialDegree::Septic,
        chunk_size,
        1,
        FixedAsync::Input,
    )?;

    let input = InterleavedSlice::new(audio, 1, audio.len())?;
    let mut output = vec![0.0f32; resampler.process_all_needed_output_len(audio.len())];
    let output_frames = output.len();
    let mut output_adapter = InterleavedSlice::new_mut(&mut output, 1, output_frames)?;
    let (_input_frames, output_frames) =
        resampler.process_all_into_buffer(&input, &mut output_adapter, audio.len(), None)?;
    output.truncate(output_frames);

    Ok(output)
}

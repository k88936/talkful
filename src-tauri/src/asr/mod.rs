pub mod sherpa_asr_service;

use crate::record::RecordResult;
use anyhow::{Error, Result};

pub trait ASRService {
    const TARGET_SAMPLE_RATE_HZ: u32 = 16_000;
    fn asr(&mut self, sample: RecordResult) -> Result<String, Error>;
}

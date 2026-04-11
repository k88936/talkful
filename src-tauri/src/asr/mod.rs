pub mod sherpa_asr_service;

use crate::config::get_base_path;
use crate::record::RecordResult;
use std::path::PathBuf;

fn get_model_base_path() -> PathBuf {
    get_base_path().join("models")
}
pub trait ASRService {
    const TARGET_SAMPLE_RATE_HZ: u32 = 16_000;
    fn asr(&mut self, sample: RecordResult) -> String;
}

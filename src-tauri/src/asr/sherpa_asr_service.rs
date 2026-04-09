use sherpa_rs::paraformer::{ParaformerConfig, ParaformerRecognizer};
use anyhow::Error;
use crate::asr;
use crate::asr::ASRService;
use crate::record::RecordResult;

pub struct SherpaASRService {
    recognizer: ParaformerRecognizer,
}

impl SherpaASRService{
    pub fn new(model: Option<&str>, tokens: Option<&str>) -> anyhow::Result<Self> {
        let config = ParaformerConfig {
            model: model.unwrap_or("models/asr/model.int8.onnx").into(),
            tokens: tokens.unwrap_or("models/asr/tokens.txt").into(),
            num_threads: Some(8),
            ..Default::default()
        };
        let recognizer = ParaformerRecognizer::new(config).map_err(Error::msg)?;
        Ok(Self { recognizer })
    }

}

impl ASRService for SherpaASRService {
    const TARGET_SAMPLE_RATE_HZ: u32 = 16_000;
    fn asr(&mut self, sample: RecordResult) -> anyhow::Result<String, Error> {
        let resampled = asr::resample_audio(
            &(&sample.samples),
            sample.sample_rate,
            Self::TARGET_SAMPLE_RATE_HZ,
        )
        .expect("resample failed");
        Ok(self
            .recognizer
            .transcribe(Self::TARGET_SAMPLE_RATE_HZ, &resampled)
            .text)
    }
}

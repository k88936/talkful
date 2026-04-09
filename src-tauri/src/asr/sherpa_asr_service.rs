use crate::asr::ASRService;
use crate::record::RecordResult;
use anyhow::Error;
use sherpa_rs::paraformer::{ParaformerConfig, ParaformerRecognizer};

pub struct SherpaASRService {
    recognizer: ParaformerRecognizer,
}

impl SherpaASRService {
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
    fn asr(&mut self, sample: RecordResult) -> anyhow::Result<String, Error> {
        Ok(self
            .recognizer
            .transcribe(sample.sample_rate, &sample.samples)
            .text)
    }
}

use crate::asr::{get_model_base_path, ASRService};
use crate::record::RecordResult;
use anyhow::Error;
use sherpa_rs::paraformer::{ParaformerConfig, ParaformerRecognizer};

pub struct SherpaASRService {
    recognizer: ParaformerRecognizer,
}

impl SherpaASRService {
    pub fn new(model: &str, tokens: &str) -> anyhow::Result<Self> {
        let base_path = get_model_base_path();
        let config = ParaformerConfig {
            model: base_path.join(model).to_str().unwrap().into(),
            tokens: base_path.join(tokens).to_str().unwrap().into(),
            num_threads: Some(8),
            ..Default::default()
        };
        let recognizer = ParaformerRecognizer::new(config).map_err(Error::msg)?;
        Ok(Self { recognizer })
    }
}

impl ASRService for SherpaASRService {
    fn asr(&mut self, sample: RecordResult) -> String {
        self.recognizer
            .transcribe(sample.sample_rate, &sample.samples)
            .text
    }
}

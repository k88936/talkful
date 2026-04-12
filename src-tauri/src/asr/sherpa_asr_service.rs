use crate::asr::{get_model_base_path, ASRService};
use crate::record::RecordResult;
use anyhow::Error;
use sherpa_rs::paraformer::{ParaformerConfig, ParaformerRecognizer};

const ASR_MODEL_FILENAME: &str = "paraformer-offline.model.int8.onnx";
const ASR_TOKEN_FILENAME: &str = "paraformer-offline.tokens.txt";

pub struct SherpaASRService {
    recognizer: ParaformerRecognizer,
}

impl SherpaASRService {
    pub fn new() -> anyhow::Result<Self> {
        let base_path = get_model_base_path();
        let config = ParaformerConfig {
            model: base_path.join(ASR_MODEL_FILENAME).to_str().unwrap().into(),
            tokens: base_path.join(ASR_TOKEN_FILENAME).to_str().unwrap().into(),
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

use crate::asr::{get_model_base_path, ASRProcessor};
use crate::record::RecordResult;
use anyhow::{anyhow, Context};
use sherpa_onnx::{OfflineParaformerModelConfig, OfflineRecognizer, OfflineRecognizerConfig};
use std::convert::TryFrom;

const ASR_MODEL_FILENAME: &str = "paraformer-offline.model.int8.onnx";
const ASR_TOKEN_FILENAME: &str = "paraformer-offline.tokens.txt";

pub struct SherpaASRProcessor {
    recognizer: OfflineRecognizer,
}

impl SherpaASRProcessor {
    pub fn new() -> anyhow::Result<Self> {
        let base_path = get_model_base_path();
        let mut config = OfflineRecognizerConfig::default();
        let path = base_path.join(ASR_MODEL_FILENAME);
        if path.exists() {
            config.model_config.paraformer = OfflineParaformerModelConfig {
                model: Some(path.to_str().unwrap().to_owned()),
            };
        } else {
            return Err(anyhow!(
                "path {} is not exist, download paraformer model first",
                path.to_str().unwrap()
            ));
        }
        let path = base_path.join(ASR_TOKEN_FILENAME);
        if path.exists() {
            config.model_config.tokens = Some(path.to_str().unwrap().to_owned());
        } else {
            return Err(anyhow!(
                "path {} is not exist, download paraformer model first",
                path.to_str().unwrap()
            ));
        }
        config.model_config.num_threads = 8;

        let recognizer = OfflineRecognizer::create(&config)
            .context("failed to create sherpa-onnx recognizer")?;
        Ok(Self { recognizer })
    }
}

impl ASRProcessor for SherpaASRProcessor {
    fn asr(&mut self, sample: RecordResult) -> String {
        let stream = self.recognizer.create_stream();
        let sample_rate = i32::try_from(sample.sample_rate).expect("sample rate exceeds i32 range");
        stream.accept_waveform(sample_rate, &sample.samples);
        self.recognizer.decode(&stream);
        stream
            .get_result()
            .expect("sherpa-onnx did not return an ASR result")
            .text
    }
}

use crate::asr::{get_model_base_path, ASRService};
use crate::record::RecordResult;
use sherpa_onnx::{
    OfflineParaformerModelConfig, OfflineRecognizer, OfflineRecognizerConfig,
};
use std::convert::TryFrom;

const ASR_MODEL_FILENAME: &str = "paraformer-offline.model.int8.onnx";
const ASR_TOKEN_FILENAME: &str = "paraformer-offline.tokens.txt";

pub struct SherpaASRService {
    recognizer_config: OfflineRecognizerConfig,
}

impl SherpaASRService {
    pub fn new() -> anyhow::Result<Self> {
        let base_path = get_model_base_path();
        let mut config = OfflineRecognizerConfig::default();
        config.model_config.paraformer = OfflineParaformerModelConfig {
            model: Some(
                base_path
                    .join(ASR_MODEL_FILENAME)
                    .to_str()
                    .unwrap()
                    .to_owned(),
            ),
        };
        config.model_config.tokens = Some(
            base_path
                .join(ASR_TOKEN_FILENAME)
                .to_str()
                .unwrap()
                .to_owned(),
        );
        config.model_config.num_threads = 8;

        Ok(Self {
            recognizer_config: config,
        })
    }
}

impl ASRService for SherpaASRService {
    fn asr(&mut self, sample: RecordResult) -> String {
        let recognizer = OfflineRecognizer::create(&self.recognizer_config)
            .expect("failed to create sherpa-onnx recognizer");
        let stream = recognizer.create_stream();
        let sample_rate = i32::try_from(sample.sample_rate).expect("sample rate exceeds i32 range");
        stream.accept_waveform(sample_rate, &sample.samples);
        recognizer.decode(&stream);
        stream
            .get_result()
            .expect("sherpa-onnx did not return an ASR result")
            .text
    }
}

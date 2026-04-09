pub mod cpal_record_service;
use std::future::Future;
use anyhow::Error;
use tokio::sync::oneshot;


#[derive(Debug)]
pub enum RecordSignal {
    Stop,
}

pub struct RecordResult {
    pub sample_rate: u32,
    pub samples: Vec<f32>,
}

pub trait RecordService {
    fn record(&self, signal: oneshot::Receiver<RecordSignal>) -> impl Future<Output = Result<RecordResult, Error>>+Send;
}

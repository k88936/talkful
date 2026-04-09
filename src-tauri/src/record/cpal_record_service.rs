use std::future::Future;
use crate::record::{RecordResult, RecordService, RecordSignal};
use anyhow::Error;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample};
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

pub struct CPALRecordService;

impl CPALRecordService {
    pub fn new() -> Self {
        Self {}
    }
}

impl RecordService for CPALRecordService {
    fn record(&self, signal: oneshot::Receiver<RecordSignal>) -> impl Future<Output = Result<RecordResult, Error>> + Send {
        async move {
            let host = cpal::default_host();

            let device = host
                .default_input_device()
                .expect("failed to find input device");

            let config = device
                .default_input_config()
                .expect("Failed to get default input/output config");

            let channels = config.channels();
            let sample_rate = config.sample_rate().0;

            let buffer: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
            let buffer_append_handle = buffer.clone();

            let err_fn = |err| eprintln!("Stream error: {}", err);
            let stream = match config.sample_format() {
                cpal::SampleFormat::I8 => device.build_input_stream(
                    &config.clone().into(),
                    move |data, _: &_| collect_audio_data::<i8>(data, &buffer_append_handle, channels),
                    err_fn,
                    None,
                )?,
                cpal::SampleFormat::I16 => device.build_input_stream(
                    &config.clone().into(),
                    move |data, _: &_| collect_audio_data::<i16>(data, &buffer_append_handle, channels),
                    err_fn,
                    None,
                )?,
                cpal::SampleFormat::I32 => device.build_input_stream(
                    &config.clone().into(),
                    move |data, _: &_| collect_audio_data::<i32>(data, &buffer_append_handle, channels),
                    err_fn,
                    None,
                )?,
                cpal::SampleFormat::F32 => device.build_input_stream(
                    &config.clone().into(),
                    move |data, _: &_| collect_audio_data::<f32>(data, &buffer_append_handle, channels),
                    err_fn,
                    None,
                )?,
                sample_format => {
                    return Err(Error::msg(format!(
                        "Unsupported sample format '{sample_format}'"
                    )));
                }
            };
            stream.play()?;

            match signal.await.expect("signal sender is dropped") {
                RecordSignal::Stop => {
                    drop(stream);
                    let mut guard = buffer.lock().unwrap();
                    let samples = std::mem::take(&mut *guard);
                    Ok(RecordResult {
                        sample_rate,
                        samples,
                    })
                }
            }
        }
    }
}

fn collect_audio_data<T>(input: &[T], buffer: &Arc<Mutex<Vec<f32>>>, channels: u16)
where
    T: Sample,
    f32: FromSample<T>,
{
    if let Ok(mut guard) = buffer.lock() {
        if channels == 1 {
            guard.reserve(input.len());
            input
                .iter()
                .copied()
                .map(f32::from_sample)
                .for_each(|v| guard.push(v))
        } else {
            guard.reserve(input.len() / channels as usize);
            input
                .chunks_exact(channels as usize)
                .map(|chunk| f32::from_sample(chunk[0]))
                .for_each(|v| guard.push(v))
        };
    }
}


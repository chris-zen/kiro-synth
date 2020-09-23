use anyhow::Result;
use thiserror::Error;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BuildStreamError, DefaultStreamConfigError, PlayStreamError};
use cpal::{Device, OutputCallbackInfo, SampleRate, Stream, StreamConfig};

#[derive(Error, Debug)]
pub enum AudioError {
  #[error("No default output device")]
  NoDefaultOutputDevice,

  #[error("No default stream config")]
  NoDefaultStreamConfig(#[from] DefaultStreamConfigError),

  #[error("Error building stream")]
  BuildStream(#[from] BuildStreamError),

  #[error("Error playing stream")]
  PlayStream(#[from] PlayStreamError),
}

pub trait AudioHandler: Send {
  fn prepare(&mut self, len: usize);
  fn next(&mut self) -> (f32, f32);
  fn finalize(&mut self);
}

pub struct AudioDriver {
  _device: Device,
  _config: StreamConfig,
  _stream: Stream,
}

impl AudioDriver {
  pub fn new<Handler: AudioHandler + 'static>(
    sample_rate: u32,
    mut handler: Handler,
  ) -> Result<Self> {
    let host = cpal::default_host();

    let device = host
      .default_output_device()
      .ok_or(AudioError::NoDefaultOutputDevice)?;
    println!("Using default output device: '{}'", device.name()?);

    let mut config: StreamConfig = device
      .default_output_config()
      .map_err(AudioError::NoDefaultStreamConfig)?
      .into();

    let channels = config.channels as usize;

    config.sample_rate = SampleRate(sample_rate);
    println!("Using default output stream config: {:?}", config);

    let stream = device.build_output_stream(
      &config,
      move |data: &mut [f32], _: &OutputCallbackInfo| {
        handler.prepare(data.len());
        for sample in data.chunks_mut(channels) {
          let (left, right) = handler.next();
          sample[0] = left;
          if channels > 1 {
            sample[1] = right;
          }
          sample
            .iter_mut()
            .take(channels)
            .skip(2)
            .for_each(|s| *s = 0.0f32);
        }
        handler.finalize();
      },
      move |err| {
        eprintln!("an error occurred on stream: {}", err);
      },
    )?;

    stream.play().map_err(AudioError::PlayStream)?;

    Ok(AudioDriver {
      _device: device,
      _config: config,
      _stream: stream,
    })
  }
}

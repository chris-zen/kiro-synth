use anyhow::Result;
use thiserror::Error;

use cpal::{Device, StreamConfig, SampleRate, Stream};
use cpal::{DefaultStreamConfigError, BuildStreamError, PlayStreamError};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

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
}

pub struct AudioDriver {
  device: Device,
  config: StreamConfig,
  stream: Stream,
}

impl AudioDriver {
  pub fn new<Handler: AudioHandler + 'static>(sample_rate: u32, mut handler: Handler) -> Result<Self> {
    let host = cpal::default_host();

    let device = host
      .default_output_device()
      .ok_or(AudioError::NoDefaultOutputDevice)?;
    println!("Using default output device: '{}'", device.name()?);

    let mut config: StreamConfig = device.default_output_config()
      .map_err(|source| AudioError::NoDefaultStreamConfig(source))?
      .into();

    let channels = config.channels as usize;

    config.sample_rate = SampleRate(sample_rate);
    println!("Using default output stream config: {:?}", config);

    let stream = device.build_output_stream(
      &config,
      move |data: &mut [f32]| {
        handler.prepare(data.len());
        for sample in data.chunks_mut(channels) {
          let (left, right) = handler.next();
          sample[0] = left;
          if channels > 1 {
            sample[1] = right;
          }
          for i in 2..channels {
            sample[i] = 0.0f32;
          }
        }
      },
      move |err| {
        eprintln!("an error occurred on stream: {}", err);
      },
    )?;

    stream
      .play()
      .map_err(|err| AudioError::PlayStream(err))?;

    Ok(AudioDriver {
      device,
      config,
      stream,
    })
  }
}

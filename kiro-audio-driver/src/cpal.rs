use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Device, OutputCallbackInfo, SampleRate, Stream, StreamConfig};

use crate::{AudioConfig, AudioError, AudioHandler, Result};

pub struct AudioDriver {
  _device: Device,
  _config: StreamConfig,
  output_stream: Stream,
}

impl AudioDriver {
  pub fn new<Handler: AudioHandler + 'static>(
    config: AudioConfig,
    mut handler: Handler,
  ) -> Result<Self> {
    let host = cpal::default_host();

    let device = host
      .default_output_device()
      .ok_or(AudioError::NoDefaultOutputDevice)?;
    println!(
      "Using default output device: '{}'",
      device.name().unwrap_or_else(|_| "unknown".to_string())
    );

    let mut output_config: StreamConfig = device
      .default_output_config()
      .map_err(AudioError::NoDefaultStreamConfig)?
      .into();

    let channels = output_config.channels as usize;

    output_config.sample_rate = SampleRate(config.sample_rate as u32);
    output_config.buffer_size = BufferSize::Fixed(config.buffer_size as u32);
    println!("Using default output stream config: {:#?}", output_config);

    let output_stream = device.build_output_stream(
      &output_config,
      move |data: &mut [f32], _: &OutputCallbackInfo| handler.process(data, channels),
      move |err| eprintln!("an error occurred on stream: {:?}", err),
    )?;

    Ok(AudioDriver {
      _device: device,
      _config: output_config,
      output_stream,
    })
  }

  pub fn start(&self) -> Result<()> {
    self.output_stream.play().map_err(AudioError::PlayStream)
  }
}

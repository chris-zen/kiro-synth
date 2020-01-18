use anyhow::Result;
use thiserror::Error;

use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{BuildStreamError, DefaultFormatError, EventLoop, Format, PlayStreamError, StreamId};

#[derive(Error, Debug)]
pub enum AudioError {
  #[error("No default output device")]
  NoDefaultOutputDevice,

  #[error("No default format")]
  NoDefaultFormat(#[from] DefaultFormatError),

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
  format: Format,
  event_loop: EventLoop,
  _stream_id: StreamId,
}

impl AudioDriver {
  pub fn new(sample_rate: u32) -> Result<Self> {
    let host = cpal::default_host();
    let device = host
      .default_output_device()
      .ok_or(AudioError::NoDefaultOutputDevice)?;
    println!("Using default output device: '{}'", device.name()?);

    let mut format = device
      .default_output_format()
      .map_err(|source| AudioError::NoDefaultFormat(source))?;
    format.sample_rate = cpal::SampleRate(sample_rate);
    format.data_type = cpal::SampleFormat::F32;
    println!("Format: `{:?}`.", format);

    let event_loop = host.event_loop();

    let stream_id = event_loop
      .build_output_stream(&device, &format)
      .map_err(|source| AudioError::BuildStream(source))?;

    event_loop
      .play_stream(stream_id.clone())
      .map_err(|source| AudioError::PlayStream(source))?;

    Ok(AudioDriver {
      format,
      event_loop,
      _stream_id: stream_id,
    })
  }

  pub fn run<Handler: AudioHandler>(&self, mut handler: Handler) {
    self.event_loop.run(move |id, result| {
      let data = match result {
        Ok(data) => data,
        Err(err) => {
          eprintln!("an error occurred on stream {:?}: {}", id, err);
          return;
        }
      };

      match data {
        cpal::StreamData::Output {
          buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer),
        } => {
          handler.prepare(buffer.len());
          for sample in buffer.chunks_mut(self.format.channels as usize) {
            let (left, right) = handler.next();
            sample[0] = ((left * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
            sample[1] = ((right * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
          }
        }
        cpal::StreamData::Output {
          buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
        } => {
          handler.prepare(buffer.len());
          for sample in buffer.chunks_mut(self.format.channels as usize) {
            let (left, right) = handler.next();
            sample[0] = (left * std::u16::MAX as f32) as i16;
            sample[1] = (right * std::u16::MAX as f32) as i16;
          }
        }
        cpal::StreamData::Output {
          buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
        } => {
          handler.prepare(buffer.len());
          for sample in buffer.chunks_mut(self.format.channels as usize) {
            let (left, right) = handler.next();
            sample[0] = left;
            sample[1] = right;
          }
        }
        _ => (),
      }
    });
  }
}

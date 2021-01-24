use std::marker::PhantomData;
use std::ops::Deref;

use crate::buffers::Buffer;
use crate::controller::owned_data::Ref;
use crate::processor::ports::{Input, Output};


pub struct AudioRenderBuffer<IO> {
  num_samples: usize,
  buffer: Ref<Buffer>,
  _mode: PhantomData<IO>,
}

impl<IO> AudioRenderBuffer<IO> {
  pub fn new(num_samples: usize, buffer: Ref<Buffer>) -> Self {
    Self {
      num_samples,
      buffer,
      _mode: PhantomData,
    }
  }

  pub fn len(&self) -> usize {
    self.num_samples
  }
}

impl AudioRenderBuffer<Input> {
  pub fn iter<'a>(&'a self) -> impl Iterator<Item=&'a f32> + 'a {
    self
        .buffer
        .deref()
        .iter()
        .take(self.num_samples)
  }

  pub fn as_slice(&self) -> &[f32] {
    self
        .buffer
        .deref()
        .as_slice()[0..self.num_samples]
        .as_ref()
  }
}

impl AudioRenderBuffer<Output> {
  pub fn fill(&mut self, value: f32) {
    self
        .buffer
        .get_mut()
        .iter_mut()
        .take(self.num_samples)
        .for_each(|v| *v = value);
  }

  pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut f32> + 'a {
    self
        .buffer
        .get_mut()
        .iter_mut()
        .take(self.num_samples)
  }

  pub fn as_mut_slice(&mut self) -> &mut [f32] {
    self
        .buffer
        .get_mut()
        .as_mut_slice()[0..self.num_samples]
        .as_mut()
  }
}

#[derive(Debug, Clone)]
pub struct AudioRenderPort<'a, IO> {
  num_samples: usize,
  channels: &'a [Ref<Buffer>],
  _mode: PhantomData<IO>,
}

impl<'a, IO> AudioRenderPort<'a, IO> {
  pub fn new(num_samples: usize, channels: &'a [Ref<Buffer>]) -> Self {
    Self {
      num_samples,
      channels,
      _mode: PhantomData,
    }
  }

  pub fn len(&self) -> usize {
    self.channels.len()
  }
}

impl<'a> AudioRenderPort<'a, Input> {
  pub fn channel(&self, index: usize) -> AudioRenderBuffer<Input> {
    AudioRenderBuffer::new(self.num_samples, self.channels[index].clone())
  }
}

impl<'a> AudioRenderPort<'a, Output> {
  pub fn channel_mut(&self, index: usize) -> AudioRenderBuffer<Output> {
    AudioRenderBuffer::new(self.num_samples, self.channels[index].clone())
  }
}

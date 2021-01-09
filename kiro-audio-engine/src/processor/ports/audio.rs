use std::marker::PhantomData;
use std::ops::Deref;

use crate::buffers::Buffer;
use crate::controller::owned_data::Ref;
use crate::processor::ports::{Input, Output};

#[derive(Debug, Clone)]
pub struct AudioRenderPort<IO> {
  channels: Vec<Ref<Buffer>>,
  _mode: PhantomData<IO>,
}

impl<IO> AudioRenderPort<IO> {
  pub fn new(channels: Vec<Ref<Buffer>>) -> Self {
    Self {
      channels,
      _mode: PhantomData,
    }
  }

  pub fn len(&self) -> usize {
    self.channels.len()
  }
}

impl AudioRenderPort<Input> {
  pub fn channel(&self, index: usize) -> &Buffer {
    self.channels[index].deref()
  }
}

impl AudioRenderPort<Output> {
  pub fn channel_mut(&self, index: usize) -> &mut Buffer {
    self.channels[index].get_mut()
  }
}

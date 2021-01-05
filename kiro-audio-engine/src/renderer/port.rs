use std::marker::PhantomData;

use crate::buffers::Buffer;
use crate::controller::owned_data::Ref;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Input;

#[derive(Debug, Clone)]
pub struct Output;

#[derive(Debug, Clone)]
pub struct RenderPort<IO> {
  channels: Vec<Ref<Buffer>>,
  _mode: PhantomData<IO>,
}

impl<IO> RenderPort<IO> {
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

impl RenderPort<Input> {
  pub fn channel(&self, index: usize) -> &Buffer {
    self.channels[index].deref()
  }
}

impl RenderPort<Output> {
  pub fn channel_mut(&self, index: usize) -> &mut Buffer {
    self.channels[index].get_mut()
  }
}

use std::ops::{Deref, Index};
use std::sync::Arc;

use crate::buffers::Buffer;
use crate::controller::owned_data::Ref;
use crate::ParamValue;

#[derive(Debug, Clone)]
pub enum ParamRenderPort {
  Value(Arc<ParamValue>, Ref<Buffer>),
  Buffer(Ref<Buffer>),
}

impl ParamRenderPort {
  pub fn value(value: Arc<ParamValue>, slice_buffer: Ref<Buffer>) -> Self {
    ParamRenderPort::Value(value, slice_buffer)
  }

  pub fn buffer(buffer: Ref<Buffer>) -> Self {
    ParamRenderPort::Buffer(buffer)
  }

  pub fn iter(&self) -> Iter {
    match self {
      ParamRenderPort::Value(value, slice_buffer) => Iter::Value(value.get(), slice_buffer.len(), 0),
      ParamRenderPort::Buffer(buffer) => {
        let b = buffer.as_slice();
        Iter::Buffer(b.iter(), f32::MIN, true)
      }
    }
  }

  pub fn as_slice(&self) -> &[f32] {
    match self {
      ParamRenderPort::Value(value, slice_buffer) => {
        slice_buffer.get_mut().fill(value.get());
        slice_buffer.deref().as_slice()
      }
      ParamRenderPort::Buffer(buffer) => buffer.deref().as_slice(),
    }
  }
}

impl Index<usize> for ParamRenderPort {
  type Output = f32;

  fn index(&self, index: usize) -> &Self::Output {
    match self {
      ParamRenderPort::Value(value, buffer) => {
        let buff = buffer.get_mut().as_mut_slice();
        buff[0] = value.get();
        &buff[0]
      }
      ParamRenderPort::Buffer(buffer) => &buffer.deref().as_slice()[index],
    }
  }
}

#[derive(Debug)]
pub enum Iter<'a> {
  Value(f32, usize, usize),
  Buffer(core::slice::Iter<'a, f32>, f32, bool),
}

impl<'a> Iter<'a> {
  pub fn last_value(&self) -> f32 {
    match self {
      Iter::Value(value, _len, _index) => *value,
      Iter::Buffer(_iter, last_value, _updated) => *last_value,
    }
  }

  pub fn updated(&self) -> bool {
    match self {
      Iter::Value(_value, _len, index) => *index <= 1,
      Iter::Buffer(_iter, _last_value, updated) => *updated,
    }
  }

  pub fn next_if_updated<F>(&mut self, prev_value_fn: F) -> Option<f32>
  where
    F: Fn() -> f32,
  {
    self
      .next()
      .filter(|v| /* self.updated() && */ *v != (prev_value_fn)())
  }
}

impl<'a> Iterator for Iter<'a> {
  type Item = f32;

  fn next(&mut self) -> Option<Self::Item> {
    match self {
      Iter::Value(value, len, index) => {
        if *index < *len {
          *index += 1;
          Some(*value)
        } else {
          None
        }
      }
      Iter::Buffer(iter, last_value, updated) => {
        let next_value = iter.next().cloned();
        next_value.iter().for_each(|value| {
          *updated = *last_value != *value;
          *last_value = *value
        });
        next_value
      }
    }
  }
}

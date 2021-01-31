use std::ops::Deref;
use std::sync::Arc;

use kiro_audio_graph::Key;

use crate::buffers::Buffer;
use crate::controller::owned_data::Ref;
use crate::ParamValue;

#[derive(Debug, Clone)]
pub enum ParamData {
  FromValue(Arc<ParamValue>, Ref<Buffer>),
  FromOutput(Ref<Buffer>),
}

impl ParamData {
  pub fn from_value(value: Arc<ParamValue>, slice_buffer: Ref<Buffer>) -> Self {
    Self::FromValue(value, slice_buffer)
  }

  pub fn from_output(buffer: Ref<Buffer>) -> Self {
    Self::FromOutput(buffer)
  }

  pub fn allocated_buffer_key(&self) -> Option<Key<Buffer>> {
    match self {
      Self::FromValue(_, buffer) => Some(buffer.key),
      Self::FromOutput(_) => None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct ParamRenderPort {
  num_samples: usize,
  data: ParamData,
}

impl ParamRenderPort {
  pub fn new(data: ParamData) -> Self {
    Self {
      num_samples: 0,
      data,
    }
  }

  pub(crate) fn set_num_samples(&mut self, num_samples: usize) {
    self.num_samples = num_samples;
  }

  pub fn iter(&self) -> Iter {
    match &self.data {
      ParamData::FromValue(value, _slice_buffer) => Iter::Value {
        value: value.get(),
        len: self.num_samples,
        index: 0,
      },
      ParamData::FromOutput(buffer) => {
        let iter = buffer.deref().iter().take(self.num_samples);
        Iter::Buffer {
          iter,
          last_value: f32::MIN,
          updated: true,
        }
      }
    }
  }

  pub fn as_slice(&self) -> &[f32] {
    match &self.data {
      ParamData::FromValue(value, slice_buffer) => {
        slice_buffer
          .get_mut()
          .fill_first(self.num_samples, value.get());
        &slice_buffer.deref().as_slice()[0..self.num_samples]
      }
      ParamData::FromOutput(buffer) => &buffer.deref().as_slice()[0..self.num_samples],
    }
  }
}

// impl Index<usize> for ParamRenderPort {
//   type Output = f32;
//
//   fn index(&self, index: usize) -> &Self::Output {
//     match &self.data {
//       ParamData::FromValue(value, buffer) => {
//         let buff = buffer.get_mut().as_mut_slice();
//         buff[0] = value.get();
//         &buff[0]
//       }
//       ParamData::FromOutput(buffer) => &buffer.deref().as_slice()[index],
//     }
//   }
// }

#[derive(Debug)]
pub enum Iter<'a> {
  Value {
    value: f32,
    len: usize,
    index: usize,
  },
  Buffer {
    iter: core::iter::Take<core::slice::Iter<'a, f32>>,
    last_value: f32,
    updated: bool,
  },
}

impl<'a> Iter<'a> {
  pub fn last_value(&self) -> f32 {
    match self {
      Iter::Value {
        value,
        len: _,
        index: _,
      } => *value,
      Iter::Buffer {
        iter: _,
        last_value,
        updated: _,
      } => *last_value,
    }
  }

  pub fn updated(&self) -> bool {
    match self {
      Iter::Value {
        value: _,
        len: _,
        index,
      } => *index <= 1,
      Iter::Buffer {
        iter: _,
        last_value: _,
        updated,
      } => *updated,
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
      Iter::Value { value, len, index } => {
        if *index < *len {
          *index += 1;
          Some(*value)
        } else {
          None
        }
      }
      Iter::Buffer {
        iter,
        last_value,
        updated,
      } => {
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

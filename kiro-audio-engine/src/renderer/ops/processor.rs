use std::ops::{Deref, Index};

use crate::buffers::Buffer;

use crate::controller::owned_data::Ref;
use crate::controller::ProcParams;
use crate::renderer::port::{Input, Output, RenderPort};

type ParameterIndex = usize;

#[derive(Debug, Clone)]
pub enum Parameter {
  Value(Ref<ProcParams>, ParameterIndex, Ref<Buffer>),
  Buffer(Ref<Buffer>),
}

impl Parameter {
  pub fn value(params: Ref<ProcParams>, index: ParameterIndex, slice_buffer: Ref<Buffer>) -> Self {
    Parameter::Value(params, index, slice_buffer)
  }

  pub fn buffer(buffer: Ref<Buffer>) -> Self {
    Parameter::Buffer(buffer)
  }

  pub fn iter(&self) -> Iter {
    match self {
      Parameter::Value(params, index, buffer) => {
        let value = params.get(*index).map(|value| value.get()).unwrap_or(0.0);
        Iter::Value(value, buffer.len(), 0)
      }
      Parameter::Buffer(buffer) => {
        let b = buffer.as_slice();
        Iter::Buffer(b.iter(), f32::MIN, true)
      }
    }
  }
  pub fn as_slice(&self) -> &[f32] {
    match self {
      Parameter::Value(proc_params, param_index, slice_buffer) => {
        let value = proc_params.deref()[*param_index].get();
        slice_buffer.get_mut().fill(value);
        slice_buffer.deref().as_slice()
      }
      Parameter::Buffer(buffer) => buffer.deref().as_slice(),
    }
  }
}

impl Index<usize> for Parameter {
  type Output = f32;

  fn index(&self, index: usize) -> &Self::Output {
    match self {
      Parameter::Value(proc_params, param_index, buffer) => {
        let value = proc_params.deref()[*param_index].get();
        let buff = buffer.get_mut().as_mut_slice();
        buff[0] = value;
        &buff[0]
      }
      Parameter::Buffer(buffer) => &buffer.deref().as_slice()[index],
    }
  }
}

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
      Iter::Value(_value, _len, index) => *index == 0,
      Iter::Buffer(_iter, _last_value, updated) => *updated,
    }
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

#[derive(Debug, Clone)]
pub struct ProcessorContext {
  audio_inputs: Vec<RenderPort<Input>>,
  audio_outputs: Vec<RenderPort<Output>>,
  parameters: Vec<Parameter>,
}

impl ProcessorContext {
  pub fn new(
    audio_inputs: Vec<RenderPort<Input>>,
    audio_outputs: Vec<RenderPort<Output>>,
    parameters: Vec<Parameter>,
  ) -> Self {
    Self {
      audio_inputs,
      audio_outputs,
      parameters,
    }
  }

  pub fn num_audio_inputs(&self) -> usize {
    self.audio_inputs.len()
  }

  pub fn audio_input(&self, index: usize) -> &RenderPort<Input> {
    &self.audio_inputs[index]
  }

  pub fn num_audio_outputs(&self) -> usize {
    self.audio_outputs.len()
  }

  pub fn audio_output(&self, index: usize) -> &RenderPort<Output> {
    &self.audio_outputs[index]
  }

  pub fn num_parameters(&self) -> usize {
    self.parameters.deref().len()
  }

  pub fn parameter(&self, index: usize) -> &Parameter {
    &self.parameters[index]
  }
}

use crate::slab::Handle;
use crate::engine::refs::ParametersRef;
use crate::engine::renderer::port::{RenderPort, Input, Output};
use crate::engine::BufferCell;

type ParameterIndex = usize;

#[derive(Debug, Clone)]
pub enum ParameterSource {
  Value(ParameterIndex),
  Buffer(Handle<BufferCell>),
}

#[derive(Debug, Clone)]
pub struct ParametersContext {
  pub values: ParametersRef,
  pub sources: Vec<ParameterSource>,
}

impl ParametersContext {
  pub fn new(values: ParametersRef, sources: Vec<ParameterSource>) -> Self {
    Self {
      values,
      sources,
    }
  }
}

#[derive(Debug, Clone)]
pub struct RenderContext {
  audio_inputs: Vec<RenderPort<Input>>,
  audio_outputs: Vec<RenderPort<Output>>,
  pub parameters: ParametersContext,
}

impl RenderContext {
  pub fn new(
    audio_inputs: Vec<RenderPort<Input>>,
    audio_outputs: Vec<RenderPort<Output>>,
    parameters: ParametersContext,
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

  pub fn audio_output(&mut self, index: usize) -> &mut RenderPort<Output> {
    &mut self.audio_outputs[index]
  }

  // pub fn num_parameters(&self) -> usize {
  //   self.parameters.len()
  // }
  //
  // pub fn parameter(&mut self, index: usize) -> Ref<Buffer> {
  //   self.parameters[index].as_ref().borrow()
  // }
}

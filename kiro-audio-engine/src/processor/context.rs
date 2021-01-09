use std::ops::Deref;

use crate::processor::ports::audio::AudioRenderPort;
use crate::processor::ports::param::ParamRenderPort;
use crate::processor::ports::{Input, Output};

#[derive(Debug, Clone)]
pub struct ProcessorContext {
  audio_inputs: Vec<AudioRenderPort<Input>>,
  audio_outputs: Vec<AudioRenderPort<Output>>,
  parameters: Vec<ParamRenderPort>,
}

impl ProcessorContext {
  pub fn new(
    audio_inputs: Vec<AudioRenderPort<Input>>,
    audio_outputs: Vec<AudioRenderPort<Output>>,
    parameters: Vec<ParamRenderPort>,
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

  pub fn audio_input(&self, index: usize) -> &AudioRenderPort<Input> {
    &self.audio_inputs[index]
  }

  pub fn num_audio_outputs(&self) -> usize {
    self.audio_outputs.len()
  }

  pub fn audio_output(&self, index: usize) -> &AudioRenderPort<Output> {
    &self.audio_outputs[index]
  }

  pub fn num_parameters(&self) -> usize {
    self.parameters.deref().len()
  }

  pub fn parameter(&self, index: usize) -> &ParamRenderPort {
    &self.parameters[index]
  }
}

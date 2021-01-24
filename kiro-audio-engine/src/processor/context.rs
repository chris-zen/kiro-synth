use std::collections::HashMap;

use crate::processor::ports::audio::AudioRenderPort;
use crate::processor::ports::param::ParamRenderPort;
use crate::processor::ports::{Input, Output};

#[derive(Debug, Clone)]
pub struct ProcessorContext {
  pub(crate) audio_inputs: HashMap<String, AudioRenderPort<Input>>,
  pub(crate) audio_outputs: HashMap<String, AudioRenderPort<Output>>,
  pub(crate) parameters: HashMap<String, ParamRenderPort>,
}

impl ProcessorContext {
  pub fn new(
    audio_inputs: HashMap<String, AudioRenderPort<Input>>,
    audio_outputs: HashMap<String, AudioRenderPort<Output>>,
    parameters: HashMap<String, ParamRenderPort>,
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

  pub fn audio_inputs(&self) -> &HashMap<String, AudioRenderPort<Input>> {
    &self.audio_inputs
  }

  pub fn audio_input<'a, ID: Into<&'a str>>(&self, id: ID) -> &AudioRenderPort<Input> {
    &self.audio_inputs[id.into()]
  }

  pub fn num_audio_outputs(&self) -> usize {
    self.audio_outputs.len()
  }

  pub fn audio_outputs(&self) -> &HashMap<String, AudioRenderPort<Output>> {
    &self.audio_outputs
  }

  pub fn audio_output<'a, ID: Into<&'a str>>(&self, id: ID) -> &AudioRenderPort<Output> {
    &self.audio_outputs[id.into()]
  }

  pub fn num_parameters(&self) -> usize {
    self.parameters.len()
  }

  pub fn parameters(&self) -> &HashMap<String, ParamRenderPort> {
    &self.parameters
  }

  pub fn parameter<'a, ID: Into<&'a str>>(&self, id: ID) -> &ParamRenderPort {
    &self.parameters[id.into()]
  }
}

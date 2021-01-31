use std::collections::HashMap;

use crate::processor::ports::audio::AudioRenderPort;
use crate::processor::ports::param::ParamRenderPort;
use crate::processor::ports::{Input, Output};

pub struct RenderContext<'a> {
  num_samples: usize,
  audio_inputs: &'a HashMap<String, AudioRenderPort<Input>>,
  audio_outputs: &'a HashMap<String, AudioRenderPort<Output>>,
  parameters: &'a HashMap<String, ParamRenderPort>,
}

impl<'a> RenderContext<'a> {
  pub fn new(
    num_samples: usize,
    audio_inputs: &'a HashMap<String, AudioRenderPort<Input>>,
    audio_outputs: &'a HashMap<String, AudioRenderPort<Output>>,
    parameters: &'a HashMap<String, ParamRenderPort>,
  ) -> Self {
    Self {
      num_samples,
      audio_inputs,
      audio_outputs,
      parameters,
    }
  }

  pub fn num_samples(&self) -> usize {
    self.num_samples
  }

  pub fn num_audio_inputs(&self) -> usize {
    self.audio_inputs.len()
  }

  pub fn audio_input<'b, ID: Into<&'b str>>(&self, id: ID) -> &'a AudioRenderPort<Input> {
    &self.audio_inputs[id.into()]
  }

  pub fn num_audio_outputs(&self) -> usize {
    self.audio_outputs.len()
  }

  pub fn audio_output<'b, ID: Into<&'b str>>(&self, id: ID) -> &'a AudioRenderPort<Output> {
    &self.audio_outputs[id.into()]
  }

  pub fn num_parameters(&self) -> usize {
    self.parameters.len()
  }

  pub fn parameter<'b, ID: Into<&'b str>>(&self, id: ID) -> &'a ParamRenderPort {
    &self.parameters[id.into()]
  }
}

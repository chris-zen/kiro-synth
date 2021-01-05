use crate::controller::handles::{BufferHandle, ParametersHandle, ProcessorHandle};

pub enum ParamSource {
  Value(ParametersHandle, usize, BufferHandle),
  Buffer(BufferHandle),
}

pub enum Operation {
  RenderProcessor {
    processor: ProcessorHandle,
    audio_inputs: Vec<Vec<BufferHandle>>,
    audio_outputs: Vec<Vec<BufferHandle>>,
    parameters: Vec<ParamSource>,
  },
  RenderOuput {
    audio_inputs: Vec<BufferHandle>,
  },
}

pub struct ControllerPlan {
  pub(crate) operations: Vec<Operation>,
}

impl Default for ControllerPlan {
  fn default() -> Self {
    Self {
      operations: Vec::new(),
    }
  }
}

impl ControllerPlan {
  pub fn new(operations: Vec<Operation>) -> Self {
    Self { operations }
  }

  pub fn render_processor(
    &mut self,
    processor: ProcessorHandle,
    audio_inputs: Vec<Vec<BufferHandle>>,
    audio_outputs: Vec<Vec<BufferHandle>>,
    parameters: Vec<ParamSource>,
  ) {
    self.operations.push(Operation::RenderProcessor {
      processor,
      audio_inputs,
      audio_outputs,
      parameters,
    })
  }

  pub fn render_output(&mut self, audio_inputs: Vec<BufferHandle>) {
    self
      .operations
      .push(Operation::RenderOuput { audio_inputs })
  }
}

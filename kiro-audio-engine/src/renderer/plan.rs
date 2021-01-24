use crate::controller::owned_data::Ref;
use crate::processor::ports::param::ParamRenderPort;
use crate::processor::ProcessorBox;
use std::collections::HashMap;
use crate::buffers::Buffer;

#[derive(Debug, Clone)]
pub enum RenderOp {
  // RenderInput(Vec<Ref<Buffer>),
  RenderOutput {
    alias: String,
    audio_input: Vec<Ref<Buffer>>,
  },
  RenderProcessor {
    processor_ref: Ref<ProcessorBox>,
    audio_inputs: HashMap<String, Vec<Ref<Buffer>>>,
    audio_outputs: HashMap<String, Vec<Ref<Buffer>>>,
    parameters: HashMap<String, ParamRenderPort>,
  },
  // WaitProcessor(usize),
}

#[derive(Debug)]
pub struct RenderPlan {
  pub operations: Vec<RenderOp>,
}

impl RenderPlan {
  pub fn new(operations: Vec<RenderOp>) -> Self {
    Self { operations }
  }
}

impl Default for RenderPlan {
  fn default() -> Self {
    Self::new(Vec::new())
  }
}

use crate::controller::owned_data::Ref;
use crate::processor::context::ProcessorContext;
use crate::processor::ports::audio::AudioRenderPort;
use crate::processor::ports::Input;
use crate::processor::ProcessorBox;

#[derive(Debug, Clone)]
pub enum RenderOp {
  // RenderInput(RenderPort<Output>),
  RenderOutput(AudioRenderPort<Input>),
  RenderProcessor(Ref<ProcessorBox>, ProcessorContext),
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

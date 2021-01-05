use crate::controller::owned_data::Ref;
use crate::processor::ProcessorBox;
use crate::renderer::ops::processor::ProcessorContext;
use crate::renderer::port::{Input, RenderPort};

#[derive(Debug, Clone)]
pub enum RenderOp {
  // RenderInput(RenderPort<Output>),
  RenderOutput(RenderPort<Input>),
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

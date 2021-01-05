use crate::renderer::plan::RenderPlan;

// #[derive(Debug, Clone)]
pub enum Message {
  MoveRenderPlan(Box<RenderPlan>),
}

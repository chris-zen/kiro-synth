pub mod context;
pub mod ports;

use std::fmt::Formatter;

use kiro_audio_graph::NodeDescriptor;

use crate::processor::context::ProcessorContext;

pub type ProcessorBox = Box<dyn Processor + 'static>;

pub trait Processor {
  fn render(&mut self, context: &mut ProcessorContext);
}

impl std::fmt::Debug for dyn Processor {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("Processor()")
  }
}

pub trait ProcessorFactory {
  fn supported_classes(&self) -> Vec<String>;
  fn create(&self, descriptor: &NodeDescriptor) -> Option<Box<dyn Processor>>;
}

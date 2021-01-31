pub(crate) mod context;
pub mod ports;

use std::collections::HashMap;
use std::fmt::Formatter;

use kiro_audio_graph::Node;

pub use context::RenderContext;

pub type ProcessorBox = Box<dyn Processor + 'static>;

pub trait Processor {
  fn render(&mut self, context: &mut RenderContext);
}

impl std::fmt::Debug for dyn Processor {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("Processor()")
  }
}

pub trait ProcessorFactory {
  fn supported_classes(&self) -> Vec<String>;
  fn create(&self, node: &Node) -> Option<Box<dyn Processor>>;
}

pub struct GenericProcessorFactory {
  factories: HashMap<String, Box<dyn Fn(&Node) -> Option<Box<dyn Processor>>>>,
}

impl GenericProcessorFactory {
  pub fn new() -> Self {
    Self {
      factories: HashMap::new(),
    }
  }

  pub fn with_factory<C>(
    mut self,
    class: C,
    create: impl Fn(&Node) -> Option<Box<dyn Processor>> + 'static,
  ) -> Self
  where
    C: Into<String>,
  {
    self.factories.insert(class.into(), Box::new(create));
    self
  }
}

impl ProcessorFactory for GenericProcessorFactory {
  fn supported_classes(&self) -> Vec<String> {
    self.factories.keys().cloned().collect()
  }

  fn create(&self, node: &Node) -> Option<Box<dyn Processor>> {
    self
      .factories
      .get(node.descriptor().class())
      .and_then(|create| (create)(node))
  }
}

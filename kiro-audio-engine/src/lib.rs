pub mod buffers; // TODO make it private
mod config;
pub mod controller; // TODO make it private
mod messages;
pub mod param_value; // TODO make it private
pub mod processor;
pub mod renderer;

use ringbuf::RingBuffer;

use crate::buffers::Buffer;
pub use crate::config::EngineConfig;
pub use crate::controller::Controller;
pub use crate::param_value::ParamValue;
pub use crate::processor::Processor;
pub use crate::processor::ProcessorFactory;
pub use crate::renderer::Renderer;

pub type BufferBox = Box<Buffer>;
pub type ParametersBox = Box<Vec<f32>>;

pub struct Engine {
  controller: Controller,
  renderer: Renderer,
}

impl Engine {
  pub fn new() -> Self {
    Self::with_config(EngineConfig::default())
  }

  pub fn with_config(config: EngineConfig) -> Self {
    let ring_buffer_capacity = config.ring_buffer_capacity;
    let (forward_tx, forward_rx) = RingBuffer::new(ring_buffer_capacity).split();
    let (backward_tx, backward_rx) = RingBuffer::new(ring_buffer_capacity).split();
    let controller = Controller::new(forward_tx, backward_rx, config.clone());
    let renderer = Renderer::new(backward_tx, forward_rx, config);

    Self {
      controller,
      renderer,
    }
  }

  pub fn split(self) -> (Controller, Renderer) {
    let Self {
      controller,
      renderer,
    } = self;
    (controller, renderer)
  }
}

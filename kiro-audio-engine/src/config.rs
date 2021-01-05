#[derive(Debug, Clone)]
pub struct EngineConfig {
  pub ring_buffer_capacity: usize,
  pub buffer_size: usize,
}

impl EngineConfig {
  const DEFAULT_RING_BUFFER_CAPACITY: usize = 1024;
  const DEFAULT_BUFFER_SIZE: usize = 256;
}

impl Default for EngineConfig {
  fn default() -> Self {
    Self {
      ring_buffer_capacity: EngineConfig::DEFAULT_RING_BUFFER_CAPACITY,
      buffer_size: EngineConfig::DEFAULT_BUFFER_SIZE,
    }
  }
}

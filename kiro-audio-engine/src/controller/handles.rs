use kiro_audio_graph::Key;
use crate::buffers::Buffer;
use crate::BufferBox;
use crate::processor::ProcessorBox;
use crate::controller::ProcParams;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BufferHandle(pub(crate) Key<Buffer>);

impl Into<Key<Buffer>> for BufferHandle {
  fn into(self) -> Key<Buffer> {
    self.0
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProcessorHandle(pub(crate) Key<ProcessorBox>);

impl Into<Key<ProcessorBox>> for ProcessorHandle {
  fn into(self) -> Key<ProcessorBox> {
    self.0
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParametersHandle(pub(crate) Key<ProcParams>);

impl Into<Key<ProcParams>> for ParametersHandle {
  fn into(self) -> Key<ProcParams> {
    self.0
  }
}

use crate::program::ParamBlock;

#[derive(Debug, Clone, PartialEq, Eq, Hash32, Copy)]
pub struct SignalRef(pub(crate) usize);

impl Into<usize> for SignalRef {
  fn into(self) -> usize {
    self.0
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash32, Copy)]
pub struct SourceRef(pub(crate) usize);

impl SourceRef {
  pub fn new(reference: usize) -> Self {
    SourceRef(reference)
  }
}

impl Into<usize> for SourceRef {
  fn into(self) -> usize {
    self.0
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash32, Copy)]
pub struct ParamRef(pub(crate) usize);

impl ParamRef {
  pub fn new(reference: usize) -> Self {
    ParamRef(reference)
  }
}

impl From<ParamBlock> for ParamRef {
  fn from(block: ParamBlock) -> Self {
    block.reference
  }
}

impl From<&ParamBlock> for ParamRef {
  fn from(block: &ParamBlock) -> Self {
    block.reference
  }
}

impl Into<usize> for ParamRef {
  fn into(self) -> usize {
    self.0
  }
}

#[derive(Debug, Clone, Copy)]
pub struct BlockRef(pub(crate) usize);

pub struct SignalRefs(usize);

impl SignalRefs {
  pub fn new() -> Self {
    SignalRefs(0)
  }

  pub fn create(&mut self) -> SignalRef {
    let reference = SignalRef(self.0);
    self.0 += 1;
    reference
  }

  pub fn count(&self) -> usize {
    self.0
  }
}

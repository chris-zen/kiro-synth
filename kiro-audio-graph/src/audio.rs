use crate::key_gen::Key;
use crate::key_store::HasId;
use crate::node::NodeRef;
use crate::port::{AudioInPort, AudioOutPort};

#[derive(Debug, Clone, Copy)]
pub struct AudioInRef {
  pub(crate) node_ref: NodeRef,
  pub(crate) audio_key: Key<AudioInPort>,
}

impl AudioInRef {
  pub fn new(node_ref: NodeRef, audio_key: Key<AudioInPort>) -> Self {
    Self {
      node_ref,
      audio_key,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct AudioOutRef {
  pub(crate) node_ref: NodeRef,
  pub(crate) audio_key: Key<AudioOutPort>,
}

impl AudioOutRef {
  pub fn new(node_ref: NodeRef, audio_key: Key<AudioOutPort>) -> Self {
    Self {
      node_ref,
      audio_key,
    }
  }
}

#[derive(Debug, Clone)]
pub struct AudioDescriptor {
  id: String,
  channels: usize,
}

impl AudioDescriptor {
  pub fn new<S: Into<String>>(id: S, channels: usize) -> Self {
    Self {
      id: id.into(),
      channels,
    }
  }

  pub fn channels(&self) -> usize {
    self.channels
  }
}

impl HasId for AudioDescriptor {
  fn id(&self) -> &str {
    self.id.as_str()
  }
}

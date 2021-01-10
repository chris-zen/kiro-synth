use crate::key_gen::Key;
use crate::key_store::HasId;
use crate::node::NodeRef;
use crate::port::{AudioInPort, AudioOutPort};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioInRef {
  pub node_ref: NodeRef,
  pub audio_port_key: Key<AudioInPort>,
}

impl AudioInRef {
  pub fn new(node_ref: NodeRef, audio_port_key: Key<AudioInPort>) -> Self {
    Self {
      node_ref,
      audio_port_key,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioOutRef {
  pub node_ref: NodeRef,
  pub audio_port_key: Key<AudioOutPort>,
}

impl AudioOutRef {
  pub fn new(node_ref: NodeRef, audio_port_key: Key<AudioOutPort>) -> Self {
    Self {
      node_ref,
      audio_port_key,
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

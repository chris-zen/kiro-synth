use crate::key_gen::Key;
use crate::key_store::HasId;
use crate::node::NodeRef;
use crate::port::{MidiInPort, MidiOutPort};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MidiInRef {
  pub(crate) node_ref: NodeRef,
  pub(crate) midi_port_key: Key<MidiInPort>,
}

impl MidiInRef {
  pub fn new(node_ref: NodeRef, midi_port_key: Key<MidiInPort>) -> Self {
    Self {
      node_ref,
      midi_port_key,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MidiOutRef {
  pub(crate) node_ref: NodeRef,
  pub(crate) midi_port_key: Key<MidiOutPort>,
}

impl MidiOutRef {
  pub fn new(node_ref: NodeRef, midi_port_key: Key<MidiOutPort>) -> Self {
    Self {
      node_ref,
      midi_port_key,
    }
  }
}

#[derive(Debug, Clone)]
pub struct MidiDescriptor {
  id: String,
}

impl MidiDescriptor {
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self { id: id.into() }
  }
}

impl HasId for MidiDescriptor {
  fn id(&self) -> &str {
    self.id.as_str()
  }
}

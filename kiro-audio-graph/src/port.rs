use std::marker::PhantomData;

use crate::audio::{AudioDescriptor, AudioInRef, AudioOutRef};
use crate::key_store::{HasId, KeyStoreWithId};
use crate::midi::{MidiDescriptor, MidiInRef, MidiOutRef};
use crate::param::ParamDescriptor;

pub type ParamPort = InputPort<ParamDescriptor, AudioOutRef>;
pub type ParamPortStore = KeyStoreWithId<ParamPort>;

pub type AudioInPort = InputPort<AudioDescriptor, AudioOutRef>;
pub type AudioInPortStore = KeyStoreWithId<AudioInPort>;

pub type AudioOutPort = OutputPort<AudioDescriptor, AudioInRef>;
pub type AudioOutPortStore = KeyStoreWithId<AudioOutPort>;

pub type MidiInPort = InputPort<MidiDescriptor, MidiOutRef>;
pub type MidiInPortStore = KeyStoreWithId<MidiInPort>;

pub type MidiOutPort = OutputPort<MidiDescriptor, MidiInRef>;
pub type MidiOutPortStore = KeyStoreWithId<MidiOutPort>;

#[derive(Debug)]
pub struct InputPort<D, C> {
  pub(crate) descriptor: D,
  pub(crate) connection: Option<C>,
}

impl<D, C> InputPort<D, C> {
  pub fn new(descriptor: D) -> Self {
    Self {
      descriptor,
      connection: None,
    }
  }

  pub fn descriptor(&self) -> &D {
    &self.descriptor
  }

  pub fn connection(&self) -> Option<&C> {
    self.connection.as_ref()
  }
}

impl<D: HasId, C> HasId for InputPort<D, C> {
  fn id(&self) -> &str {
    self.descriptor.id()
  }
}

#[derive(Debug)]
pub struct OutputPort<D, C> {
  pub(crate) descriptor: D,
  _phantom: PhantomData<C>,
}

impl<D, C> OutputPort<D, C> {
  pub fn new(descriptor: D) -> Self {
    Self {
      descriptor,
      _phantom: PhantomData,
    }
  }

  pub fn descriptor(&self) -> &D {
    &self.descriptor
  }
}

impl<D: HasId, C> HasId for OutputPort<D, C> {
  fn id(&self) -> &str {
    self.descriptor.id()
  }
}

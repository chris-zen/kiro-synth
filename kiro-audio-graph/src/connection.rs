use crate::audio::{AudioInRef, AudioOutRef};
use crate::key_gen::Key;
use crate::midi::{MidiInRef, MidiOutRef};
use crate::node::NodeRef;
use crate::param::ParamRef;
use crate::port::{AudioInPort, AudioOutPort, MidiInPort, MidiOutPort, ParamPort};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioSignal;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MidiSignal;

#[derive(Debug, PartialEq)]
pub enum Source<S> {
  AudioOut {
    node_ref: NodeRef,
    port_key: Option<Key<AudioOutPort>>,
    signal: S,
  },
  MidiOut {
    node_ref: NodeRef,
    port_key: Option<Key<MidiOutPort>>,
    signal: S,
  },
}

impl<S> Source<S> {
  pub fn node_ref(&self) -> NodeRef {
    match self {
      &Source::AudioOut { node_ref, .. } => node_ref,
      &Source::MidiOut { node_ref, .. } => node_ref,
    }
  }

  pub fn name(&self) -> &str {
    match self {
      &Source::AudioOut { .. } => "AudioOut",
      &Source::MidiOut { .. } => "MidiOut",
    }
  }
}

impl From<NodeRef> for Source<AudioSignal> {
  fn from(node_ref: NodeRef) -> Self {
    Source::AudioOut {
      node_ref,
      port_key: None,
      signal: AudioSignal,
    }
  }
}

impl From<NodeRef> for Source<MidiSignal> {
  fn from(node_ref: NodeRef) -> Self {
    Source::MidiOut {
      node_ref,
      port_key: None,
      signal: MidiSignal,
    }
  }
}

impl From<AudioOutRef> for Source<AudioSignal> {
  fn from(audio_out_ref: AudioOutRef) -> Self {
    Source::AudioOut {
      node_ref: audio_out_ref.node_ref,
      port_key: Some(audio_out_ref.audio_key),
      signal: AudioSignal,
    }
  }
}

impl From<MidiOutRef> for Source<MidiSignal> {
  fn from(midi_out_ref: MidiOutRef) -> Self {
    Source::MidiOut {
      node_ref: midi_out_ref.node_ref,
      port_key: Some(midi_out_ref.midi_key),
      signal: MidiSignal,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Destination<S> {
  AudioIn {
    node_ref: NodeRef,
    port_key: Option<Key<AudioInPort>>,
    signal: S,
  },
  MidiIn {
    node_ref: NodeRef,
    port_key: Option<Key<MidiInPort>>,
    signal: S,
  },
  Param {
    node_ref: NodeRef,
    port_key: Key<ParamPort>,
    signal: S,
  },
}

impl<S> Destination<S> {
  pub fn node_ref(&self) -> NodeRef {
    match self {
      &Destination::AudioIn { node_ref, .. } => node_ref,
      &Destination::MidiIn { node_ref, .. } => node_ref,
      &Destination::Param { node_ref, .. } => node_ref,
    }
  }

  pub fn name(&self) -> &str {
    match self {
      &Destination::AudioIn { .. } => "AudioIn",
      &Destination::MidiIn { .. } => "MidiIn",
      &Destination::Param { .. } => "Param",
    }
  }
}

impl From<NodeRef> for Destination<AudioSignal> {
  fn from(node_ref: NodeRef) -> Self {
    Destination::AudioIn {
      node_ref,
      port_key: None,
      signal: AudioSignal,
    }
  }
}

impl From<NodeRef> for Destination<MidiSignal> {
  fn from(node_ref: NodeRef) -> Self {
    Destination::MidiIn {
      node_ref,
      port_key: None,
      signal: MidiSignal,
    }
  }
}

impl From<AudioInRef> for Destination<AudioSignal> {
  fn from(audio_in_ref: AudioInRef) -> Self {
    Destination::AudioIn {
      node_ref: audio_in_ref.node_ref,
      port_key: Some(audio_in_ref.audio_key),
      signal: AudioSignal,
    }
  }
}

impl From<MidiInRef> for Destination<MidiSignal> {
  fn from(midi_in_ref: MidiInRef) -> Self {
    Destination::MidiIn {
      node_ref: midi_in_ref.node_ref,
      port_key: Some(midi_in_ref.midi_key),
      signal: MidiSignal,
    }
  }
}

impl From<ParamRef> for Destination<AudioSignal> {
  fn from(param_ref: ParamRef) -> Self {
    Destination::Param {
      node_ref: param_ref.node_ref,
      port_key: param_ref.param_key,
      signal: AudioSignal,
    }
  }
}

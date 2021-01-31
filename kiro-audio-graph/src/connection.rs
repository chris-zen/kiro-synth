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
pub enum ConnectionSource<S> {
  DefaultAudioOut {
    node_ref: NodeRef,
    signal: S,
  },
  AudioOut {
    node_ref: NodeRef,
    audio_port_key: Key<AudioOutPort>,
    signal: S,
  },
  DefaultMidiOut {
    node_ref: NodeRef,
    signal: S,
  },
  MidiOut {
    node_ref: NodeRef,
    midi_port_key: Key<MidiOutPort>,
    signal: S,
  },
}

impl<S> ConnectionSource<S> {
  pub fn node_ref(&self) -> NodeRef {
    match *self {
      ConnectionSource::DefaultAudioOut { node_ref, .. } => node_ref,
      ConnectionSource::AudioOut { node_ref, .. } => node_ref,
      ConnectionSource::DefaultMidiOut { node_ref, .. } => node_ref,
      ConnectionSource::MidiOut { node_ref, .. } => node_ref,
    }
  }

  pub fn name(&self) -> &str {
    match *self {
      ConnectionSource::DefaultAudioOut { .. } => "AudioOut",
      ConnectionSource::AudioOut { .. } => "AudioOut",
      ConnectionSource::DefaultMidiOut { .. } => "MidiOut",
      ConnectionSource::MidiOut { .. } => "MidiOut",
    }
  }
}

impl From<NodeRef> for ConnectionSource<AudioSignal> {
  fn from(node_ref: NodeRef) -> Self {
    ConnectionSource::DefaultAudioOut {
      node_ref,
      signal: AudioSignal,
    }
  }
}

impl From<NodeRef> for ConnectionSource<MidiSignal> {
  fn from(node_ref: NodeRef) -> Self {
    ConnectionSource::DefaultMidiOut {
      node_ref,
      signal: MidiSignal,
    }
  }
}

impl From<AudioOutRef> for ConnectionSource<AudioSignal> {
  fn from(audio_out_ref: AudioOutRef) -> Self {
    ConnectionSource::AudioOut {
      node_ref: audio_out_ref.node_ref,
      audio_port_key: audio_out_ref.audio_port_key,
      signal: AudioSignal,
    }
  }
}

impl From<MidiOutRef> for ConnectionSource<MidiSignal> {
  fn from(midi_out_ref: MidiOutRef) -> Self {
    ConnectionSource::MidiOut {
      node_ref: midi_out_ref.node_ref,
      midi_port_key: midi_out_ref.midi_port_key,
      signal: MidiSignal,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum ConnectionDestination<S> {
  Param {
    node_ref: NodeRef,
    param_port_key: Key<ParamPort>,
    signal: S,
  },
  DefaultAudioIn {
    node_ref: NodeRef,
    signal: S,
  },
  AudioIn {
    node_ref: NodeRef,
    audio_port_key: Key<AudioInPort>,
    signal: S,
  },
  DefaultMidiIn {
    node_ref: NodeRef,
    signal: S,
  },
  MidiIn {
    node_ref: NodeRef,
    midi_port_key: Key<MidiInPort>,
    signal: S,
  },
}

impl<S> ConnectionDestination<S> {
  pub fn node_ref(&self) -> NodeRef {
    match *self {
      ConnectionDestination::Param { node_ref, .. } => node_ref,
      ConnectionDestination::DefaultAudioIn { node_ref, .. } => node_ref,
      ConnectionDestination::AudioIn { node_ref, .. } => node_ref,
      ConnectionDestination::DefaultMidiIn { node_ref, .. } => node_ref,
      ConnectionDestination::MidiIn { node_ref, .. } => node_ref,
    }
  }

  pub fn name(&self) -> &str {
    match *self {
      ConnectionDestination::Param { .. } => "Param",
      ConnectionDestination::DefaultAudioIn { .. } => "AudioIn",
      ConnectionDestination::AudioIn { .. } => "AudioIn",
      ConnectionDestination::DefaultMidiIn { .. } => "MidiIn",
      ConnectionDestination::MidiIn { .. } => "MidiIn",
    }
  }
}

impl From<NodeRef> for ConnectionDestination<AudioSignal> {
  fn from(node_ref: NodeRef) -> Self {
    ConnectionDestination::DefaultAudioIn {
      node_ref,
      signal: AudioSignal,
    }
  }
}

impl From<NodeRef> for ConnectionDestination<MidiSignal> {
  fn from(node_ref: NodeRef) -> Self {
    ConnectionDestination::DefaultMidiIn {
      node_ref,
      signal: MidiSignal,
    }
  }
}

impl From<AudioInRef> for ConnectionDestination<AudioSignal> {
  fn from(audio_in_ref: AudioInRef) -> Self {
    ConnectionDestination::AudioIn {
      node_ref: audio_in_ref.node_ref,
      audio_port_key: audio_in_ref.audio_port_key,
      signal: AudioSignal,
    }
  }
}

impl From<MidiInRef> for ConnectionDestination<MidiSignal> {
  fn from(midi_in_ref: MidiInRef) -> Self {
    ConnectionDestination::MidiIn {
      node_ref: midi_in_ref.node_ref,
      midi_port_key: midi_in_ref.midi_port_key,
      signal: MidiSignal,
    }
  }
}

impl From<ParamRef> for ConnectionDestination<AudioSignal> {
  fn from(param_ref: ParamRef) -> Self {
    ConnectionDestination::Param {
      node_ref: param_ref.node_ref,
      param_port_key: param_ref.param_port_key,
      signal: AudioSignal,
    }
  }
}

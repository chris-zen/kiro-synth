use derive_more::Display;

use crate::audio::AudioDescriptor;
use crate::graph::Node;
use crate::key_gen::Key;
use crate::midi::MidiDescriptor;
use crate::param::ParamDescriptor;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Display)]
pub struct NodeRef {
  pub(crate) key: Key<Node>,
}

impl NodeRef {
  pub fn new(key: Key<Node>) -> Self {
    Self { key }
  }

  pub fn ref_string(&self) -> String {
    format!("{}", self.key)
  }
}

impl Into<Key<Node>> for NodeRef {
  fn into(self) -> Key<Node> {
    self.key
  }
}

#[derive(Debug, Clone)]
pub enum DynamicPorts {
  None,
  Limited(usize),
  Unlimited,
}

#[derive(Debug, Clone)]
pub struct NodeDescriptor {
  pub class: String,
  pub static_audio_inputs: Vec<AudioDescriptor>,
  pub dynamic_audio_inputs: DynamicPorts,
  pub static_audio_outputs: Vec<AudioDescriptor>,
  pub dynamic_audio_outputs: DynamicPorts,
  pub static_parameters: Vec<ParamDescriptor>,
  pub dynamic_parameters: DynamicPorts,
  pub static_midi_inputs: Vec<MidiDescriptor>,
  pub static_midi_outputs: Vec<MidiDescriptor>,
}

impl NodeDescriptor {
  pub fn new<S: Into<String>>(class: S) -> Self {
    Self {
      class: class.into(),
      static_audio_inputs: Vec::new(),
      dynamic_audio_inputs: DynamicPorts::None,
      static_audio_outputs: Vec::new(),
      dynamic_audio_outputs: DynamicPorts::None,
      static_parameters: Vec::new(),
      dynamic_parameters: DynamicPorts::None,
      static_midi_inputs: Vec::new(),
      static_midi_outputs: Vec::new(),
    }
  }

  pub fn class(&self) -> &str {
    self.class.as_str()
  }

  pub fn static_audio_inputs(mut self, descriptors: Vec<AudioDescriptor>) -> Self {
    self.static_audio_inputs = descriptors;
    self
  }

  pub fn static_audio_inputs_cardinality(mut self, cardinality: usize, channels: usize) -> Self {
    self.static_audio_inputs = (0..cardinality)
      .into_iter()
      .map(|i| AudioDescriptor::new(format!("input-{}", i), channels))
      .collect();
    self
  }

  pub fn dynamic_audio_inputs(mut self, dynamic_ports: DynamicPorts) -> Self {
    self.dynamic_audio_inputs = dynamic_ports;
    self
  }

  pub fn static_audio_outputs(mut self, descriptors: Vec<AudioDescriptor>) -> Self {
    self.static_audio_outputs = descriptors;
    self
  }

  pub fn static_audio_outputs_cardinality(mut self, cardinality: usize, channels: usize) -> Self {
    self.static_audio_outputs = (0..cardinality)
      .into_iter()
      .map(|i| AudioDescriptor::new(format!("output-{}", i), channels))
      .collect();
    self
  }

  pub fn dynamic_audio_outputs(mut self, dynamic_ports: DynamicPorts) -> Self {
    self.dynamic_audio_outputs = dynamic_ports;
    self
  }

  pub fn static_parameters(mut self, params: Vec<ParamDescriptor>) -> Self {
    self.static_parameters = params;
    self
  }

  pub fn dynamic_parameters(mut self, dynamic_ports: DynamicPorts) -> Self {
    self.dynamic_parameters = dynamic_ports;
    self
  }

  pub fn static_midi_inputs(mut self, descriptors: Vec<MidiDescriptor>) -> Self {
    self.static_midi_inputs = descriptors;
    self
  }

  pub fn static_midi_outputs(mut self, descriptors: Vec<MidiDescriptor>) -> Self {
    self.static_midi_outputs = descriptors;
    self
  }

  // pub(crate) fn valid_audio_input_index(&self, index: usize) -> bool {
  //   match &self.dynamic_audio_inputs {
  //     &DynamicPorts::None => index < self.static_audio_inputs.len(),
  //     &DynamicPorts::Limited(dlen) => index < self.static_audio_inputs.len() + dlen,
  //     &DynamicPorts::Unlimited => true,
  //   }
  // }
  //
  // pub(crate) fn valid_audio_output_index(&self, index: usize) -> bool {
  //   match &self.dynamic_audio_outputs {
  //     &DynamicPorts::None => index < self.static_audio_outputs.len(),
  //     &DynamicPorts::Limited(dlen) => index < self.static_audio_outputs.len() + dlen,
  //     &DynamicPorts::Unlimited => true,
  //   }
  // }
  //
  // pub(crate) fn valid_parameter_index(&self, index: usize) -> bool {
  //   match &self.dynamic_parameters {
  //     &DynamicPorts::None => index < self.static_audio_outputs.len(),
  //     &DynamicPorts::Limited(dlen) => index < self.static_audio_outputs.len() + dlen,
  //     &DynamicPorts::Unlimited => true,
  //   }
  // }
}

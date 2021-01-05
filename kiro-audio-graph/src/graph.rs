use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use thiserror::Error;

use crate::audio::{AudioDescriptor, AudioInRef, AudioOutRef};
use crate::connection::{AudioSignal, Destination, MidiSignal, Source};
use crate::key_store::KeyStoreWithId;
use crate::key_gen::{KeyGen, Key};
use crate::key_store::HasId;
use crate::midi::{MidiDescriptor, MidiInRef, MidiOutRef};
use crate::node::{NodeDescriptor, NodeRef};
use crate::param::{ParamDescriptor, ParamRef};
use crate::port::{MidiOutPortStore, AudioInPortStore, InputPort, AudioOutPortStore, OutputPort, ParamPortStore, MidiInPortStore, AudioOutPort, MidiOutPort, AudioInPort, MidiInPort, ParamPort};

#[derive(Error, Debug)]
pub enum GraphError {
  #[error("Node '{0}' already exists")]
  NodeAlreadyExists(String),

  #[error("Node '{0}' not found")]
  NodeNotFound(String),

  #[error("Invalid Node Reference: {0}")]
  InvalidNodeRef(NodeRef),

  #[error("Param '{1}' already exists for Node {0}")]
  ParamExists(NodeRef, String),

  #[error("Param '{1}' not found for Node {0}")]
  ParamNotFound(NodeRef, String),

  #[error("Audio port '{1}' not found for Node {0}")]
  AudioPortNotFound(NodeRef, String),

  #[error("MIDI port '{1}' not found for Node {0}")]
  MidiPortNotFound(NodeRef, String),

  #[error("Invalid Source Node: {0}::{1}")]
  InvalidSourceNode(String, String),

  #[error("Invalid Audio Source Key: {0}::{1}[{2}]")]
  InvalidAudioSourceKey(String, String, Key<AudioOutPort>),

  #[error("Invalid MIDI Source Key: {0}::{1}[{2}]")]
  InvalidMidiSourceKey(String, String, Key<MidiOutPort>),

  #[error("No Source Default Port: {0}::{1}")]
  NoSourceDefaultPort(String, String),

  #[error("Invalid Destination Node: {0}::{1}")]
  InvalidDestinationNode(String, String),

  #[error("Invalid Param Destination Key: {0}::{1}[{2}]")]
  InvalidParamDestinationKey(String, String, Key<ParamPort>),

  #[error("Invalid Audio Destination Key: {0}::{1}[{2}]")]
  InvalidAudioDestinationKey(String, String, Key<AudioInPort>),

  #[error("Invalid MIDI Destination Key: {0}::{1}[{2}]")]
  InvalidMidiDestinationKey(String, String, Key<MidiInPort>),

  #[error("No Destination Default Port: {0}::{1}")]
  NoDestinationDefaultPort(String, String),

  #[error("Destination already connected: {0}::{1}[{2}]")]
  DestinationAlreadyConnected(String, String, String),
}

pub type Result<T> = std::result::Result<T, GraphError>;

pub struct Node {
  id: String,
  invalidated: bool,
  descriptor: NodeDescriptor,

  audio_inputs: AudioInPortStore,
  audio_outputs: AudioOutPortStore,
  params: ParamPortStore,
  midi_inputs: MidiInPortStore,
  midi_outputs: MidiOutPortStore,

  sources: HashSet<Key<Node>>,
  // destinations: HashSet<Key>,
}

impl Node {
  pub fn new(id: String, descriptor: NodeDescriptor) -> Self {
    let audio_inputs = AudioInPortStore::from(
      descriptor
        .static_audio_inputs
        .iter()
        .map(|descriptor| InputPort::new(descriptor.clone())),
    );
    let audio_outputs = AudioOutPortStore::from(
      descriptor
        .static_audio_outputs
        .iter()
        .map(|descriptor| OutputPort::new(descriptor.clone())),
    );
    let params = ParamPortStore::from(
      descriptor
        .static_parameters
        .iter()
        .map(|descriptor| InputPort::new(descriptor.clone())),
    );
    let midi_inputs = MidiInPortStore::from(
      descriptor
        .static_midi_inputs
        .iter()
        .map(|descriptor| InputPort::new(descriptor.clone())),
    );
    let midi_outputs = MidiOutPortStore::from(
      descriptor
        .static_midi_outputs
        .iter()
        .map(|descriptor| OutputPort::new(descriptor.clone())),
    );

    Self {
      id,
      invalidated: true,
      descriptor,
      audio_inputs,
      audio_outputs,
      params,
      midi_inputs,
      midi_outputs,
      sources: HashSet::new(),
      // destinations: HashSet::new(),
    }
  }

  pub fn ref_string(&self) -> String {
    format!("Node[{}]", self.id)
  }

  pub fn invalidated(&self) -> bool {
    self.invalidated
  }

  pub fn descriptor(&self) -> &NodeDescriptor {
    &self.descriptor
  }

  pub fn audio_inputs(&self) -> &AudioInPortStore {
    &self.audio_inputs
  }

  pub fn audio_outputs(&self) -> &AudioOutPortStore {
    &self.audio_outputs
  }

  pub fn params(&self) -> &ParamPortStore {
    &self.params
  }

  pub fn midi_inputs(&self) -> &MidiInPortStore {
    &self.midi_inputs
  }

  pub fn midi_outputs(&self) -> &MidiOutPortStore {
    &self.midi_outputs
  }

  pub fn sources<'a>(&'a self) -> impl Iterator<Item = NodeRef> + 'a {
    self.sources.iter().cloned().map(NodeRef::new)
  }
}

impl HasId for Node {
  fn id(&self) -> &str {
    self.id.as_str()
  }
}

#[derive(Debug, Clone)]
pub struct GraphTopology {
  pub nodes: Vec<NodeRef>,
  pub counts: Vec<usize>,
}

impl GraphTopology {
  pub fn new(capacity: usize) -> Self {
    Self {
      nodes: Vec::with_capacity(capacity),
      counts: Vec::with_capacity(capacity),
    }
  }
}

pub struct Graph {
  nodes: KeyStoreWithId<Node>,
}

impl Graph {
  pub fn new() -> Self {
    Self {
      nodes: KeyStoreWithId::new(),
    }
  }

  pub fn add_node<'a, S: Into<&'a str>>(
    &mut self,
    id: S,
    descriptor: NodeDescriptor,
  ) -> Result<NodeRef> {
    let id = id.into();
    if self.nodes.contains_id(id) {
      Err(GraphError::NodeAlreadyExists(id.to_string()))
    } else {
      let key = self.nodes.add(Node::new(id.to_string(), descriptor));
      Ok(NodeRef::new(key))
    }
  }

  pub fn get_node_ref<'a, S: Into<&'a str>>(&self, node_id: S) -> Result<NodeRef> {
    let node_id = node_id.into();
    self
      .nodes
      .key_from_id(node_id)
      .map(NodeRef::new)
      .ok_or_else(|| GraphError::NodeNotFound(node_id.to_string()))
  }

  pub fn get_node<N: Into<NodeRef>>(&self, node_ref: N) -> Result<&Node> {
    let node_ref = node_ref.into();
    self
      .nodes
      .get(node_ref.key)
      .ok_or(GraphError::InvalidNodeRef(node_ref))
  }

  fn get_node_mut<N: Into<NodeRef>>(&mut self, node_ref: N) -> Result<&mut Node> {
    let node_ref = node_ref.into();
    self
      .nodes
      .get_mut(node_ref.key)
      .ok_or(GraphError::InvalidNodeRef(node_ref))
  }

  pub fn param<'a, S: Into<&'a str>>(&self, node_ref: NodeRef, param_id: S) -> Result<ParamRef> {
    self.get_node(node_ref).and_then(|node| {
      let param_id = param_id.into();
      node
        .params
        .key_from_id(param_id)
        .map(|param_key| ParamRef::new(node_ref, param_key))
        .ok_or_else(|| GraphError::ParamNotFound(node_ref, param_id.to_string()))
    })
  }

  pub fn audio_input<'a, S: Into<&'a str>>(
    &self,
    node_ref: NodeRef,
    port_id: S,
  ) -> Result<AudioInRef> {
    self.get_node(node_ref).and_then(|node| {
      let id = port_id.into();
      node
        .audio_inputs
        .key_from_id(id)
        .map(|key| AudioInRef::new(node_ref, key))
        .ok_or_else(|| GraphError::AudioPortNotFound(node_ref, id.to_string()))
    })
  }

  pub fn audio_output<'a, S: Into<&'a str>>(
    &self,
    node_ref: NodeRef,
    port_id: S,
  ) -> Result<AudioOutRef> {
    self.get_node(node_ref).and_then(|node| {
      let id = port_id.into();
      node
        .audio_outputs
        .key_from_id(id)
        .map(|key| AudioOutRef::new(node_ref, key))
        .ok_or_else(|| GraphError::AudioPortNotFound(node_ref, id.to_string()))
    })
  }

  pub fn midi_input<'a, S: Into<&'a str>>(
    &self,
    node_ref: NodeRef,
    port_id: S,
  ) -> Result<MidiInRef> {
    self.get_node(node_ref).and_then(|node| {
      let id = port_id.into();
      node
        .midi_inputs
        .key_from_id(id)
        .map(|key| MidiInRef::new(node_ref, key))
        .ok_or_else(|| GraphError::MidiPortNotFound(node_ref, id.to_string()))
    })
  }

  pub fn midi_output<'a, S: Into<&'a str>>(
    &self,
    node_ref: NodeRef,
    port_id: S,
  ) -> Result<MidiOutRef> {
    self.get_node(node_ref).and_then(|node| {
      let id = port_id.into();
      node
        .midi_outputs
        .key_from_id(id)
        .map(|key| MidiOutRef::new(node_ref, key))
        .ok_or_else(|| GraphError::MidiPortNotFound(node_ref, id.to_string()))
    })
  }

  pub fn connect<S, D, G>(&mut self, source: S, destination: D) -> Result<()>
  where
    S: Into<Source<G>>,
    D: Into<Destination<G>>,
    G: Copy,
  {
    let source = self.ensure_valid_source(source.into())?;
    let destination = self.ensure_valid_destination(destination.into())?;

    let node = self.get_node_mut(destination.node_ref())?;

    node.invalidated = true;
    node.sources.insert(source.node_ref().key);

    match destination {
      Destination::Param {
        key: destination_key,
        ..
      } => match source {
        Source::AudioOut {
          node_ref,
          key: source_key,
          ..
        } => {
          let port = node.params.get_mut(destination_key).unwrap();
          port.connection = Some(Source::AudioOut {
            node_ref,
            key: source_key,
            signal: AudioSignal,
          })
        }
        _ => unreachable!(),
      },
      Destination::AudioIn {
        key: destination_key,
        ..
      } => match source {
        Source::AudioOut {
          node_ref,
          key: source_key,
          ..
        } => {
          let port = node.audio_inputs.get_mut(destination_key.unwrap()).unwrap();
          port.connection = Some(Source::AudioOut {
            node_ref,
            key: source_key,
            signal: AudioSignal,
          })
        }
        _ => unreachable!(),
      },
      Destination::MidiIn {
        key: destination_key,
        ..
      } => match source {
        Source::MidiOut {
          node_ref,
          key: source_key,
          ..
        } => {
          let port = node.midi_inputs.get_mut(destination_key.unwrap()).unwrap();
          port.connection = Some(Source::MidiOut {
            node_ref,
            key: source_key,
            signal: MidiSignal,
          })
        }
        _ => unreachable!(),
      },
    }

    Ok(())
  }

  pub fn connect_audio<S, D>(&mut self, source: S, destination: D) -> Result<()>
  where
    S: Into<Source<AudioSignal>>,
    D: Into<Destination<AudioSignal>>,
  {
    self.connect(source, destination)
  }

  pub fn connect_midi<S, D>(&mut self, source: S, destination: D) -> Result<()>
  where
    S: Into<Source<MidiSignal>>,
    D: Into<Destination<MidiSignal>>,
  {
    self.connect(source, destination)
  }

  fn ensure_valid_source<G>(&self, source: Source<G>) -> Result<Source<G>>
  where
    G: Copy,
  {
    let node = self.get_node(source.node_ref()).map_err(|_| {
      GraphError::InvalidSourceNode(source.node_ref().ref_string(), source.name().to_string())
    })?;

    match source {
      Source::AudioOut {
        node_ref,
        key,
        signal,
      } => {
        let key = key
          .or_else(|| node.audio_outputs.first_key())
          .ok_or_else(|| {
            GraphError::NoSourceDefaultPort(node.ref_string(), source.name().to_string())
          })?;

        if node.audio_outputs.contains_key(key) {
          Ok(Source::AudioOut {
            node_ref,
            key: Some(key),
            signal,
          })
        } else {
          Err(GraphError::InvalidAudioSourceKey(
            node.ref_string(),
            source.name().to_string(),
            key,
          ))
        }
      }
      Source::MidiOut {
        node_ref,
        key,
        signal,
      } => {
        let key = key
          .or_else(|| node.midi_outputs.first_key())
          .ok_or_else(|| {
            GraphError::NoSourceDefaultPort(node.ref_string(), source.name().to_string())
          })?;

        if node.midi_outputs.contains_key(key) {
          Ok(Source::MidiOut {
            node_ref,
            key: Some(key),
            signal,
          })
        } else {
          Err(GraphError::InvalidMidiSourceKey(
            node.ref_string(),
            source.name().to_string(),
            key,
          ))
        }
      }
    }
  }

  fn ensure_valid_destination<G>(&self, destination: Destination<G>) -> Result<Destination<G>>
  where
    G: Copy,
  {
    let node = self.get_node(destination.node_ref()).map_err(|_| {
      GraphError::InvalidDestinationNode(
        destination.node_ref().ref_string(),
        destination.name().to_string(),
      )
    })?;

    match destination {
      Destination::Param {
        node_ref,
        key,
        signal,
      } => match node.params.get(key) {
        Some(port) if port.connection.is_none() => Ok(Destination::Param {
          node_ref,
          key,
          signal,
        }),
        Some(port) => Err(GraphError::DestinationAlreadyConnected(
          node.ref_string(),
          destination.name().to_string(),
          port.id().to_string(),
        )),
        None => Err(GraphError::InvalidParamDestinationKey(
          node.ref_string(),
          destination.name().to_string(),
          key,
        )),
      },
      Destination::AudioIn {
        node_ref,
        key,
        signal,
      } => {
        let key = key
          .or_else(|| node.audio_inputs.first_key())
          .ok_or_else(|| {
            GraphError::NoDestinationDefaultPort(node.ref_string(), destination.name().to_string())
          })?;

        match node.audio_inputs.get(key) {
          Some(port) if port.connection.is_none() => Ok(Destination::AudioIn {
            node_ref,
            key: Some(key),
            signal,
          }),
          Some(port) => Err(GraphError::DestinationAlreadyConnected(
            node.ref_string(),
            destination.name().to_string(),
            port.id().to_string(),
          )),
          None => Err(GraphError::InvalidAudioDestinationKey(
            node.ref_string(),
            destination.name().to_string(),
            key,
          )),
        }
      }
      Destination::MidiIn {
        node_ref,
        key,
        signal,
      } => {
        let key = key
          .or_else(|| node.midi_inputs.first_key())
          .ok_or_else(|| {
            GraphError::NoDestinationDefaultPort(node.ref_string(), destination.name().to_string())
          })?;

        match node.midi_inputs.get(key) {
          Some(port) if port.connection.is_none() => Ok(Destination::MidiIn {
            node_ref,
            key: Some(key),
            signal,
          }),
          Some(port) => Err(GraphError::DestinationAlreadyConnected(
            node.ref_string(),
            destination.name().to_string(),
            port.id().to_string(),
          )),
          None => Err(GraphError::InvalidMidiDestinationKey(
            node.ref_string(),
            destination.name().to_string(),
            key,
          )),
        }
      }
    }
  }

  pub fn topology(&self) -> GraphTopology {
    enum DfsState {
      Unseen,
      Traversing,
      Visited,
    }

    let mut topology = GraphTopology::new(self.nodes.len());

    let mut dfs_state = self
      .nodes
      .keys()
      .map(|key| (*key, DfsState::Unseen))
      .collect::<HashMap<Key<Node>, DfsState>>();

    let mut counts = self
      .nodes
      .keys()
      .map(|key| (*key, 0))
      .collect::<HashMap<Key<Node>, usize>>();

    let mut stack = self.nodes.keys().cloned().collect::<Vec<Key<Node>>>();

    while let Some(key) = stack.pop() {
      let node_state = dfs_state.get_mut(&key).unwrap();
      match node_state {
        DfsState::Unseen => {
          *node_state = DfsState::Traversing;
          stack.push(key);
          let node = self.nodes.get(key).unwrap();
          *counts.get_mut(&key).unwrap() += node.sources.len();
          for adj_key in node.sources.iter() {
            stack.push(*adj_key);
          }
        }
        DfsState::Traversing => {
          *node_state = DfsState::Visited;
          topology.nodes.push(NodeRef::new(key));
        }
        DfsState::Visited => {}
      }
    }

    topology.counts = topology
      .nodes
      .iter()
      .map(|node_ref| *counts.get(&node_ref.key).unwrap())
      .collect();

    topology
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn create_graph_for_connections() -> Result<(Graph, NodeRef, NodeRef)> {
    let mut g = Graph::new();

    let source_desc = NodeDescriptor::new("Source")
      .static_audio_outputs(vec![AudioDescriptor::new("OUT", 1)])
      .static_midi_outputs(vec![MidiDescriptor::new("OUT")]);
    let sink_desc = NodeDescriptor::new("Sink")
      .static_audio_inputs(vec![AudioDescriptor::new("IN", 1)])
      .static_parameters(vec![ParamDescriptor::new("P1")])
      .static_midi_inputs(vec![MidiDescriptor::new("IN")]);

    let n1 = g.add_node("N1", source_desc.clone())?;
    let n2 = g.add_node("N2", sink_desc.clone())?;

    Ok((g, n1, n2))
  }

  #[test]
  fn connect_audio_node_with_node() -> Result<()> {
    let (mut g, n1, n2) = create_graph_for_connections()?;

    g.connect_audio(n1, n2)?;
    let port = g.nodes.get(n2.key).unwrap().audio_inputs.get(0).unwrap();
    assert_eq!(
      port.connection,
      Some(Source::AudioOut {
        node_ref: n1,
        key: Some(0),
        signal: AudioSignal
      })
    );

    Ok(())
  }

  #[test]
  fn connect_node_with_audio_input() -> Result<()> {
    let (mut g, n1, n2) = create_graph_for_connections()?;

    let destination = g.audio_input(n2, "IN")?;
    g.connect(n1, destination)?;
    let port = g
      .nodes
      .get(destination.node_ref.key)
      .unwrap()
      .audio_inputs
      .get(destination.audio_key)
      .unwrap();
    assert_eq!(
      port.connection,
      Some(Source::AudioOut {
        node_ref: n1,
        key: Some(0),
        signal: AudioSignal
      })
    );

    Ok(())
  }

  #[test]
  fn connect_audio_output_with_audio_input() -> Result<()> {
    let (mut g, n1, n2) = create_graph_for_connections()?;

    let source = g.audio_output(n1, "OUT")?;
    let destination = g.audio_input(n2, "IN")?;
    g.connect(source, destination)?;
    let port = g
      .nodes
      .get(destination.node_ref.key)
      .unwrap()
      .audio_inputs
      .get(destination.audio_key)
      .unwrap();
    assert_eq!(
      port.connection,
      Some(Source::AudioOut {
        node_ref: source.node_ref,
        key: Some(source.audio_key),
        signal: AudioSignal
      })
    );

    Ok(())
  }

  #[test]
  fn connect_node_with_param() -> Result<()> {
    let (mut g, n1, n2) = create_graph_for_connections()?;

    let destination = g.param(n2, "P1")?;
    g.connect(n1, destination)?;
    let port = g
      .nodes
      .get(destination.node_ref.key)
      .unwrap()
      .params
      .get(destination.param_key)
      .unwrap();
    assert_eq!(
      port.connection,
      Some(Source::AudioOut {
        node_ref: n1,
        key: Some(0),
        signal: AudioSignal
      })
    );

    Ok(())
  }

  #[test]
  fn connect_audio_output_with_param() -> Result<()> {
    let (mut g, n1, n2) = create_graph_for_connections()?;

    let source = g.audio_output(n1, "OUT")?;
    let destination = g.param(n2, "P1")?;
    g.connect(source, destination)?;
    let port = g
      .nodes
      .get(destination.node_ref.key)
      .unwrap()
      .params
      .get(destination.param_key)
      .unwrap();
    assert_eq!(
      port.connection,
      Some(Source::AudioOut {
        node_ref: source.node_ref,
        key: Some(source.audio_key),
        signal: AudioSignal
      })
    );

    Ok(())
  }

  #[test]
  fn connect_midi_node_with_node() -> Result<()> {
    let (mut g, n1, n2) = create_graph_for_connections()?;

    g.connect_midi(n1, n2)?;
    let port = g.nodes.get(n2.key).unwrap().midi_inputs.get(0).unwrap();
    assert_eq!(
      port.connection,
      Some(Source::MidiOut {
        node_ref: n1,
        key: Some(0),
        signal: MidiSignal
      })
    );

    Ok(())
  }

  #[test]
  fn connect_node_with_midi_input() -> Result<()> {
    let (mut g, n1, n2) = create_graph_for_connections()?;

    let destination = g.midi_input(n2, "IN")?;
    g.connect(n1, destination)?;
    let port = g
      .nodes
      .get(destination.node_ref.key)
      .unwrap()
      .midi_inputs
      .get(destination.midi_key)
      .unwrap();
    assert_eq!(
      port.connection,
      Some(Source::MidiOut {
        node_ref: n1,
        key: Some(0),
        signal: MidiSignal
      })
    );

    Ok(())
  }

  #[test]
  fn connect_midi_output_with_midi_input() -> Result<()> {
    let (mut g, n1, n2) = create_graph_for_connections()?;

    let source = g.midi_output(n1, "OUT")?;
    let destination = g.midi_input(n2, "IN")?;
    g.connect(source, destination)?;
    let port = g
      .nodes
      .get(destination.node_ref.key)
      .unwrap()
      .midi_inputs
      .get(destination.midi_key)
      .unwrap();
    assert_eq!(
      port.connection,
      Some(Source::MidiOut {
        node_ref: source.node_ref,
        key: Some(source.midi_key),
        signal: MidiSignal
      })
    );

    // g.connect(n1_midi_out, n2)?;

    Ok(())
  }

  #[test]
  fn connect_midi_output_with_node() -> Result<()> {
    let (mut g, n1, n2) = create_graph_for_connections()?;

    let source = g.midi_output(n1, "OUT")?;
    g.connect(source, n2)?;
    let port = g.nodes.get(n2.key).unwrap().midi_inputs.get(0).unwrap();
    assert_eq!(
      port.connection,
      Some(Source::MidiOut {
        node_ref: n1,
        key: Some(0),
        signal: MidiSignal
      })
    );

    Ok(())
  }

  #[test]
  fn topology() -> Result<()> {
    let mut g = Graph::new();

    let sink_desc = NodeDescriptor::new("Sink").static_audio_inputs(vec![
      AudioDescriptor::new("in1", 1),
      AudioDescriptor::new("in2", 1),
    ]);
    let source_desc =
      NodeDescriptor::new("Source").static_audio_outputs(vec![AudioDescriptor::new("out", 1)]);
    let proc_desc = NodeDescriptor::new("Proc")
      .static_audio_inputs(vec![
        AudioDescriptor::new("in1", 1),
        AudioDescriptor::new("in2", 1),
      ])
      .static_audio_outputs(vec![AudioDescriptor::new("out", 1)]);

    let a = g.add_node("A", sink_desc.clone())?;
    let b = g.add_node("B", proc_desc.clone())?;
    let c = g.add_node("C", proc_desc.clone())?;
    let d = g.add_node("D", source_desc.clone())?;
    let e = g.add_node("E", source_desc.clone())?;
    let f = g.add_node("F", source_desc.clone())?;

    g.connect(d, g.audio_input(b, "in1")?)?;
    g.connect(e, g.audio_input(b, "in2")?)?;
    g.connect(b, g.audio_input(a, "in1")?)?;
    g.connect_audio(f, c)?;
    g.connect(c, g.audio_input(a, "in2")?)?;

    let topology = g.topology();

    // Check the topology order
    // For each edge u -> w, the index of u in the topology order should be less than the one for w

    let topo_index = topology
      .nodes
      .iter()
      .enumerate()
      .map(|(index, node_ref)| (*node_ref, index))
      .collect::<HashMap<NodeRef, usize>>();

    let edges = vec![(d, b), (e, b), (b, a), (f, c), (c, a)];

    for (from, to) in edges.iter() {
      assert!(topo_index.get(from).unwrap() < topo_index.get(to).unwrap());
    }

    // Check the counts for incoming edges per node

    let counts = topology
      .nodes
      .iter()
      .zip(topology.counts.iter())
      .map(|(node_ref, count)| (*node_ref, *count))
      .collect::<HashMap<NodeRef, usize>>();

    println!("{:?}", topology);

    let expected_counts = vec![(a, 2usize), (b, 2), (c, 1), (d, 0), (e, 0), (f, 0)];
    for (node_ref, count) in expected_counts.iter() {
      println!("{} -> {}", node_ref.key, count);
      assert_eq!(counts.get(node_ref).cloned(), Some(*count))
    }

    Ok(())
  }
}

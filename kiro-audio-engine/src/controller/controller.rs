use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use ringbuf::{Consumer, Producer};
use thiserror::Error;

use kiro_audio_graph::connection::AudioSignal;
use kiro_audio_graph::key_store::KeyStore;
use kiro_audio_graph::port::{AudioInPort, AudioOutPort, ParamPort};
use kiro_audio_graph::{Graph, NodeRef, Source};
use kiro_audio_graph::{GraphTopology, Key, Node};

use crate::buffers::Buffer;
use crate::controller::owned_data::{OwnedData, Ref};
use crate::controller::ProcParams;
use crate::messages::Message;
use crate::processor::context::ProcessorContext;
use crate::processor::ports::audio::AudioRenderPort;
use crate::processor::ports::param::ParamRenderPort;
use crate::processor::ports::{Input, Output};
use crate::processor::{ProcessorBox, ProcessorFactory};
use crate::renderer::plan::{RenderOp, RenderPlan};
use crate::{EngineConfig, ParamValue};

#[derive(Error, Debug, PartialEq)]
pub enum ControllerError {
  #[error("Processor not found: {0:?}")]
  ProcessorNotFound(Key<ProcessorBox>),

  #[error("Parameters not found: {0:?}")]
  ParametersNotFound(Key<ProcParams>),

  #[error("Buffer not found: {0:?}")]
  BufferNotFound(Key<Buffer>),

  #[error("Node cache not found: {0:?}")]
  NodeCacheNotFound(NodeRef),

  #[error("Node not found: {0:?}")]
  NodeNotFound(NodeRef),

  #[error("Failed to send data to the renderer")]
  SendFailure,

  #[error("Processor factory not found for {0} with class {1}")]
  ProcessorFactoryNotFound(String, String),

  #[error("Failed to create a Processor for {0} with class {1}")]
  ProcessorCreationFailed(String, String),

  #[error("Parameter key not found in the node cache for port {0:?}")]
  ParamKeyNotFound(Key<ParamPort>),

  #[error("Parameter value with key {0:?} not found")]
  ParamValueNotFound(Key<Arc<ParamValue>>),

  #[error("Parameter slice buffer not found for port {0:?}")]
  SliceBufferNotFound(Key<ParamPort>),

  #[error("Audio output buffer not found in the node cache for port {0:?}")]
  AudioOutBufferNotFound(Key<AudioOutPort>),
}

// TODO figure out how to remove Sync for ControllerError
unsafe impl Sync for ControllerError {}

pub type Result<T> = core::result::Result<T, ControllerError>;

struct NodeCache {
  processor_key: Key<ProcessorBox>,
  parameter_keys: HashMap<Key<ParamPort>, Key<Arc<ParamValue>>>,
  audio_output_buffers: HashMap<Key<AudioOutPort>, Vec<Ref<Buffer>>>,
  allocated_buffers: HashSet<Key<Buffer>>,
  render_ops: Vec<RenderOp>,
}

impl NodeCache {
  pub fn new(
    processor_key: Key<ProcessorBox>,
    parameter_keys: HashMap<Key<ParamPort>, Key<Arc<ParamValue>>>,
  ) -> Self {
    Self {
      processor_key,
      parameter_keys,
      audio_output_buffers: HashMap::new(),
      allocated_buffers: HashSet::new(),
      render_ops: Vec::new(),
    }
  }

  pub fn get_param_key(&self, port_key: Key<ParamPort>) -> Result<Key<Arc<ParamValue>>> {
    self
      .parameter_keys
      .get(&port_key)
      .cloned()
      .ok_or(ControllerError::ParamKeyNotFound(port_key))
  }

  pub fn get_audio_output_buffer(&self, port_key: Key<AudioOutPort>) -> Result<&Vec<Ref<Buffer>>> {
    self
      .audio_output_buffers
      .get(&port_key)
      .ok_or(ControllerError::AudioOutBufferNotFound(port_key))
  }
}

struct UpdateContext {
  destination_counts: HashMap<NodeRef, usize>,
  free_buffers: HashSet<Key<Buffer>>,
}

impl UpdateContext {
  pub fn new(topology: &GraphTopology, free_buffers: impl Iterator<Item = Key<Buffer>>) -> Self {
    let destination_counts = topology.destination_counts.clone();

    let free_buffers = free_buffers.collect();

    Self {
      destination_counts,
      free_buffers,
    }
  }

  pub fn add_to_free_buffers(&mut self, buffers: &HashSet<Key<Buffer>>) {
    self.free_buffers = self.free_buffers.union(buffers).cloned().collect();
  }

  pub fn remove_from_free_buffers(&mut self, buffers: &HashSet<Key<Buffer>>) {
    self.free_buffers = self.free_buffers.difference(buffers).cloned().collect();
  }
}

pub struct Controller {
  tx: Producer<Message>,
  rx: Consumer<Message>,

  config: EngineConfig,

  parameters: KeyStore<Arc<ParamValue>>,

  processor_factories: HashMap<String, Rc<dyn ProcessorFactory>>,
  processors: OwnedData<ProcessorBox>,

  buffers: OwnedData<Buffer>,
  empty_buffer: Key<Buffer>,

  nodes: HashMap<NodeRef, NodeCache>,
}

impl Controller {
  pub fn new(tx: Producer<Message>, rx: Consumer<Message>, config: EngineConfig) -> Self {
    let mut buffers = OwnedData::new();
    let mut empty_buffer = Buffer::new(config.buffer_size);
    empty_buffer.fill(0.0);
    let empty_buffer = buffers.add(empty_buffer);

    Self {
      tx,
      rx,
      config,
      parameters: KeyStore::new(),
      processor_factories: HashMap::new(),
      processors: OwnedData::new(),
      buffers,
      empty_buffer,
      nodes: HashMap::new(),
    }
  }

  pub fn register_processor_factory<F>(&mut self, factory: F)
  where
    F: ProcessorFactory + 'static,
  {
    let factory = Rc::new(factory);
    for class in factory.supported_classes().iter() {
      self
        .processor_factories
        .insert(class.clone(), factory.clone());
    }
  }

  pub fn update_graph(&mut self, graph: &Graph) -> Result<()> {
    let mut render_plan = RenderPlan::default();
    let topology = graph.topology();
    println!("{:#?}\n{:#?}", graph, topology);

    let buffers = self
      .buffers
      .keys()
      .filter(|buffer_key| *buffer_key != self.empty_buffer);

    let mut update_context = UpdateContext::new(&topology, buffers);

    self.update_nodes(topology.nodes.as_slice(), graph, &mut update_context)?;

    for node_ref in topology.nodes {
      let node_cache = self.get_node_cache(node_ref)?;

      render_plan
        .operations
        .extend(node_cache.render_ops.iter().cloned());
    }
    self
      .tx
      .push(Message::MoveRenderPlan(Box::new(render_plan)))
      .map_err(|_| ControllerError::SendFailure)
  }

  fn update_nodes(
    &mut self,
    node_refs: &[NodeRef],
    graph: &Graph,
    context: &mut UpdateContext,
  ) -> Result<()> {
    for node_ref in node_refs {
      let node_cache_create = !self.nodes.contains_key(node_ref);
      if node_cache_create {
        let node_cache = self.create_node(*node_ref, graph)?;
        self.nodes.insert(*node_ref, node_cache);
      }

      let node = graph
        .get_node(*node_ref)
        .map_err(|_| ControllerError::NodeNotFound(*node_ref))?;

      if node.invalidated() || node_cache_create {
        self.visit_invalidated_node(*node_ref, graph, context)?;
      } else {
        self.visit_unchanged_node(*node_ref, graph, context)?;
      }
    }

    // TODO free node cache that has been removed from the graph

    Ok(())
  }

  fn create_node(&mut self, node_ref: NodeRef, graph: &Graph) -> Result<NodeCache> {
    let node = graph
      .get_node(node_ref)
      .map_err(|_| ControllerError::NodeNotFound(node_ref))?;

    let node_descriptor = node.descriptor();
    let node_class = node_descriptor.class();
    let factory = self.processor_factories.get(node_class).ok_or_else(|| {
      ControllerError::ProcessorFactoryNotFound(node.ref_string(), node_class.to_string())
    })?;
    let processor = factory.deref().create(node_descriptor).ok_or_else(|| {
      ControllerError::ProcessorCreationFailed(node.ref_string(), node_class.to_string())
    })?;
    let processor_key = self.processors.add(processor);

    let parameter_keys = node
      .params()
      .iter()
      .map(|(port_key, port)| {
        let initial_value = port.descriptor().initial;
        let param_value = Arc::new(ParamValue::new(initial_value));
        let param_key = self.parameters.add(param_value);
        (port_key, param_key)
      })
      .collect::<HashMap<Key<ParamPort>, Key<Arc<ParamValue>>>>();

    Ok(NodeCache::new(processor_key, parameter_keys))
  }

  /// Visit a node that has been invalidated and requires to regenerate the cache
  fn visit_invalidated_node(
    &mut self,
    node_ref: NodeRef,
    graph: &Graph,
    context: &mut UpdateContext,
  ) -> Result<()> {
    self.clear_node_cache(node_ref, context)?;

    let node = graph
      .get_node(node_ref)
      .map_err(|_| ControllerError::NodeNotFound(node_ref))?;

    let param_value_buffers = self.allocate_param_value_buffers(node, context);
    let param_render_ports = self.build_param_render_ports(node_ref, node, &param_value_buffers)?;

    let audio_output_buffers = self.allocate_audio_output_buffers(node, context);
    let audio_output_render_ports = self.build_audio_output_render_ports(&audio_output_buffers);
    let audio_input_render_ports = self.build_audio_input_render_ports(node)?;

    self.release_input_buffers(node, context)?;

    self.update_node_cache(
      node_ref,
      param_value_buffers,
      param_render_ports,
      audio_output_buffers,
      audio_output_render_ports,
      audio_input_render_ports,
    )?;

    Ok(())
  }

  fn update_node_cache(
    &mut self,
    node_ref: NodeRef,
    param_value_buffers: HashMap<Key<ParamPort>, Ref<Buffer>>,
    param_render_ports: Vec<ParamRenderPort>,
    audio_output_buffers: HashMap<Key<AudioOutPort>, Vec<Ref<Buffer>>>,
    audio_output_render_ports: Vec<AudioRenderPort<Output>>,
    audio_input_render_ports: Vec<AudioRenderPort<Input>>,
  ) -> Result<()> {
    let node_cache = self
      .nodes
      .get_mut(&node_ref)
      .ok_or(ControllerError::NodeCacheNotFound(node_ref))?;

    let allocated_param_buffers = param_value_buffers
      .iter()
      .map(|(_port_key, buffer)| buffer.key);

    let allocated_audio_buffers = audio_output_buffers
      .iter()
      .flat_map(|(_port_key, buffers)| buffers)
      .map(|buffer_ref| buffer_ref.key);

    node_cache.allocated_buffers = allocated_param_buffers
      .chain(allocated_audio_buffers)
      .collect::<HashSet<Key<Buffer>>>();

    node_cache.audio_output_buffers = audio_output_buffers;

    let processor = self
      .processors
      .get(node_cache.processor_key)
      .ok_or_else(|| ControllerError::ProcessorNotFound(node_cache.processor_key))?;

    let processor_context = ProcessorContext::new(
      audio_input_render_ports,
      audio_output_render_ports,
      param_render_ports,
    );

    node_cache
      .render_ops
      .push(RenderOp::RenderProcessor(processor, processor_context));

    Ok(())
  }

  fn clear_node_cache(&mut self, node_ref: NodeRef, context: &mut UpdateContext) -> Result<()> {
    let node_cache = self.get_node_cache_mut(node_ref)?;

    context.add_to_free_buffers(&node_cache.allocated_buffers);
    node_cache.allocated_buffers.clear();
    node_cache.audio_output_buffers.clear();
    node_cache.render_ops.clear();

    Ok(())
  }

  fn allocate_audio_output_buffers(
    &mut self,
    node: &Node,
    context: &mut UpdateContext,
  ) -> HashMap<Key<AudioOutPort>, Vec<Ref<Buffer>>> {
    node
      .audio_outputs()
      .iter()
      .map(|(port_key, port)| {
        let buffer_keys = (0..port.descriptor().channels())
          .map(|_| self.allocate_buffer(context))
          .collect::<Vec<Key<Buffer>>>();

        let buffers = buffer_keys
          .iter()
          .filter_map(|key| self.buffers.get(*key))
          .collect::<Vec<Ref<Buffer>>>();

        // TODO check that the number of buffers matches the number of channels

        (port_key, buffers)
      })
      .collect()
  }

  fn build_audio_output_render_ports(
    &self,
    buffers: &HashMap<Key<AudioOutPort>, Vec<Ref<Buffer>>>,
  ) -> Vec<AudioRenderPort<Output>> {
    let mut port_keys = buffers.keys().cloned().collect::<Vec<Key<AudioOutPort>>>();
    port_keys.sort();

    port_keys
      .into_iter()
      .map(|port_key| {
        let port_buffers = buffers.get(&port_key).unwrap();
        AudioRenderPort::new(port_buffers.clone())
      })
      .collect()
  }

  fn build_audio_input_render_ports(&mut self, node: &Node) -> Result<Vec<AudioRenderPort<Input>>> {
    let mut port_keys = node
      .audio_inputs()
      .keys()
      .cloned()
      .collect::<Vec<Key<AudioInPort>>>();
    port_keys.sort();

    let mut render_ports = Vec::<AudioRenderPort<Input>>::new();
    for port_key in port_keys {
      let port = node.audio_inputs().get(port_key).unwrap();

      let render_port = match port.connection() {
        None => self.build_empty_audio_input_render_port(port.descriptor().channels()),
        Some(source) => self.build_audio_input_render_port(source),
      }?;

      render_ports.push(render_port);
    }

    Ok(render_ports)
  }

  fn build_empty_audio_input_render_port(&self, channels: usize) -> Result<AudioRenderPort<Input>> {
    let empty_buffer = self.buffers.get(self.empty_buffer).unwrap();
    let buffers = (0..channels)
      .map(|_| empty_buffer.clone())
      .collect::<Vec<Ref<Buffer>>>();
    Ok(AudioRenderPort::new(buffers))
  }

  fn build_audio_input_render_port(
    &self,
    source: &Source<AudioSignal>,
  ) -> Result<AudioRenderPort<Input>> {
    // TODO find a better way to express that several things here should be defined at this point, then the unwraps are safe.
    match source {
      Source::AudioOut {
        node_ref, port_key, ..
      } => {
        let port_key = port_key.unwrap(); // This should have been defined during the connection
        let node_cache = self.get_node_cache(*node_ref)?;
        let buffers = node_cache.get_audio_output_buffer(port_key)?;
        Ok(AudioRenderPort::new(buffers.clone()))
      }
      Source::MidiOut {
        node_ref: _,
        port_key: _,
        ..
      } => {
        unimplemented!()
      }
    }
  }

  fn allocate_param_value_buffers(
    &mut self,
    node: &Node,
    context: &mut UpdateContext,
  ) -> HashMap<Key<ParamPort>, Ref<Buffer>> {
    node
      .params()
      .iter()
      .filter_map(|(port_key, port)| {
        let maybe_buffer = match port.connection() {
          None => {
            let buffer_key = self.allocate_buffer(context);
            Some(self.buffers.get(buffer_key).unwrap())
          }
          Some(_source) => None,
        };
        maybe_buffer.map(|buffer| (port_key, buffer))
      })
      .collect()
  }

  fn build_param_render_ports(
    &self,
    node_ref: NodeRef,
    node: &Node,
    value_buffers: &HashMap<Key<ParamPort>, Ref<Buffer>>,
  ) -> Result<Vec<ParamRenderPort>> {
    let mut render_ports = Vec::<ParamRenderPort>::new();
    for (port_key, port) in node.params().iter() {
      match port.connection() {
        None => {
          let value = self
            .get_node_cache(node_ref)
            .and_then(|node_cache| node_cache.get_param_key(port_key))
            .and_then(|param_key| self.get_param_value(param_key))?;

          let slice_buffer = value_buffers
            .get(&port_key)
            .cloned()
            .ok_or(ControllerError::SliceBufferNotFound(port_key))?;

          render_ports.push(ParamRenderPort::value(value, slice_buffer));
        }
        Some(source) => match source {
          Source::AudioOut {
            node_ref, port_key, ..
          } => {
            let port_key = port_key.unwrap(); // This should have been defined during the connection
            let node_cache = self.get_node_cache(*node_ref)?;
            let buffers = node_cache.get_audio_output_buffer(port_key)?;
            // TODO Users should be able to choose a different channel when connecting the audio output to a parameter
            let buffer = buffers.get(0).unwrap(); // The connection should have tested that there is at least one channel
            render_ports.push(ParamRenderPort::buffer(buffer.clone()));
          }
          Source::MidiOut { .. } => unreachable!(),
        },
      }
    }
    Ok(render_ports)
  }

  /// Visit a node that has not been invalidated
  fn visit_unchanged_node(
    &mut self,
    node_ref: NodeRef,
    graph: &Graph,
    context: &mut UpdateContext,
  ) -> Result<()> {
    let node = graph
      .get_node(node_ref)
      .map_err(|_| ControllerError::NodeNotFound(node_ref))?;

    self.release_input_buffers(node, context)?;

    // Mark output buffers as allocated

    let node_cache = self.get_node_cache(node_ref)?;
    context.remove_from_free_buffers(&node_cache.allocated_buffers);

    Ok(())
  }

  /// Release input buffers that are not used anymore
  fn release_input_buffers(&mut self, node: &Node, context: &mut UpdateContext) -> Result<()> {
    for source_node_ref in node.sources() {
      let count = *context
        .destination_counts
        .entry(source_node_ref)
        // .and_modify(|e| if *e > 0 { *e = *e - 1 })
        .and_modify(|e| *e = *e - 1)
        .or_default();
      if count <= 0 {
        let source_node_cache = self
          .nodes
          .get_mut(&source_node_ref)
          .ok_or(ControllerError::NodeCacheNotFound(source_node_ref))?;

        context.add_to_free_buffers(&source_node_cache.allocated_buffers);
        source_node_cache.allocated_buffers.clear();
      }
    }
    Ok(())
  }

  fn allocate_buffer(&mut self, context: &mut UpdateContext) -> Key<Buffer> {
    let maybe_key = context
      .free_buffers
      .iter()
      .take(1)
      .cloned()
      .collect::<Vec<Key<Buffer>>>()
      .first()
      .cloned();

    match maybe_key {
      Some(key) => {
        context.free_buffers.remove(&key);
        key
      }
      None => self.buffers.add(Buffer::new(self.config.buffer_size)),
    }
  }

  fn get_node_cache(&self, node_ref: NodeRef) -> Result<&NodeCache> {
    self
      .nodes
      .get(&node_ref)
      .ok_or(ControllerError::NodeCacheNotFound(node_ref))
  }

  fn get_node_cache_mut(&mut self, node_ref: NodeRef) -> Result<&mut NodeCache> {
    self
      .nodes
      .get_mut(&node_ref)
      .ok_or(ControllerError::NodeCacheNotFound(node_ref))
  }

  fn get_param_value(&self, param_key: Key<Arc<ParamValue>>) -> Result<Arc<ParamValue>> {
    self
      .parameters
      .get(param_key)
      .cloned()
      .ok_or(ControllerError::ParamValueNotFound(param_key))
  }

  pub fn process_messages(&mut self) {
    self.rx.pop_each(
      move |message| {
        match message {
          Message::MoveRenderPlan(plan) => {
            drop(plan);
          }
        }
        true
      },
      None,
    );
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use kiro_audio_graph::{AudioDescriptor, MidiDescriptor, NodeDescriptor, ParamDescriptor};
  use ringbuf::RingBuffer;

  struct TestProcessor(NodeDescriptor);

  impl Processor for TestProcessor {
    fn render(&mut self, context: &mut ProcessorContext) {
      unimplemented!()
    }
  }

  struct TestProcessorFactory;

  impl ProcessorFactory for TestProcessorFactory {
    fn supported_classes(&self) -> Vec<String> {
      vec!["source-class".to_string(), "sink-class".to_string()]
    }

    fn create(&self, descriptor: &NodeDescriptor) -> Option<Box<dyn Processor>> {
      Some(Box::new(TestProcessor(descriptor.clone())))
    }
  }

  fn create_graph() -> anyhow::Result<(Graph, NodeRef, NodeRef, NodeRef)> {
    let mut g = Graph::new();

    let source_desc = NodeDescriptor::new("source-class")
      .static_audio_outputs(vec![AudioDescriptor::new("OUT", 1)])
      .static_midi_outputs(vec![MidiDescriptor::new("OUT")]);
    let sink_desc = NodeDescriptor::new("sink-class")
      .static_audio_inputs(vec![
        AudioDescriptor::new("IN1", 1),
        AudioDescriptor::new("IN2", 1),
      ])
      .static_parameters(vec![
        ParamDescriptor::new("P1"),
        ParamDescriptor::new("P2"),
        ParamDescriptor::new("P3"),
      ])
      .static_midi_inputs(vec![MidiDescriptor::new("IN")]);

    let n1 = g.add_node("N1", source_desc.clone())?;
    let n2 = g.add_node("N2", source_desc.clone())?;
    let n3 = g.add_node("N3", sink_desc.clone())?;

    g.connect_audio(n1, g.audio_input(n3, "IN1")?)?;
    g.connect_audio(n2, g.audio_input(n3, "IN2")?)?;
    g.connect(n2, g.param(n3, "P1")?)?;

    Ok((g, n1, n2, n3))
  }

  fn create_controller_without_processor_factory() -> anyhow::Result<Controller> {
    let ring_buffer = RingBuffer::new(1);
    let (tx, rx) = ring_buffer.split();
    let config = EngineConfig::default();
    Ok(Controller::new(tx, rx, config))
  }

  fn create_controller() -> anyhow::Result<Controller> {
    let mut controller = create_controller_without_processor_factory()?;
    controller.register_processor_factory(TestProcessorFactory);
    Ok(controller)
  }

  #[test]
  fn update_graph__processor_factory_not_found() -> anyhow::Result<()> {
    let (mut g, _, _, _) = create_graph()?;
    let mut ct = create_controller_without_processor_factory()?;

    assert_eq!(
      ct.update_graph(&g),
      Err(ControllerError::ProcessorFactoryNotFound(
        "Node[N2]".to_string(),
        "source-class".to_string()
      ))
    );

    Ok(())
  }

  #[test]
  fn update_graph__success() -> anyhow::Result<()> {
    let (mut g, n1, n2, n3) = create_graph()?;
    let mut ct = create_controller()?;

    ct.update_graph(&g)?;

    assert_eq!(ct.parameters.len(), 3);
    assert_eq!(ct.processors.len(), 3);
    assert_eq!(ct.buffers.len(), 5); // empty + 2 outputs + 2 param slice

    let nc1 = ct.nodes.get(&n1).unwrap();
    assert_eq!(nc1.parameter_keys.len(), 0);
    assert_eq!(
      nc1.audio_output_buffers.values().cloned().flatten().count(),
      1
    );
    assert_eq!(nc1.allocated_buffers.len(), 0);
    assert_eq!(nc1.render_ops.len(), 1);
    assert!(match nc1.render_ops.get(0).unwrap() {
      RenderOp::RenderProcessor(_, _) => true,
      _ => false,
    });

    let nc2 = ct.nodes.get(&n2).unwrap();
    assert_eq!(nc2.parameter_keys.len(), 0);
    assert_eq!(
      nc2.audio_output_buffers.values().cloned().flatten().count(),
      1
    );
    assert_eq!(nc2.allocated_buffers.len(), 0);
    assert_eq!(nc2.render_ops.len(), 1);
    assert!(match nc2.render_ops.get(0).unwrap() {
      RenderOp::RenderProcessor(_, _) => true,
      _ => false,
    });

    let nc3 = ct.nodes.get(&n3).unwrap();
    assert_eq!(nc3.parameter_keys.len(), 3);
    assert_eq!(
      nc3.audio_output_buffers.values().cloned().flatten().count(),
      0
    );
    assert_eq!(nc3.allocated_buffers.len(), 2);
    assert_eq!(nc3.render_ops.len(), 1);
    assert!(match nc3.render_ops.get(0).unwrap() {
      RenderOp::RenderProcessor(_, _) => true,
      _ => false,
    });

    Ok(())
  }
}

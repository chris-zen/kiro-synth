use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;

use ringbuf::{Consumer, Producer};
use thiserror::Error;

use kiro_audio_graph::connection::AudioSignal;
use kiro_audio_graph::{GraphTopology, Key, Node};
use kiro_audio_graph::{Graph, GraphError, HasId, NodeRef, Source};

use crate::buffers::Buffer;
use crate::controller::handles::{BufferHandle, ParametersHandle, ProcessorHandle};
use crate::controller::owned_data::{OwnedData, Ref};
use crate::controller::plan::{ControllerPlan, Operation, ParamSource};
use crate::controller::ProcParams;
use crate::messages::Message;
use crate::processor::{Processor, ProcessorBox, ProcessorFactory};
use crate::renderer::ops::processor::{Parameter, ProcessorContext};
use crate::renderer::plan::{RenderOp, RenderPlan};
use crate::renderer::port::{Input, Output, RenderPort};
use crate::{BufferBox, EngineConfig, ParamValue};
use kiro_audio_graph::port::{AudioOutPort, AudioInPort};
use kiro_audio_graph::key_store::KeyStore;
use std::sync::Arc;

#[derive(Error, Debug)]
pub enum ControllerError {
  #[error("Processor not found: {0:?}")]
  ProcessorNotFound(ProcessorHandle),

  #[error("Parameters not found: {0:?}")]
  ParametersNotFound(ParametersHandle),

  #[error("Buffer not found: {0:?}")]
  BufferNotFound(BufferHandle),

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
}

// TODO figure out how to remove Sync for ControllerError
unsafe impl Sync for ControllerError {}

pub type Result<T> = core::result::Result<T, ControllerError>;

struct NodeCache {
  node_ref: NodeRef,
  processor_key: Key<ProcessorBox>,
  parameters_key: Key<ProcParams>,
  allocated_audio_buffers: HashSet<Key<Buffer>>,
  audio_output_buffers: HashMap<Key<AudioOutPort>, Vec<Ref<Buffer>>>,
  render_ops: Vec<RenderOp>,
}

impl NodeCache {
  pub fn new(node_ref: NodeRef, processor_key: Key<ProcessorBox>, parameters_key: Key<ProcParams>) -> Self {
    Self {
      node_ref,
      processor_key,
      parameters_key,
      allocated_audio_buffers: HashSet::new(),
      audio_output_buffers: HashMap::new(),
      render_ops: Vec::new(),
    }
  }
}

struct UpdateContext {
  node_counts: HashMap<NodeRef, usize>,
  free_buffers: HashSet<Key<Buffer>>,
}

impl UpdateContext {
  pub fn new(topology: &GraphTopology, free_buffers: impl Iterator<Item=Key<Buffer>>) -> Self {
    let node_counts = topology
      .nodes
      .iter()
      .cloned()
      .zip(topology.counts.iter().cloned())
      .collect();

    let free_buffers = free_buffers.collect();

    Self {
      node_counts,
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

  processor_factories: HashMap<String, Rc<dyn ProcessorFactory>>,

  parameters: OwnedData<ProcParams>,
  _parameters: KeyStore<Arc<ParamValue>>,
  processors: OwnedData<ProcessorBox>,
  buffers: OwnedData<Buffer>,

  empty_buffer: Key<Buffer>,

  nodes: HashMap<NodeRef, NodeCache>,

  plan: ControllerPlan,
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
      processor_factories: HashMap::new(),
      processors: OwnedData::new(),
      parameters: OwnedData::new(),
      _parameters: KeyStore::new(),
      buffers,
      empty_buffer,
      nodes: HashMap::new(),
      plan: ControllerPlan::default(),
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

    let mut update_context = UpdateContext::new(&topology, self.buffers.keys());
    self.update_nodes(topology.nodes.as_slice(), graph, &mut update_context)?;

    for node_ref in topology.nodes {
      let node_cache = self
        .nodes
        .get(&node_ref)
        .ok_or(ControllerError::NodeCacheNotFound(node_ref))?;

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

    let parameters_key = unimplemented!();

    Ok(NodeCache::new(node_ref, processor_key, parameters_key))
  }

  /// Visit a node that has been invalidated and requires to regenerate the cache
  fn visit_invalidated_node(
    &mut self,
    node_ref: NodeRef,
    graph: &Graph,
    context: &mut UpdateContext,
  ) -> Result<()> {
    self.clear_node_cache(node_ref, context);

    let node = graph
      .get_node(node_ref)
      .map_err(|_| ControllerError::NodeNotFound(node_ref))?;

    let audio_output_buffers = self.allocate_audio_output_buffers(node, context);
    let audio_output_render_ports = self.build_audio_output_render_ports(&audio_output_buffers);
    let audio_input_render_ports = self.build_audio_input_render_ports(node)?;
    let parameter_render_ports = self.build_parameter_render_ports(node);

    self.release_input_buffers(node, context);

    self.update_node_cache(
      node_ref,
      audio_output_buffers,
      audio_output_render_ports,
      audio_input_render_ports,
      parameter_render_ports,
    );

    Ok(())
  }

  fn update_node_cache(
    &mut self,
    node_ref: NodeRef,
    audio_output_buffers: HashMap<Key<AudioOutPort>, Vec<Ref<Buffer>>>,
    audio_output_render_ports: Vec<RenderPort<Output>>,
    audio_input_render_ports: Vec<RenderPort<Input>>,
    parameter_render_ports: Vec<Parameter>,
  ) -> Result<()> {
    let node_cache = self
      .nodes
      .get_mut(&node_ref)
      .ok_or(ControllerError::NodeCacheNotFound(node_ref))?;

    node_cache.allocated_audio_buffers = audio_output_buffers
      .iter()
      .flat_map(|(port_key, buffers)| buffers)
      .map(|buffer_ref| buffer_ref.key)
      .collect::<HashSet<Key<Buffer>>>();

    node_cache.audio_output_buffers = audio_output_buffers;

    let processor = self
      .processors
      .get(node_cache.processor_key)
      .ok_or_else(|| {
        ControllerError::ProcessorNotFound(ProcessorHandle(node_cache.processor_key))
      })?;

    let processor_context = ProcessorContext::new(
      audio_input_render_ports,
      audio_output_render_ports,
      parameter_render_ports,
    );

    node_cache
      .render_ops
      .push(RenderOp::RenderProcessor(processor, processor_context));

    Ok(())
  }

  fn clear_node_cache(&mut self, node_ref: NodeRef, context: &mut UpdateContext) -> Result<()> {
    let node_cache = self
      .nodes
      .get_mut(&node_ref)
      .ok_or(ControllerError::NodeCacheNotFound(node_ref))?;

    context.add_to_free_buffers(&node_cache.allocated_audio_buffers);
    node_cache.allocated_audio_buffers.clear();
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
  ) -> Vec<RenderPort<Output>> {
    let mut port_keys = buffers.keys().cloned().collect::<Vec<Key<AudioOutPort>>>();
    port_keys.sort();

    port_keys
      .into_iter()
      .map(|port_key| {
        let port_buffers = buffers.get(&port_key).unwrap();
        RenderPort::new(port_buffers.clone())
      })
      .collect()
  }

  fn build_audio_input_render_ports(&mut self, node: &Node) -> Result<Vec<RenderPort<Input>>> {
    let mut port_keys = node
      .audio_inputs()
      .keys()
      .cloned()
      .collect::<Vec<Key<AudioInPort>>>();
    port_keys.sort();

    let mut render_ports = Vec::<RenderPort<Input>>::new();
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

  fn build_empty_audio_input_render_port(&self, channels: usize) -> Result<RenderPort<Input>> {
    let empty_buffer = self.buffers.get(self.empty_buffer).unwrap();
    let buffers = (0..channels)
      .map(|_| empty_buffer.clone())
      .collect::<Vec<Ref<Buffer>>>();
    Ok(RenderPort::new(buffers))
  }

  fn build_audio_input_render_port(
    &self,
    source: &Source<AudioSignal>,
  ) -> Result<RenderPort<Input>> {
    // TODO find a better way to express that several things here should be defined at this point, then the unwraps are safe.
    match source {
      Source::AudioOut { node_ref, key, .. } => {
        let node_cache = self.nodes.get(node_ref).unwrap();
        let buffers = node_cache
          .audio_output_buffers
          .get(&key.unwrap())
          .unwrap()
          .clone();
        Ok(RenderPort::new(buffers))
      }
      Source::MidiOut { node_ref: _, key: _, .. } => {
        unimplemented!()
      }
    }
  }

  fn build_parameter_render_ports(&self, _node: &Node) -> Vec<Parameter> {
    unimplemented!()
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

    self.release_input_buffers(node, context);

    // Mark output buffers as allocated

    let node_cache = self
      .nodes
      .get(&node_ref)
      .ok_or(ControllerError::NodeCacheNotFound(node_ref))?;

    context.remove_from_free_buffers(&node_cache.allocated_audio_buffers);

    Ok(())
  }

  /// Release input buffers that are not used anymore
  fn release_input_buffers(&mut self, node: &Node, context: &mut UpdateContext) -> Result<()> {
    for source_node_ref in node.sources() {
      let count = *context
        .node_counts
        .entry(source_node_ref)
        .and_modify(|e| *e = *e - 1)
        .or_default();
      if count <= 0 {
        let source_node_cache = self
          .nodes
          .get_mut(&source_node_ref)
          .ok_or(ControllerError::NodeCacheNotFound(source_node_ref))?;

        context.add_to_free_buffers(&source_node_cache.allocated_audio_buffers);
        source_node_cache.allocated_audio_buffers.clear();
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

  // ----------------------------------------------------------------------------------------------

  pub fn add_buffer(&mut self, buffer: Buffer) -> BufferHandle {
    BufferHandle(self.buffers.add(buffer))
  }

  pub fn add_parameters(&mut self, parameters: ProcParams) -> ParametersHandle {
    ParametersHandle(self.parameters.add(parameters))
  }

  pub fn add_processor(&mut self, processor: impl Processor + 'static) -> ProcessorHandle {
    ProcessorHandle(self.processors.add(Box::new(processor)))
  }

  pub fn get_render_plan(&mut self) -> &mut ControllerPlan {
    &mut self.plan
  }

  pub fn send_render_plan(&mut self) -> Result<()> {
    let render_plan = self.build_render_plan()?;
    self.plan = ControllerPlan::default();
    self
      .tx
      .push(Message::MoveRenderPlan(Box::new(render_plan)))
      .map_err(|msg| {
        drop(msg);
        ControllerError::SendFailure
      })
  }

  fn build_render_plan(&mut self) -> Result<RenderPlan> {
    let mut render_plan = RenderPlan::default();
    for op in self.plan.operations.iter() {
      match op {
        Operation::RenderProcessor {
          processor,
          audio_inputs,
          audio_outputs,
          parameters,
        } => {
          let processor_ref = self
            .processors
            .get(*processor)
            .ok_or(ControllerError::ProcessorNotFound(*processor))?;
          let processor_context =
            self.build_processor_context(audio_inputs, audio_outputs, parameters)?;
          render_plan
            .operations
            .push(RenderOp::RenderProcessor(processor_ref, processor_context));
        }
        Operation::RenderOuput { audio_inputs } => {
          let audio_input_port = self.build_render_port::<Input>(audio_inputs)?;
          render_plan
            .operations
            .push(RenderOp::RenderOutput(audio_input_port));
        }
      }
    }
    Ok(render_plan)
  }

  fn build_processor_context(
    &self,
    audio_inputs: &Vec<Vec<BufferHandle>>,
    audio_outputs: &Vec<Vec<BufferHandle>>,
    parameters: &Vec<ParamSource>,
  ) -> Result<ProcessorContext> {
    let audio_input_ports = self.build_render_ports::<Input>(audio_inputs)?;
    let audio_output_ports = self.build_render_ports::<Output>(audio_outputs)?;
    let parameters = self.build_render_params(parameters)?;

    Ok(ProcessorContext::new(
      audio_input_ports,
      audio_output_ports,
      parameters,
    ))
  }

  fn build_render_ports<IO>(
    &self,
    buffers: &Vec<Vec<BufferHandle>>,
  ) -> Result<Vec<RenderPort<IO>>> {
    buffers
      .iter()
      .try_fold(Vec::<RenderPort<IO>>::new(), |mut ports, handlers| {
        let port = self.build_render_port(handlers)?;
        ports.push(port);
        Ok(ports)
      })
  }

  fn build_render_port<IO>(&self, handlers: &Vec<BufferHandle>) -> Result<RenderPort<IO>> {
    let mut channels = Vec::with_capacity(handlers.capacity());
    for handle in handlers {
      let channel = self
        .buffers
        .get(*handle)
        .ok_or(ControllerError::BufferNotFound(*handle))?;
      channels.push(channel);
    }
    Ok(RenderPort::<IO>::new(channels))
  }

  fn build_render_params(&self, parameters: &Vec<ParamSource>) -> Result<Vec<Parameter>> {
    parameters
      .iter()
      .try_fold(Vec::<Parameter>::new(), |mut parameters, source| {
        match source {
          ParamSource::Value(parameters_handle, index, buffer_handle) => {
            let params = self
              .parameters
              .get(*parameters_handle)
              .ok_or(ControllerError::ParametersNotFound(*parameters_handle))?;
            let buffer = self
              .buffers
              .get(*buffer_handle)
              .ok_or(ControllerError::BufferNotFound(*buffer_handle))?;
            parameters.push(Parameter::Value(params, *index, buffer));
          }
          ParamSource::Buffer(buffer_handle) => {
            let buffer = self
              .buffers
              .get(*buffer_handle)
              .ok_or(ControllerError::BufferNotFound(*buffer_handle))?;
            parameters.push(Parameter::Buffer(buffer));
          }
        }
        Ok(parameters)
      })
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

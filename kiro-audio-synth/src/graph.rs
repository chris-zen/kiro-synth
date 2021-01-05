use anyhow::Result;
use std::time::Duration;

use kiro_audio_driver::{AudioConfig, AudioDriver, AudioHandler};
use kiro_audio_engine::{Engine, EngineConfig, ParamSource, ParamValue, Renderer};
use kiro_audio_graph::Graph;

use crate::analog_osc::AnalogOsc;
use crate::audio_handler::RendererAudioHandler;

pub fn graph_main() -> Result<()> {
  let audio_config = AudioConfig::default();

  let mut engine_config = EngineConfig::default();
  engine_config.buffer_size = audio_config.buffer_size;

  let engine = Engine::with_config(engine_config);
  let (mut controller, renderer) = engine.split();

  let renderer_handler = RendererAudioHandler::new(renderer);
  let audio_driver = AudioDriver::new(audio_config, renderer_handler)?;

  let mut graph = Graph::new();
  let osc1 = graph.add_node("analog-osc-1", AnalogOsc::node_descriptor())?;
  let osc2 = graph.add_node("analog-osc-2", AnalogOsc::node_descriptor())?;
  let amp2 = graph.param(osc2, AnalogOsc::AMPLITUDE)?;
  graph.connect(osc1, amp2)?;

  controller.update_graph(&graph)?;

  audio_driver.start()?;

  loop {
    controller.process_messages();
    std::thread::sleep(Duration::from_secs(1));
  }
}

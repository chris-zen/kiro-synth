use anyhow::Result;
use std::time::Duration;

use kiro_audio_driver::{AudioConfig, AudioDriver};
use kiro_audio_engine::{Engine, EngineConfig};
use kiro_audio_graph::Graph;

use crate::analog_osc::AnalogOsc;
use crate::audio_handler::RendererAudioHandler;
use kiro_audio_engine::processor::GenericProcessorFactory;

mod analog_osc;
mod audio_handler;

fn main() -> Result<()> {
  let audio_config = AudioConfig::default();
  let sample_rate = audio_config.sample_rate as f32;

  let mut engine_config = EngineConfig::default();
  engine_config.buffer_size = audio_config.buffer_size;

  let engine = Engine::with_config(engine_config);
  let (mut controller, renderer) = engine.split();

  let renderer_handler = RendererAudioHandler::new(renderer);
  let audio_driver = AudioDriver::new(audio_config, renderer_handler)?;

  let factory = GenericProcessorFactory::new()
    .with_factory(AnalogOsc::descriptor().class(), move |_node| {
      Some(Box::new(AnalogOsc::new(sample_rate)))
    });

  controller.register_processor_factory(factory);

  let mut graph = Graph::new();

  let osc1 = graph.add_node("analog-osc-1", AnalogOsc::descriptor())?;
  let osc1_freq = graph.param(osc1, AnalogOsc::FREQUENCY)?;

  let osc2 = graph.add_node("analog-osc-2", AnalogOsc::descriptor())?;
  let osc2_freq = graph.param(osc2, AnalogOsc::FREQUENCY)?;
  let osc2_amp = graph.param(osc2, AnalogOsc::AMPLITUDE)?;
  let osc2_pitch_bend = graph.param(osc2, AnalogOsc::PITCH_BEND)?;
  graph.connect(osc1, osc2_pitch_bend)?;

  let osc2_out = graph.audio_output(osc2, AnalogOsc::OUT)?;
  graph.bind_output(osc2_out, "OUT")?;

  controller.update_graph(&graph)?;

  let mut osc1_freq_value = 1.0;
  controller.set_param_value(osc1_freq, osc1_freq_value)?;
  controller.set_param_value(osc2_freq, 220.0)?;

  audio_driver.start()?;

  loop {
    controller.process_messages();
    std::thread::sleep(Duration::from_secs(1));
    osc1_freq_value += 1.0;
    controller.set_param_value(osc1_freq, osc1_freq_value)?;
    println!("{}", osc1_freq_value);
  }
}

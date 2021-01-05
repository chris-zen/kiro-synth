use anyhow::Result;
use std::time::Duration;

use kiro_audio_driver::{AudioConfig, AudioDriver, AudioHandler};
use kiro_audio_engine::buffers::Buffer;
use kiro_audio_engine::{Engine, ParamSource, ParamValue, Renderer};

use crate::analog_osc::AnalogOsc;
use crate::audio_handler::RendererAudioHandler;

pub fn controller_main() -> Result<()> {
  let engine = Engine::new();
  let (mut controller, renderer) = engine.split();

  let renderer_handler = RendererAudioHandler::new(renderer);
  let audio_config = AudioConfig::default();
  let audio_driver = AudioDriver::new(audio_config.clone(), renderer_handler)?;

  let buff1 = controller.add_buffer(Buffer::new(audio_config.buffer_size));
  let buff2 = controller.add_buffer(Buffer::new(audio_config.buffer_size));
  let buff3 = controller.add_buffer(Buffer::new(audio_config.buffer_size));

  let params1 = controller.add_parameters(vec![ParamValue::new(1.0)]);
  let _params2 = controller.add_parameters(vec![ParamValue::new(1.0)]);

  let osc1 = AnalogOsc::new(audio_config.sample_rate as f32, 1.0);
  let proc1 = controller.add_processor(osc1);

  let osc2 = AnalogOsc::new(audio_config.sample_rate as f32, 220.0);
  let proc2 = controller.add_processor(osc2);

  let render_plan = controller.get_render_plan();
  render_plan.render_processor(
    proc1,
    vec![],
    vec![vec![buff1]],
    vec![ParamSource::Value(params1, 0, buff3)],
  );
  render_plan.render_processor(
    proc2,
    vec![],
    vec![vec![buff2]],
    vec![ParamSource::Buffer(buff1)],
  );
  render_plan.render_output(vec![buff2]);
  controller.send_render_plan()?;

  audio_driver.start()?;

  loop {
    controller.process_messages();
    std::thread::sleep(Duration::from_secs(1));
  }
}

use kiro_audio_driver::AudioHandler;
use kiro_audio_engine::Renderer;

pub struct RendererAudioHandler(Renderer);

impl RendererAudioHandler {
  pub fn new(renderer: Renderer) -> Self {
    Self(renderer)
  }
}

impl AudioHandler for RendererAudioHandler {
  fn process(&mut self, data: &mut [f32], channels: usize) {
    data.iter_mut().for_each(|v| *v = 0.0);
    let input = [0.0f32; 0];
    self.0.render(&input[..0], 0, data, channels);
  }
}

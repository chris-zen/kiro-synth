pub mod widgets;
mod model;
mod view;

use std::sync::{Arc, Mutex};

use druid::{WindowDesc, LocalizedString, AppLauncher, Env, theme, Color};

use kiro_synth_core::float::Float;

use crate::synth::SynthClient;

pub use model::SynthModel;


pub fn start<F: Float + 'static>(synth_model: SynthModel,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) {

  let data = synth_model.clone();

  let window = WindowDesc::new(move || view::build(&synth_model, synth_client.clone()))
      .title(
        LocalizedString::new("custom-widget-demo-window-title")
            .with_placeholder("Kiro Synth")
      )
      .window_size((340.0, 480.0));

  AppLauncher::with_window(window)
      .configure_env(setup_theme)
      .use_simple_logger()
      .launch(data)
      .expect("UI launch failed");
}

fn setup_theme(env: &mut Env, _data: &SynthModel) {
  widgets::knob::theme::init(env);

  env.set(theme::WINDOW_BACKGROUND_COLOR, Color::grey8(46));
  env.set(theme::TEXT_SIZE_NORMAL, 11.0);
}

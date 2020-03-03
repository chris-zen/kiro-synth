pub mod knob;
mod model;
mod widget;
mod params;

use std::sync::{Arc, Mutex};

use ringbuf::Producer;

use druid::{WindowDesc, LocalizedString, AppLauncher, AppDelegate, Data};

use kiro_synth_core::float::Float;
use kiro_synth_engine::event::Event;

use crate::synth::SynthClient;

pub use params::SynthParams;
pub use model::Model;


pub fn start<F: Float + 'static>(synth_params: SynthParams,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) {

  let model = Model::new(&synth_params);

  let window = WindowDesc::new(move || widget::build(&synth_params, synth_client.clone()))
      .title(
        LocalizedString::new("custom-widget-demo-window-title")
            .with_placeholder("Kiro Synth")
      )
      .window_size((480.0, 440.0));

  AppLauncher::with_window(window)
      .configure_env(|env, _data| knob::theme::init(env))
      .use_simple_logger()
      .launch(model)
      .expect("UI launch failed");
}

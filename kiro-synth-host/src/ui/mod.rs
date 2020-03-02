pub mod knob;
mod model;
mod widget;
mod params;

use druid::{WindowDesc, LocalizedString, AppLauncher};

use crate::ui::model::Model;

pub use params::SynthParams;

pub fn start(synth_params: SynthParams) {

  let model = Model::new(&synth_params);

  let window = WindowDesc::new(move || widget::build(&synth_params))
      .title(
        LocalizedString::new("custom-widget-demo-window-title")
            .with_placeholder("Kiro Synth")
      )
      .window_size((400.0, 300.0));

  AppLauncher::with_window(window)
      .configure_env(|env, _data| knob::theme::init(env))
      .use_simple_logger()
      // .launch(KnobModel::new(1.0, -0.5))
      .launch(model)
      .expect("UI launch failed");
}

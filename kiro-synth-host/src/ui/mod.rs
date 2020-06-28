pub mod controllers;
pub mod widgets;
mod model;
mod view;
mod icons;

use std::sync::{Arc, Mutex};

use druid::{WindowDesc, LocalizedString, AppLauncher, Env, theme, Color, Data};

use kiro_synth_core::float::Float;

use crate::synth::SynthClient;

pub use model::Synth;


pub fn start<F: Float + 'static>(synth_model: Synth,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) {

  let data = synth_model.clone();

  let window = WindowDesc::new(move || view::build(&synth_model, synth_client.clone()))
      .title(
        LocalizedString::new("custom-widget-demo-window-title")
            .with_placeholder("Kiro Synth")
      )
      .window_size((500.0, 514.0));

  AppLauncher::with_window(window)
      .configure_env(setup_theme)
      .use_simple_logger()
      .launch(data)
      .expect("UI launch failed");
}

pub const KNOB_VALUE: Color = Color::rgb8(236, 138, 56);
pub const KNOB_MODULATION: Color = Color::rgb8(204, 76, 0);
pub const KNOB_CONFIG: Color = Color::rgb8(253, 191, 0);

pub const GREY_23: Color = Color::grey8(23);
pub const GREY_46: Color = Color::grey8(46);
pub const GREY_54: Color = Color::grey8(54);
pub const GREY_65: Color = Color::grey8(65);
pub const GREY_74: Color = Color::grey8(74);
pub const GREY_83: Color = Color::grey8(83);
pub const GREY_214: Color = Color::grey8(214);

fn setup_theme<T: Data>(env: &mut Env, _data: &T) {
  widgets::knob::theme::init(env);

  env.set(theme::WINDOW_BACKGROUND_COLOR, GREY_46);
  env.set(theme::TEXT_SIZE_NORMAL, 11.0);
  env.set(theme::LABEL_COLOR, GREY_214);
  env.set(widgets::knob::theme::KNOB_VALUE_FG_COLOR, KNOB_VALUE);
  env.set(widgets::knob::theme::KNOB_VALUE_BG_COLOR, GREY_83);
  env.set(widgets::knob::theme::KNOB_MODULATION_VALUE_FG_COLOR, KNOB_MODULATION);
  env.set(widgets::knob::theme::KNOB_MODULATION_VALUE_BG_COLOR, GREY_54);
  env.set(widgets::knob::theme::KNOB_MODULATION_TOTAL_AMOUNT_COLOR, KNOB_CONFIG);
}

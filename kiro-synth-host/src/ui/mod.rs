pub mod controllers;
mod icons;
pub mod data;
mod view;
pub mod widgets;

use std::sync::{Arc, Mutex};

use druid::{theme, AppLauncher, Color, Data, Env, WindowDesc};

use kiro_synth_dsp::float::Float;

use crate::synth::SynthClient;

pub use data::AppData;
use widgets::knob;

pub fn start<F: Float + 'static>(app_data: AppData, synth_client: Arc<Mutex<SynthClient<F>>>) {
  let window = WindowDesc::new(move || view::build(synth_client.clone()))
    .title("Kiro Synth")
    .window_size((600.0, 514.0))
    .resizable(false);

  AppLauncher::with_window(window)
    .configure_env(setup_theme)
    .use_simple_logger()
    .launch(app_data)
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

  env.set(knob::theme::KNOB_VALUE_FG_COLOR, KNOB_VALUE);
  env.set(knob::theme::KNOB_VALUE_BG_COLOR, GREY_83);
  env.set(knob::theme::KNOB_MODULATION_VALUE_FG_COLOR, KNOB_MODULATION);
  env.set(knob::theme::KNOB_MODULATION_VALUE_BG_COLOR, GREY_54);
  env.set(knob::theme::KNOB_MODULATION_TOTAL_AMOUNT_COLOR, KNOB_CONFIG);
  env.set(
    knob::theme::KNOB_MODULATION_CONFIG_AMOUNT_COLOR,
    KNOB_CONFIG,
  );
}

use druid::{Widget, Env};
use druid::widget::{Flex, WidgetExt};

use crate::ui::model::{Synth, Dca, ZeroIndex};
use crate::ui::view::{build_tabs, build_switcher, build_knob_value};


pub struct DcaView;

impl DcaView {
  pub fn new(synth_model: &Synth) -> impl Widget<Synth> {
    let eg_len = synth_model.eg.len();
    let tabs = build_tabs(eg_len, |_index| "DCA".to_string())
        .lens(ZeroIndex);

    build_switcher(tabs,
                   |_data: &Synth, _env: &Env| 0usize,
                   move |_index: &usize, _data: &Synth, _env: &Env| {
                     Box::new(build_dca_view().lens(Synth::dca))
                   })
  }
}

fn build_dca_view() -> impl Widget<Dca> {

  Flex::row()
      .with_child(
        build_knob_value("Amplitude", " dB").lens(Dca::amplitude)
      )
      .with_child(
        build_knob_value("Pan", "").lens(Dca::pan)
      )
}

use druid::{Widget, Env};
use druid::widget::{Flex, WidgetExt};

use crate::ui::model::{SynthModel, Filter, FilterFromSynth};
use crate::ui::view::{build_tabs, build_switcher, build_knob_value};

pub struct FiltersView;

impl FiltersView {
  pub fn new(synth_model: &SynthModel) -> impl Widget<SynthModel> {

    let filter_len = synth_model.filter.len();
    let tabs = build_tabs(filter_len, |index| format!("FILTER{}", index + 1))
        .lens(SynthModel::filter_index);

    build_switcher(tabs,
                   |data: &SynthModel, _env: &Env| data.filter_index,
                   move |_index: &usize, _data: &SynthModel, _env: &Env| {
                     Box::new(build_filter_view().lens(FilterFromSynth))
                   })
  }
}

fn build_filter_view() -> impl Widget<Filter> {

  Flex::row()
      .with_child(
        build_knob_value("Mode", "").lens(Filter::mode)
      )
      .with_child(
        build_knob_value("Cutoff", " Hz").lens(Filter::freq)
      )
      .with_child(
        build_knob_value("Res", "").lens(Filter::q)
      )
      .with_flex_spacer(1.0)
}

use std::sync::{Arc, Mutex};

use druid::widget::WidgetExt;
use druid::widget::{CrossAxisAlignment, Flex};
use druid::Widget;

use kiro_synth_core::float::Float;

use crate::synth::SynthClient;
use crate::ui::data::synth::Synth;
use crate::ui::view::synth::dca::DcaView;
use crate::ui::view::synth::filters::FiltersView;
use crate::ui::view::synth::modulators::ModulatorsView;
use crate::ui::view::synth::oscillators::OscillatorsView;

mod dca;
mod filters;
mod modulators;
mod oscillators;

pub fn build<F: Float + 'static>(
  synth_data: &Synth,
  synth_client: Arc<Mutex<SynthClient<F>>>,
) -> impl Widget<Synth> {
  let height = 114.0;

  Flex::column()
    .with_child(
      OscillatorsView::build(synth_data, synth_client.clone())
        .fix_height(height)
        .padding(4.0),
    )
    .with_child(
      Flex::row()
        .with_flex_child(
          FiltersView::build(synth_data)
            .fix_height(height)
            .padding(4.0),
          1.0,
        )
        .with_child(DcaView::build(synth_data).fix_height(height).padding(4.0))
        .must_fill_main_axis(true),
    )
    .with_child(
      ModulatorsView::build(synth_data, synth_client)
        .fix_height(height * 2.0)
        .padding(4.0),
    )
    .cross_axis_alignment(CrossAxisAlignment::Start)
}

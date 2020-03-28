use std::sync::{Arc, Mutex};

use druid::{Widget, Data, Env};
use druid::widget::{Flex, WidgetExt, Label, Container, ViewSwitcher};

use kiro_synth_core::float::Float;

use crate::synth::SynthClient;
use crate::ui::model::{SynthModel, Osc, EnvGen, KnobDataFromParam, Param, Filter, Dca, OscFromSynth, EgFromSynth, FilterFromSynth, ZeroIndex};
use crate::ui::widgets::knob::{KnobData, Knob};
use crate::ui::{GREY_83, GREY_65, GREY_74};
use crate::ui::widgets::tab::Tab;


pub fn build<F: Float + 'static>(synth_model: &SynthModel,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthModel> {

  Flex::column()
    .with_child(
      build_osc_tabs(synth_model, synth_client.clone()).padding(4.0),
      1.0
    )
    .with_child(
      build_eg_tabs(synth_model, synth_client.clone()).padding(4.0),
      1.0
    )
    .with_child(
      Flex::row()
          .with_child(
            build_filter_tabs(synth_model, synth_client.clone()).padding(4.0),
            1.0
          )
          .with_child(
            build_dca_tabs(synth_model, synth_client.clone()).padding(4.0),
            1.0
          ),
      1.0
    )
}

fn build_osc_tabs<F: Float + 'static>(synth_model: &SynthModel,
                                      synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthModel> {

  let tabs = build_tabs(synth_model.osc.len(), |index| format!("OSC{}", index + 1))
      .lens(SynthModel::osc_index);

  build_switcher(tabs,
                 |data: &SynthModel, _env: &Env| data.osc_index,
                 move |index: &usize, data: &SynthModel, _env: &Env| {
                   Box::new(
                     build_osc_view(&data.osc[*index], synth_client.clone())
                         .lens(OscFromSynth)
                   )
                 })
}

fn build_osc_view<F: Float + 'static>(osc_model: &Osc,
                                      synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Osc> {

  let shape_client = synth_client.clone();
  let shape_fn = move |index: usize| shape_client.lock().unwrap().waveforms().name(index).to_string();

  Flex::row()
      .with_child(
        build_knob_enum("Shape", shape_fn, &osc_model.shape, synth_client.clone())
            .lens(Osc::shape),
        1.0
      )
      .with_child(
        build_knob_value("Octaves", "", &osc_model.octaves, synth_client.clone())
            .lens(Osc::octaves),
        1.0
      )
      .with_child(
        build_knob_value("Semitones", "", &osc_model.semitones, synth_client.clone())
            .lens(Osc::semitones),
        1.0
      )
      .with_child(
        build_knob_value("Cents", "", &osc_model.cents, synth_client.clone())
            .lens(Osc::cents),
        1.0
      )
      .with_child(
        build_knob_value("Amplitude", "", &osc_model.amplitude, synth_client.clone())
            .lens(Osc::amplitude),
        1.0
      )
}

fn build_eg_tabs<F: Float + 'static>(synth_model: &SynthModel,
                                     synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthModel> {

  let tabs = build_tabs(synth_model.eg.len(), |index| format!("EG{}", index + 1))
      .lens(SynthModel::eg_index);

  build_switcher(tabs,
                 |data: &SynthModel, _env: &Env| data.eg_index,
                 move |index: &usize, data: &SynthModel, _env: &Env| {
                   Box::new(
                     build_eg_view(&data.eg[*index], synth_client.clone())
                         .lens(EgFromSynth)
                   )
                 })
}

fn build_eg_view<F: Float + 'static>(eg_model: &EnvGen,
                                     synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<EnvGen> {

  Flex::row()
    .with_child(
      build_knob_value("Attack", " s", &eg_model.attack, synth_client.clone())
            .lens(EnvGen::attack),
      1.0
    )
    .with_child(
      build_knob_value("Decay", " s", &eg_model.decay, synth_client.clone())
            .lens(EnvGen::decay),
      1.0
    )
    .with_child(
      build_knob_value("Sustain", "", &eg_model.sustain, synth_client.clone())
            .lens(EnvGen::sustain),
      1.0
    )
    .with_child(
      build_knob_value("Release", " s", &eg_model.release, synth_client.clone())
            .lens(EnvGen::release),
      1.0
    )
    .with_child(
      build_knob_value("Mode", "", &eg_model.mode, synth_client.clone())
            .lens(EnvGen::mode),
      1.0
    )
    .with_child(
      build_knob_value("Intensity", "", &eg_model.dca_intensity, synth_client.clone())
            .lens(EnvGen::dca_intensity),
      1.0
    )
}

fn build_filter_tabs<F: Float + 'static>(synth_model: &SynthModel,
                                         synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthModel> {

  let tabs = build_tabs(synth_model.filter.len(), |index| format!("FILTER{}", index + 1))
      .lens(SynthModel::filter_index);

  build_switcher(tabs,
                 |data: &SynthModel, _env: &Env| data.filter_index,
                 move |index: &usize, data: &SynthModel, _env: &Env| {
                   Box::new(
                     build_filter_view(&data.filter[*index], synth_client.clone())
                         .lens(FilterFromSynth)
                   )
                 })
}

fn build_filter_view<F: Float + 'static>(filter_model: &Filter,
                                         synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Filter> {

  Flex::row()
    .with_child(
      build_knob_value("Mode", "", &filter_model.mode, synth_client.clone())
            .lens(Filter::mode),
      1.0
    )
    .with_child(
      build_knob_value("Cutoff", " Hz", &filter_model.freq, synth_client.clone())
            .lens(Filter::freq),
      1.0
    )
    .with_child(
      build_knob_value("Res", "", &filter_model.q, synth_client.clone())
            .lens(Filter::q),
      1.0
    )
}

fn build_dca_tabs<F: Float + 'static>(synth_model: &SynthModel,
                                      synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthModel> {

  let tabs = build_tabs(synth_model.eg.len(), |_index| "DCA".to_string())
      .lens(ZeroIndex);

  build_switcher(tabs,
                 |_data: &SynthModel, _env: &Env| 0usize,
                 move |_index: &usize, data: &SynthModel, _env: &Env| {
                   Box::new(
                     build_dca_view(&data.dca, synth_client.clone())
                         .lens(SynthModel::dca)
                   )
                 })
}

fn build_dca_view<F: Float + 'static>(dca_model: &Dca,
                                      synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Dca> {

  Flex::row()
    .with_child(
      build_knob_value("Amplitude", " dB", &dca_model.amplitude, synth_client.clone())
            .lens(Dca::amplitude),
      1.0
    )
    .with_child(
      build_knob_value("Pan", "", &dca_model.pan, synth_client.clone())
            .lens(Dca::pan),
      1.0
    )
}

fn build_tabs(n: usize, title: impl Fn(usize) -> String + 'static) -> impl Widget<usize> {
  let mut tabs = Flex::row();
  for tab_index in 0..n {
    let label = Label::<usize>::new((title)(tab_index))
        .padding((6.0, 4.0, 4.0, 2.0));

    let on_click = move |index: &mut usize, _env: &Env| *index = tab_index;
    let is_selected = move |index: &usize| *index == tab_index;
    let tab = Tab::new(label, on_click, is_selected)
        .border_width(2.0)
        .selected_border_color(GREY_83)
        .unselected_border_color(GREY_65)
        .hover_border_color(GREY_74)
        .selected_background(GREY_83)
        .unselected_background(GREY_65)
        .hover_background(GREY_74)
        .corner_radius(2.0);

    tabs.add_child(tab, 0.0);
    tabs.add_spacer(4.0);
  }
  tabs
}

fn build_switcher<T, U, W>(tabs: W,
                           child_picker: impl Fn(&T, &Env) -> U + 'static,
                           child_builder: impl Fn(&U, &T, &Env) -> Box<dyn Widget<T>> + 'static) -> impl Widget<T>
  where T: Data, U: PartialEq + 'static, W: Widget<T> + 'static {

  let switcher = ViewSwitcher::new(child_picker, child_builder);

  let body = Container::new(switcher.padding(6.0))
      .rounded(2.0)
      .border(GREY_83, 2.0);

  Flex::column()
      .with_child(tabs, 0.0)
      .with_child(body, 1.0)
}

fn build_knob_value<F: Float + 'static>(title: &'static str,
                                        unit: &'static str,
                                        param: &Param,
                                        synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Param> {

  let step = param.step.max(0.001);
  let precision = (-step.log10().floor()).max(0.0).min(3.0) as usize;
  let value_fn = move |data: &KnobData| {
    let value = (data.value / step).round() * step;
    format!("{:.*}{}", precision, value, unit)
  };

  build_knob(title, value_fn, param, synth_client)
}

fn build_knob_enum<F: Float + 'static>(title: &'static str,
                                       value_fn: impl Fn(usize) -> String + 'static,
                                       param: &Param,
                                       synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Param> {

  build_knob(title, move |data: &KnobData| value_fn(data.value as usize), param, synth_client)
}

fn build_knob<F: Float + 'static>(title: &'static str,
                                  value_fn: impl Fn(&KnobData) -> String + 'static,
                                  param: &Param,
                                  synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Param> {

  let param_ref = param.param_ref;
  let callback = move |data: &KnobData| {
    synth_client.lock().unwrap().send_param_value(param_ref, F::val(data.value));
  };

  Flex::column()
    .with_child(Label::new(title).center(), 0.0)
    .with_child(Knob::new(param.origin, param.min, param.max, param.step, callback).fix_size(48.0, 48.0).center(),0.0)
    .with_child(Label::new(move |data: &KnobData, _env: &Env| value_fn(data)).center(), 0.0)
    .lens(KnobDataFromParam)
}

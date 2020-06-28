use typenum::marker_traits::Unsigned;

use druid::kurbo::Rect;
use druid::widget::{Container, FillStrat, Flex, Label, Painter, WidgetExt};
use druid::{Env, PaintCtx, RenderContext, Widget};

use kiro_synth_engine::synth::MaxVoices;

use druid_icon::Icon;

use crate::ui::model::Synth;
use crate::ui::{icons, GREY_46, GREY_65, GREY_83};

pub struct HeaderView;

impl HeaderView {
  pub fn new() -> impl Widget<Synth> {
    let icon = Icon::new(&icons::LOGO_KIRO_SYNTH)
      .fill_strategy(FillStrat::ScaleDown)
      .fix_width(108.0)
      .fix_height(48.0)
      .padding((4.0, 0.0));

    Container::new(
      Flex::row()
        .with_child(icon)
        .with_flex_spacer(1.0)
        .with_child(Self::voices()),
    )
    .rounded(4.0)
    .background(GREY_65)
    .padding(4.0)
  }

  fn voices() -> impl Widget<Synth> {
    let value_fn = |data: &usize, _: &Env| format!("{}", data);

    let num_voices = Label::new(value_fn)
      .center()
      .padding(2.0)
      .fix_width(44.0)
      .background(Painter::new(Self::paint_voices))
      .lens(Synth::active_voices);

    Flex::column()
      .with_child(Label::new("VOICES"))
      .with_spacer(4.0)
      .with_child(num_voices)
      .padding(6.0)
  }

  fn paint_voices(ctx: &mut PaintCtx, num_voices: &usize, _env: &Env) {
    let max_voices = MaxVoices::to_usize();
    let size = ctx.size();
    let margin = 0.0;
    let fill_width = (size.width - 2.0 * margin) * *num_voices as f64 / max_voices as f64;
    ctx.fill(size.to_rect(), &GREY_46);
    ctx.fill(
      Rect::new(margin, margin, margin + fill_width, size.height - margin),
      &GREY_83,
    );
  }
}

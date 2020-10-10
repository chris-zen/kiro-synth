use typenum::marker_traits::Unsigned;

use druid::kurbo::{BezPath, Rect, Size};
use druid::widget::{Container, FillStrat, Flex, Label, Padding, Painter, SizedBox, WidgetExt};
use druid::{
  BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
  RenderContext, UpdateCtx, Widget, WidgetPod,
};

use druid_icon::Icon;
use kiro_synth_engine::synth::MaxVoices;

use crate::ui::data::header::{AudioLevel, Header, SelectedView};
use crate::ui::widgets::knob::theme::KNOB_MODULATION_VALUE_FG_COLOR;
use crate::ui::{icons, GREY_46, GREY_65};
use druid::theme::LABEL_COLOR;

const LEVEL_GREEN_BG: Color = Color::rgb8(23, 40, 11);
const LEVEL_RED_BG: Color = Color::rgb8(40, 11, 11);
const LEVEL_GREEN_FG: Color = Color::rgb8(55, 200, 113);
const LEVEL_RED_FG: Color = Color::rgb8(200, 55, 55);

pub struct HeaderView;

impl HeaderView {
  pub fn build() -> impl Widget<Header> {
    let icon = Icon::new(&icons::LOGO_KIRO_SYNTH)
      .fill_strategy(FillStrat::ScaleDown)
      .fix_width(108.0)
      .fix_height(48.0)
      .padding((4.0, 0.0));

    Container::new(
      Flex::row()
        .with_child(icon)
        .with_spacer(32.0)
        .with_child(Self::view_selectors())
        .with_flex_spacer(1.0)
        .with_child(Self::voices())
        .with_spacer(12.0)
        .with_child(Self::audio_levels())
        .with_spacer(8.0),
    )
    .rounded(4.0)
    .background(GREY_65)
    .padding(4.0)
  }

  fn view_selectors() -> impl Widget<Header> {
    Flex::row()
      .with_child(SelectionButton::new(SelectedView::Presets))
      .with_spacer(8.0)
      .with_child(SelectionButton::new(SelectedView::Synth))
      .with_spacer(8.0)
      .with_child(SelectionButton::new(SelectedView::Effects))
      .lens(Header::selected_view)
  }

  fn voices() -> impl Widget<Header> {
    let value_fn = |data: &usize, _: &Env| format!("{}", data);

    let num_voices = Label::new(value_fn)
      .center()
      .fix_size(44.0, 14.0)
      .background(Painter::new(Self::paint_voices))
      .lens(Header::active_voices);

    Flex::column()
      .with_child(Label::new("VOICES").fix_height(14.0))
      .with_spacer(1.0)
      .with_child(num_voices)
  }

  fn paint_voices(ctx: &mut PaintCtx, num_voices: &usize, env: &Env) {
    let max_voices = MaxVoices::to_usize();
    let size = ctx.size();
    let margin = 0.0;
    let width = (size.width - 2.0 * margin) * *num_voices as f64 / max_voices as f64;
    let color = env.get(KNOB_MODULATION_VALUE_FG_COLOR);
    let rect = Rect::new(margin, margin, margin + width, size.height - margin);
    ctx.fill(size.to_rect(), &GREY_46);
    ctx.fill(rect, &color);
  }

  fn audio_levels() -> impl Widget<Header> {
    let scale = Icon::new(&icons::LEVEL_METER_SCALE)
      .fill_strategy(FillStrat::ScaleDown)
      .fix_size(64.0, 12.0);

    Flex::column()
      .with_child(scale)
      .with_child(Self::audio_level().lens(Header::left_level))
      .with_child(Self::audio_level().lens(Header::right_level))
  }

  fn audio_level() -> impl Widget<AudioLevel> {
    SizedBox::empty()
      .fix_size(64.0, 9.0)
      .background(Painter::new(Self::paint_level))
  }

  fn paint_level(ctx: &mut PaintCtx, level: &AudioLevel, _env: &Env) {
    let size = ctx.size();
    let shape = LevelShapeBuilder::new(&size);

    ctx.fill(shape.green_bar(0.0), &LEVEL_GREEN_BG);
    ctx.fill(shape.max_bar(), &LEVEL_RED_BG);

    if level.level >= -60.0 {
      ctx.fill(shape.green_bar(level.level), &LEVEL_GREEN_FG);
    }

    if level.level > 0.0 {
      ctx.fill(shape.red_bar(level.level), &LEVEL_RED_FG);
    }

    if level.peak > -60.0 {
      let path = shape.peak_line(level.peak);
      let color = if level.peak <= 0.0 {
        &LEVEL_GREEN_FG
      } else {
        &LEVEL_RED_FG
      };
      ctx.stroke(&path, color, 1.0);
    }
  }
}

struct LevelShapeBuilder<'a> {
  size: &'a Size,
  total_width: f64,
  total_db_recip: f64,
}

impl<'a> LevelShapeBuilder<'a> {
  const MARGIN: f64 = 0.0;
  const MIN_DB: f64 = -60.0;
  const MAX_DB: f64 = 4.0;
  const TOTAL_DB: f64 = Self::MAX_DB - Self::MIN_DB;
  const DB_OFFSET: f64 = 0.0 - Self::MIN_DB;

  fn new(size: &'a Size) -> Self {
    let total_width = size.width - 2.0 * Self::MARGIN;
    let total_db_recip = Self::TOTAL_DB.recip();

    LevelShapeBuilder {
      size,
      total_width,
      total_db_recip,
    }
  }

  fn green_bar(&self, db: f64) -> Rect {
    let db = Self::MIN_DB.max(db).min(0.0) + Self::DB_OFFSET;
    let end_pos = self.total_width * db * self.total_db_recip;

    Rect::new(
      Self::MARGIN,
      Self::MARGIN,
      Self::MARGIN + end_pos + 1.0,
      self.size.height - Self::MARGIN,
    )
  }

  fn red_bar(&self, db: f64) -> Rect {
    let db = Self::MIN_DB.max(db) + Self::DB_OFFSET;
    let start_pos = self.total_width * Self::DB_OFFSET * self.total_db_recip;
    let end_pos = self.total_width * db * self.total_db_recip;

    Rect::new(
      Self::MARGIN + start_pos,
      Self::MARGIN,
      Self::MARGIN + end_pos + 1.0,
      self.size.height - Self::MARGIN,
    )
  }

  fn max_bar(&self) -> Rect {
    self.red_bar(Self::MAX_DB)
  }

  fn peak_line(&self, db: f64) -> BezPath {
    let db = Self::MIN_DB.max(db) + Self::DB_OFFSET;
    let pos = self.total_width * db * self.total_db_recip;

    let mut path = BezPath::new();
    path.move_to((pos, Self::MARGIN));
    path.line_to((pos, self.size.height - 2.0 * Self::MARGIN));
    path
  }
}

struct SelectionButton {
  view: SelectedView,
  child: WidgetPod<SelectedView, Padding<SelectedView>>,
}

impl SelectionButton {
  const BORDER_WIDTH: f64 = 1.0;

  pub fn new(view: SelectedView) -> Self {
    let title = view.title();
    let label = Label::new(title).with_text_size(14.0).padding((12.0, 6.0));

    SelectionButton {
      view,
      child: WidgetPod::new(label),
    }
  }
}

impl Widget<SelectedView> for SelectionButton {
  fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut SelectedView, env: &Env) {
    self.child.event(ctx, event, data, env);
    if !ctx.is_handled() {
      match event {
        Event::MouseDown(_) => {
          ctx.set_active(true);
          ctx.request_paint();
        }
        Event::MouseUp(_) => {
          if ctx.is_active() {
            ctx.set_active(false);
            ctx.request_paint();
            if ctx.is_hot() {
              *data = self.view.clone();
            }
          }
        }
        _ => (),
      }
    }
  }

  fn lifecycle(
    &mut self,
    ctx: &mut LifeCycleCtx,
    event: &LifeCycle,
    data: &SelectedView,
    env: &Env,
  ) {
    if let LifeCycle::HotChanged(_) = event {
      ctx.request_paint();
    }
    self.child.lifecycle(ctx, event, data, env);
  }

  fn update(
    &mut self,
    ctx: &mut UpdateCtx,
    _old_data: &SelectedView,
    data: &SelectedView,
    env: &Env,
  ) {
    self.child.update(ctx, data, env);
  }

  fn layout(
    &mut self,
    ctx: &mut LayoutCtx,
    bc: &BoxConstraints,
    data: &SelectedView,
    env: &Env,
  ) -> Size {
    bc.debug_check("SelectionButton");

    let child_size = self.child.layout(ctx, bc, data, env);
    self
      .child
      .set_layout_rect(ctx, data, env, child_size.to_rect());
    child_size
  }

  fn paint(&mut self, ctx: &mut PaintCtx, data: &SelectedView, env: &Env) {
    if self.view == *data || ctx.is_hot() {
      let border_width = Self::BORDER_WIDTH;
      let border_rect = ctx
        .size()
        .to_rect()
        .inset(border_width / -2.0)
        .to_rounded_rect(4.0);
      let color = env.get(LABEL_COLOR);
      ctx.stroke(border_rect, &color, border_width)
    }
    self.child.paint(ctx, data, env);
  }
}

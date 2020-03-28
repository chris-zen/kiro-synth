use std::f64::consts::FRAC_PI_2;

use druid::piet::RenderContext;
use druid::kurbo::{BezPath, Arc};
use druid::{Data, Widget, EventCtx, LifeCycle, PaintCtx, LifeCycleCtx, BoxConstraints, Size, LayoutCtx, Event, Env, UpdateCtx, Color, Point, Rect, WidgetPod, Vec2};
use druid::widget::BackgroundBrush;

use crate::ui::widgets::tab::theme::{TAB_CORNER_RADIUS, TAB_BORDER_WIDTH, TAB_UNSELECTED_BORDER_COLOR, TAB_SELECTED_BACKGROUND_COLOR, TAB_HOVER_BACKGROUND_COLOR, TAB_UNSELECTED_BACKGROUND_COLOR, TAB_SELECTED_BORDER_COLOR, TAB_HOVER_BORDER_COLOR};

const ARC_TOLERANCE: f64 = 1e-12;

mod theme {
  pub use druid::theme;
  use druid::{Key, Color};

  pub const TAB_CORNER_RADIUS: Key<f64> = Key::new("tab-corner-radius");
  pub const TAB_BORDER_WIDTH: Key<f64> = Key::new("tab-border-width");
  pub const TAB_SELECTED_BORDER_COLOR: Key<Color> = Key::new("tab-selected-border-color");
  pub const TAB_UNSELECTED_BORDER_COLOR: Key<Color> = Key::new("tab-unselected-border-color");
  pub const TAB_HOVER_BORDER_COLOR: Key<Color> = Key::new("tab-hover-border-color");
  pub const TAB_SELECTED_BACKGROUND_COLOR: Key<Color> = Key::new("tab-selected-background-color");
  pub const TAB_UNSELECTED_BACKGROUND_COLOR: Key<Color> = Key::new("tab-unselected-background-color");
  pub const TAB_HOVER_BACKGROUND_COLOR: Key<Color> = Key::new("tab-hover-background-color");
}

pub struct Tab<T: Data> {
  corner_radius: Option<f64>,
  border_width: Option<f64>,
  selected_border_color: Option<Color>,
  unselected_border_color: Option<Color>,
  hover_border_color: Option<Color>,
  selected_background: Option<BackgroundBrush<T>>,
  unselected_background: Option<BackgroundBrush<T>>,
  hover_background: Option<BackgroundBrush<T>>,
  inner: WidgetPod<T, Box<dyn Widget<T>>>,
  on_click: Box<dyn Fn(&mut T, &Env)>,
  is_selected: Box<dyn Fn(&T) -> bool>,
  selected: bool,
}

impl<T: Data> Tab<T> {
  pub fn new(widget: impl Widget<T> + 'static,
             on_click: impl Fn(&mut T, &Env) + 'static,
             is_selected: impl Fn(&T) -> bool + 'static) -> Self {
    Tab {
      corner_radius: None,
      border_width: None,
      selected_border_color: None,
      unselected_border_color: None,
      hover_border_color: None,
      selected_background: None,
      unselected_background: None,
      hover_background: None,
      inner: WidgetPod::new(Box::new(widget)),
      on_click: Box::new(on_click),
      is_selected: Box::new(is_selected),
      selected: false,
    }
  }

  pub fn corner_radius(self, radius: f64) -> Self {
    Self {
      corner_radius: Some(radius),
      .. self
    }
  }

  pub fn border_width(self, width: f64) -> Self {
    Self {
      border_width: Some(width),
      .. self
    }
  }

  pub fn border_color(self, color: impl Into<Color>) -> Self {
    let color = color.into();
    Self {
      selected_border_color: Some(color.clone()),
      unselected_border_color: Some(color.clone()),
      hover_border_color: Some(color),
      .. self
    }
  }

  pub fn selected_border_color(self, color: impl Into<Color>) -> Self {
    Self {
      selected_border_color: Some(color.into()),
      .. self
    }
  }

  pub fn unselected_border_color(self, color: impl Into<Color>) -> Self {
    Self {
      unselected_border_color: Some(color.into()),
      .. self
    }
  }

  pub fn hover_border_color(self, color: impl Into<Color>) -> Self {
    Self {
      hover_border_color: Some(color.into()),
      .. self
    }
  }

  pub fn selected_background(self, brush: impl Into<BackgroundBrush<T>>) -> Self {
    Self {
      selected_background: Some(brush.into()),
      .. self
    }
  }

  pub fn unselected_background(self, brush: impl Into<BackgroundBrush<T>>) -> Self {
    Self {
      unselected_background: Some(brush.into()),
      .. self
    }
  }

  pub fn hover_background(self, brush: impl Into<BackgroundBrush<T>>) -> Self {
    Self {
      hover_background: Some(brush.into()),
      .. self
    }
  }

  fn append_corner(path: &mut BezPath, center: Point, radius: f64, angle_index: usize) {
    if radius > 0.0 {
      let radii = Vec2 {
        x: radius,
        y: radius,
      };

      let arc = Arc {
        center,
        radii,
        start_angle: FRAC_PI_2 * angle_index as f64,
        sweep_angle: FRAC_PI_2,
        x_rotation: 0.0,
      };

      for path_el in arc.append_iter(ARC_TOLERANCE) {
        path.push(path_el);
      }
    }
  }
}

impl<T: Data> Widget<T> for Tab<T> {
  fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
    self.inner.event(ctx, event, data, env);
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
              (self.on_click)(data, env);
            }
          }
        }
        _ => (),
      }
    }
  }

  fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
    match event {
      LifeCycle::WidgetAdded => self.selected = (self.is_selected)(data),
      LifeCycle::HotChanged(_) => ctx.request_paint(),
      _ => {},
    }
    self.inner.lifecycle(ctx, event, data, env)
  }

  fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
    self.selected = (self.is_selected)(data);
    self.inner.update(ctx, data, env);
  }

  fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
    bc.debug_check("Tab");

    let border_width = self.border_width
        .or(env.try_get(TAB_BORDER_WIDTH))
        .unwrap_or(0.0);

    let inner_bc = bc.shrink((2.0 * border_width, 2.0 * border_width));
    let inner_size = self.inner.layout(ctx, &inner_bc, data, env);
    let origin = Point::new(border_width, border_width);
    self.inner.set_layout_rect(Rect::from_origin_size(origin, inner_size));

    let total_size = Size::new(
      inner_size.width + 2.0 * border_width,
      inner_size.height + 2.0 * border_width,
    );

    let insets = self.inner.compute_parent_paint_insets(total_size);
    ctx.set_paint_insets(insets);

    total_size
  }

  fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
    let size = paint_ctx.size();

    let corner_radius = self.corner_radius
        .or(env.try_get(TAB_CORNER_RADIUS))
        .unwrap_or(0.0);

    let mut path = BezPath::new();
    path.move_to(Point::new(0.0, size.height));
    path.line_to(Point::new(0.0, corner_radius));
    Self::append_corner(&mut path, Point::new(corner_radius, corner_radius), corner_radius, 2);
    path.line_to(Point::new(size.width - corner_radius, 0.0));
    Self::append_corner(&mut path, Point::new(size.width - corner_radius, corner_radius), corner_radius, 3);
    path.line_to(Point::new(size.width, size.height));
    path.close_path();

    let result = paint_ctx
      .save()
      .and_then(|_status| {
        paint_ctx.clip(&path);
        let (widget_background, env_key) = if self.selected {
          (self.selected_background.as_mut(), TAB_SELECTED_BACKGROUND_COLOR)
        } else if paint_ctx.is_hot() {
          (self.hover_background.as_mut(), TAB_HOVER_BACKGROUND_COLOR)
        } else {
          (self.unselected_background.as_mut(), TAB_UNSELECTED_BACKGROUND_COLOR)
        };
        let mut env_background = env.try_get(env_key)
            .map(|color| BackgroundBrush::Color(color));
        if let Some(background) = widget_background.or(env_background.as_mut()) {
          background.paint(paint_ctx, data, env);
        }
        Ok(())
      })
      .and_then(|_status| paint_ctx.restore())
      .and_then(|_| {
        let border_width = self.border_width
            .or(env.try_get(TAB_BORDER_WIDTH));

        if let Some(border_width) = border_width {
          let (current_border_color, env_key) = if self.selected {
            (self.selected_border_color.as_ref(), TAB_SELECTED_BORDER_COLOR)
          } else if paint_ctx.is_hot() {
            (self.hover_border_color.as_ref(), TAB_HOVER_BORDER_COLOR)
          } else {
            (self.unselected_border_color.as_ref(), TAB_UNSELECTED_BORDER_COLOR)
          };

          if let Some(border_color) = current_border_color.or(env.try_get(env_key).as_ref()) {
            paint_ctx.stroke(&path, border_color, border_width);
          }
        }

        self.inner.paint(paint_ctx, data, env);

        Ok(())
      });

    drop(result);
  }
}

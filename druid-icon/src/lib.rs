use std::marker::PhantomData;
use druid::{
  kurbo::{Affine, BezPath, PathEl, Rect, Size},
  widget::prelude::*,
  Color, Data,
};
use druid::widget::FillStrat;

pub mod prelude {
  pub use druid::kurbo::{Affine, PathEl, Point, Size};
  pub use crate::{IconStaticData, IconStaticPath, IconPathFill, IconPathStroke};
}

#[derive(Debug)]
pub struct IconStaticPath {
  pub transform: Affine,
  pub fill: Option<IconPathFill>,
  pub stroke: Option<IconPathStroke>,
  pub elements: &'static [PathEl]
}

#[derive(Debug)]
pub struct IconStaticData {
  pub paths: &'static [IconStaticPath],
  pub size: Size,
}

#[derive(Debug, Clone, Copy)]
pub struct IconPathFill {
  pub opacity: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct IconPathStroke {
  pub opacity: f64,
  pub width: f64,
}

#[derive(Debug)]
pub struct IconPath {
  pub transform: Affine,
  pub fill: Option<IconPathFill>,
  pub stroke: Option<IconPathStroke>,
  pub bezier_path: BezPath,
}

impl From<&IconStaticPath> for IconPath {
  fn from(static_path: &IconStaticPath) -> Self {
    IconPath {
      transform: static_path.transform,
      fill: static_path.fill,
      stroke: static_path.stroke,
      bezier_path: BezPath::from_vec(static_path.elements.to_vec()),
    }
  }
}

#[derive(Debug)]
pub struct IconData {
  pub paths: Vec<IconPath>,
  pub size: Size,
}

impl From<&IconStaticData> for IconData {
  fn from(static_data: &IconStaticData) -> Self {
    let paths = static_data.paths.iter()
        .map(|static_path| IconPath::from(static_path))
        .collect::<Vec<IconPath>>();

    IconData {
      paths,
      size: static_data.size,
    }
  }
}

pub struct Icon<T: Data> {
  data: IconData,
  fill_strategy: FillStrat,
  color: Color,
  _phantom: PhantomData<T>,
}

impl<T: Data> Icon<T> {
  pub fn new<D: Into<IconData>>(data: D) -> Self {
    Self {
      data: data.into(),
      fill_strategy: FillStrat::None,
      color: Color::BLACK,
      _phantom: PhantomData
    }
  }

  pub fn color(mut self, color: Color) -> Icon<T> {
    self.color = color;
    self
  }

  pub fn fill_strategy(mut self, fill_strategy: FillStrat) -> Icon<T> {
    self.fill_strategy = fill_strategy;
    self
  }
}

impl<T: Data> Widget<T> for Icon<T> {
  fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {
    // no events
  }

  fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {
    // no lifecycle
  }

  fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {
    // no update
  }

  fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
    bc.debug_check("Icon");

    if bc.is_width_bounded() {
      bc.max()
    } else {
      bc.constrain(self.data.size)
    }
  }

  fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, _env: &Env) {
    let offset_matrix = self.fill_strategy.affine_to_fill(ctx.size(), self.data.size);

    let clip_rect = Rect::ZERO.with_size(ctx.size());

    ctx.clip(clip_rect);

    for path in self.data.paths.iter() {
      let bezier_path = path.transform * &path.bezier_path;
      let bezier_path = offset_matrix * bezier_path;
      if let Some(fill) = path.fill.as_ref() {
        let color = self.color.clone().with_alpha(fill.opacity);
        ctx.fill(&bezier_path, &color);
      }
      if let Some(stroke) = path.stroke.as_ref() {
        let color = self.color.clone().with_alpha(stroke.opacity);
        ctx.stroke(&bezier_path, &color, stroke.width);
      }
    }
  }
}

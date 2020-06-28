use std::{error::Error, fs::File, io::Read, path::PathBuf};

use druid::kurbo::{BezPath, Size};
use druid::Affine;
use druid_icon::{IconData, IconPath, IconPathFill, IconPathStroke};
use log::{error, info};
use std::path::Path;

#[derive(Debug)]
pub struct IconFile {
  pub path: PathBuf,
  pub module: PathBuf,
  pub name: String,
}

impl IconFile {
  pub fn with_module<P: AsRef<Path>>(mut self, module: P) -> Self {
    self.module = module.as_ref().to_path_buf();
    self
  }

  pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
    self.name = name.into();
    self
  }

  pub fn load(&self) -> Result<IconData, Box<dyn Error>> {
    info!("Loading icon {} ...", self.path.display());

    let re_opt = usvg::Options {
      keep_named_groups: false,
      ..usvg::Options::default()
    };

    let mut file = File::open(&self.path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    match usvg::Tree::from_str(contents.as_str(), &re_opt) {
      Ok(tree) => Ok(Self::from_tree(tree)),
      Err(err) => Err(err.into()),
    }
  }

  fn from_tree(tree: usvg::Tree) -> IconData {
    let mut paths = Vec::<IconPath>::new();
    for child in tree.root().children() {
      match *child.borrow() {
        usvg::NodeKind::Path(ref svg_path) => {
          let mut bezier_path = BezPath::new();
          for segment in svg_path.data.iter() {
            match *segment {
              usvg::PathSegment::MoveTo { x, y } => {
                bezier_path.move_to((x, y));
              }
              usvg::PathSegment::LineTo { x, y } => {
                bezier_path.line_to((x, y));
              }
              usvg::PathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
              } => {
                bezier_path.curve_to((x1, y1), (x2, y2), (x, y));
              }
              usvg::PathSegment::ClosePath => {
                bezier_path.close_path();
              }
            }
          }
          let transform = Affine::new([
            svg_path.transform.a,
            svg_path.transform.b,
            svg_path.transform.c,
            svg_path.transform.d,
            svg_path.transform.e,
            svg_path.transform.f,
          ]);

          let fill = svg_path.fill.as_ref().map(|fill| IconPathFill {
            opacity: fill.opacity.value(),
          });

          let stroke = svg_path.stroke.as_ref().map(|stroke| IconPathStroke {
            opacity: stroke.opacity.value(),
            width: stroke.width.value(),
          });

          paths.push(IconPath {
            transform,
            fill,
            stroke,
            bezier_path,
          });
        }

        usvg::NodeKind::Defs => {
          // TODO: implement defs
          error!("{:?} is unimplemented", child.clone());
        }

        _ => {
          // TODO: handle more of the SVG spec.
          error!("{:?} is unimplemented", child.clone());
        }
      }
    }

    IconData {
      paths,
      size: Self::get_size_from_svg(tree),
    }
  }

  /// Measure the SVG's size
  #[allow(clippy::needless_return)]
  fn get_size_from_svg(tree: usvg::Tree) -> Size {
    return match *tree.root().borrow() {
      usvg::NodeKind::Svg(svg) => {
        // Borrow checker gets confused without an explicit return
        Size::new(svg.size.width(), svg.size.height())
      }
      _ => {
        // TODO: It doesn't seem reachable?
        error!("This SVG has no size for some reason.");
        Size::ZERO
      }
    };
  }
}

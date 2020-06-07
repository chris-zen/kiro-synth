use std::{
  fs::File,
  io::Write,
  path::Path,
  path::PathBuf,
  error::Error,
};

use heck::SnakeCase;
use druid_icon::prelude::*;
use druid_icon::{IconData, IconPath};

use crate::file::IconFile;


macro_rules! wi {
  ($dst:expr, $ident:expr, $($arg:tt)*) => {
    $dst.write_fmt(format_args!("{:ident$}", "", ident=$ident))?;
    $dst.write_fmt(format_args!($($arg)*))?;
  }
}

macro_rules! wlni {
  ($dst:expr, $ident:expr, $($arg:tt)*) => {
    $dst.write_fmt(format_args!("{:ident$}", "", ident=$ident))?;
    $dst.write_fmt(format_args!($($arg)*))?;
    $dst.write_all("\n".as_bytes())?;
  }
}

macro_rules! wln {
  ($dst:expr, $($arg:tt)*) => {
    $dst.write_fmt(format_args!($($arg)*))?;
    $dst.write_all("\n".as_bytes())?;
  }
}

pub struct Generator {
  base_path: PathBuf,
}

impl Generator {
  pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
    Generator {
      base_path: base_path.as_ref().to_path_buf(),
    }
  }

  pub fn generate(&self, icons: impl Iterator<Item=(IconFile, IconData)>) -> Result<(), Box<dyn Error>>{
    let mut out = File::create(&self.base_path)?;

    wln!(out, "use druid_icon::prelude::*;");
    wln!(out, "");

    for icon in icons {
      icon.generate(&mut out, 0)?;
    }
    Ok(())
  }
}

pub trait Generate {
  fn generate(&self, out: &mut impl Write, ident: usize) -> Result<(), Box<dyn Error>>;
}

impl Generate for (IconFile, IconData) {
  fn generate(&self, out: &mut impl Write, ident: usize) -> Result<(), Box<dyn Error>> {
    let const_name = self.0.name.to_snake_case().to_uppercase();
    wlni!(out, ident, "");
    wlni!(out, ident, "pub const {}: IconStaticData = IconStaticData {{", const_name);
    wlni!(out, ident, "  size: Size::new({:?}, {:?}),", self.1.size.width, self.1.size.height);
    wlni!(out, ident, "  paths: &[");
    for path in self.1.paths.iter() {
      path.generate(out, ident + 4)?;
    }
    wlni!(out, ident, "  ],");
    wlni!(out, ident, "}};");
    Ok(())
  }
}

impl Generate for IconPath {
  fn generate(&self, out: &mut impl Write, ident: usize) -> Result<(), Box<dyn Error>> {
    wlni!(out, ident, "IconStaticPath {{");
    wlni!(out, ident, "  transform: Affine::scale(1.0),");
    wi!(out, ident, "  fill: "); self.fill.generate(out, ident)?; writeln!(out, ",")?;
    wi!(out, ident, "  stroke: "); self.stroke.generate(out, ident)?; writeln!(out, ",")?;
    wlni!(out, ident, "  elements: &[");
    for element in self.bezier_path.iter() {
      element.generate(out, ident + 4)?;
    }
    wlni!(out, ident, "  ],");
    wlni!(out, ident, "}},");
    Ok(())
  }
}

impl<T: Generate> Generate for Option<T> {
  fn generate(&self, out: &mut impl Write, ident: usize) -> Result<(), Box<dyn Error>> {
    match self {
      Some(value) => {
        write!(out, "Some(")?;
        value.generate(out, ident)?;
        write!(out, ")")?;
      },
      None => {
        write!(out, "None")?;
      }
    }
    Ok(())
  }
}

impl Generate for IconPathFill {
  fn generate(&self, out: &mut impl Write, _ident: usize) -> Result<(), Box<dyn Error>> {
    write!(out, "IconPathFill {{ opacity: {:?} }}", self.opacity)?;
    Ok(())
  }
}

impl Generate for IconPathStroke {
  fn generate(&self, out: &mut impl Write, _ident: usize) -> Result<(), Box<dyn Error>> {
    write!(out, "IconPathStroke {{ opacity: {:?}, width: {:?} }}", self.opacity, self.width)?;
    Ok(())
  }
}

impl Generate for PathEl {
  fn generate(&self, out: &mut impl Write, ident: usize) -> Result<(), Box<dyn Error>> {
    match self {
      PathEl::MoveTo(p) => {
        wi!(out, ident, "PathEl::MoveTo(");
        p.generate(out, ident)?;
        wln!(out, "),");
      },
      PathEl::LineTo(p) => {
        wi!(out, ident, "PathEl::LineTo(");
        p.generate(out, ident)?;
        wln!(out, "),");
      },
      PathEl::QuadTo(p1, p2) => {
        wi!(out, ident, "PathEl::QuadTo(");
        p1.generate(out, ident)?;
        write!(out, ", ")?;
        p2.generate(out, ident)?;
        wln!(out, "),");
      },
      PathEl::CurveTo(p1, p2, p3) => {
        wi!(out, ident, "PathEl::CurveTo(");
        p1.generate(out, ident)?;
        write!(out, ", ")?;
        p2.generate(out, ident)?;
        write!(out, ", ")?;
        p3.generate(out, ident)?;
        wln!(out, "),");
      },
      PathEl::ClosePath => {
        wlni!(out, ident, "PathEl::ClosePath,");
      },
    }
    Ok(())
  }
}

impl Generate for Point {
  fn generate(&self, out: &mut impl Write, _ident: usize) -> Result<(), Box<dyn Error>> {
    write!(out, "Point::new({:?}, {:?})", self.x, self.y)?;
    Ok(())
  }
}

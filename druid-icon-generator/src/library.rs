use std::path::{Path, PathBuf};

use log::info;
use walkdir::{WalkDir, DirEntry};

use crate::file::IconFile;


pub struct IconLibrary {
  base_path: PathBuf,
}

impl IconLibrary {
  pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
    IconLibrary {
      base_path: base_path.as_ref().to_path_buf(),
    }
  }

  pub fn iter(&self) -> impl Iterator<Item=IconFile> {
    info!("Traversing icons at {} ...", self.base_path.display());
    let base_path = self.base_path.clone();
    WalkDir::new(base_path.clone()).into_iter()
      .filter_map(Result::ok)
      .filter_map(move |entry: DirEntry| {
        let base_path = base_path.clone();
        let path = entry.path().to_path_buf();
        path.clone().strip_prefix(base_path).ok()
          .and_then(Path::parent)
          .and_then(move |module| {
            entry.file_name().to_os_string().into_string().ok().map(|name| {
              IconFile {
                path,
                module: module.to_path_buf(),
                name,
              }
            })
        })
      })
  }
}

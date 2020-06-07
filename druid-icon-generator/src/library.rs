use std::{
  fs,
  path::PathBuf,
};

use log::info;

use crate::file::IconFile;
use std::path::Path;


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
    fs::read_dir(base_path.clone()).unwrap()
        .filter_map(move |result_entry| {
          let base_path = base_path.clone();
          result_entry.ok().and_then(move |entry| {
            let path = entry.path();
            path.clone().strip_prefix(base_path).ok().and_then(move |module| {
              entry.file_name().into_string().ok().map(|name| {
                IconFile {
                  path,
                  module: module.to_path_buf(),
                  name,
                }
              })
            })
          })
        })
  }
}

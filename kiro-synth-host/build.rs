use druid_icon_generator::file::IconFile;
use druid_icon_generator::generator::Generator;
use druid_icon_generator::library::IconLibrary;
use regex::Regex;

fn main() {
  let icon_name_regex = Regex::new(r"^(.*)\.svg$").unwrap();

  let icons = IconLibrary::new("art/icons")
    .iter()
    .filter_map(|icon_file| {
      icon_name_regex
        .captures(icon_file.name.clone().as_str())
        .map(|captures| IconFile {
          name: captures.get(1).unwrap().as_str().to_string(),
          ..icon_file
        })
    })
    .filter_map(|icon_file| {
      icon_file
        .load()
        .ok()
        .map(|icon_data| (icon_file, icon_data))
    });

  Generator::new("src/ui/icons/mod.rs")
    .generate(icons)
    .unwrap();
}

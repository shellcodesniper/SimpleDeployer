use std::path::Path;

#[allow(dead_code)]
pub fn check_is_dir(path: &Path) -> bool {
  path.is_dir()
}

pub fn check_str_file_exist(path: String) -> bool {
  let path = Path::new(&path);
  path.is_file()
}

pub fn mkdir_if_none_exist(path: &Path) {
  if path.is_dir() || path.is_file() {
    return;
  }
  std::fs::create_dir_all(path).unwrap();
}

use std::path::Path;

use minifier::css::minify;

fn main() {
  let out_dir = std::env::var_os("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("style.css");

  let input = std::fs::read_to_string("assets/style.css").unwrap();
  let output = minify(&input).unwrap();

  std::fs::write(&dest_path, output.to_string()).unwrap();

  println!("cargo:rerun-if-changed=assets/style.css");
}

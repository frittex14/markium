use std::path::Path;

use minifier::{css, js};

fn main() {
  let out_dir = std::env::var_os("OUT_DIR").unwrap();

  // Minify base CSS
  let css_dest_path = Path::new(&out_dir).join("style.css");
  let css_input = std::fs::read_to_string("assets/style.css").unwrap();
  let css_output = css::minify(&css_input).unwrap();
  std::fs::write(&css_dest_path, css_output.to_string()).unwrap();
  println!("cargo:rerun-if-changed=assets/style.css");

  // Minify the Starry Night theme
  let sn_dest_path = Path::new(&out_dir).join("starry-night.css");
  let sn_input = std::fs::read_to_string("assets/starry-night.css").unwrap();
  let sn_output = css::minify(&sn_input).unwrap();
  std::fs::write(&sn_dest_path, sn_output.to_string()).unwrap();
  println!("cargo:rerun-if-changed=assets/starry-night.css");

  // Minify JS
  let js_dest_path = Path::new(&out_dir).join("main.mjs");
  let js_input = std::fs::read_to_string("assets/main.js").unwrap();
  let js_output = js::minify(&js_input);
  std::fs::write(&js_dest_path, js_output.to_string()).unwrap();
  println!("cargo:rerun-if-changed=assets/main.js");
}

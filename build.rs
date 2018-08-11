extern crate webgl_generator;

use std::env;
use std::fs::File;
use std::path::Path;
use webgl_generator::*;

fn main() {
    let directory = env::var("OUT_DIR").unwrap();
    let path = Path::new(&directory).join("webgl.rs");
    let mut file = File::create(&path).unwrap();

    Registry::new(Api::WebGl2, Exts::NONE)
        .write_bindings(StdwebGenerator, &mut file)
        .unwrap();
}

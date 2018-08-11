#![allow(dead_code, unused_parens, unused_imports)]

// Include webgl bindings generated in build.rs script.
include!(concat!(env!("OUT_DIR"), "/webgl.rs"));

#[cfg(test)]
mod test;

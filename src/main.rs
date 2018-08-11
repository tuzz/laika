extern crate rand;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate stdweb;
#[macro_use] extern crate stdweb_derive;

#[cfg(test)]
#[macro_use] extern crate assert_approx_eq;

mod model;
mod view;

fn main() {
    println!("Hello, world!");
}

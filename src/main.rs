extern crate rand;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate stdweb;
#[macro_use] extern crate stdweb_derive;

#[cfg(test)]
#[macro_use] extern crate assert_approx_eq;

mod model;
mod view;
mod controller;

use self::model::Model;
use self::view::View;
use self::controller::Controller;

fn main() {
    let model = Model::new();
    let view = View::new();
    let controller = Controller::new(model, view);

    controller.handle_events();
}

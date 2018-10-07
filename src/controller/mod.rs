use super::model::Model;
use super::view::View;

pub struct Controller {
    model: Model,
    view: View,
}

impl Controller {
    pub fn new(model: Model, view: View) -> Self {
        Self { model, view }
    }

    pub fn handle_events(mut self) {
        View::on_frame(move |delta, elapsed| self.render_frame(delta, elapsed));
    }

    fn render_frame(&mut self, delta: f64, elapsed: f64) {
        self.model.advance(delta, elapsed);

        self.view.clear();
        self.view.render(&self.model);
    }
}

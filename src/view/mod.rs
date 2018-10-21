mod buffer;
mod program;
mod scene;
mod shader;
mod viewport;
mod webgl;
mod webpage;

use super::model::Model;

use self::scene::Scene;
use self::viewport::Viewport;
use self::webpage::Webpage;

pub struct View {
    webpage: Webpage,
    viewport: Viewport,
    scene: Scene,
}

impl View {
    pub fn new() -> Self {
        let webpage = Webpage::new("laika");
        let viewport = Viewport::new(0.0, 0.0, 0.0, 1.0);
        let scene = Scene::new(&webpage.context);

        Self { webpage, viewport, scene }
    }

    pub fn clear(&self) {
        self.viewport.clear(&self.webpage.context);
    }

    pub fn render(&self, model: &Model) {
        self.scene.render(model, &self.webpage.context);
    }

    pub fn on_frame<F: FnMut(f64, f64) + 'static>(callback: F) {
        Webpage::on_frame(callback);
    }
}

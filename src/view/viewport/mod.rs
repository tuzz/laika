use super::webgl::WebGLRenderingContext as GL;

pub struct Viewport {
    clear_color: (f32, f32, f32, f32),
}

impl Viewport {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self { clear_color: (red, green, blue, alpha) }
    }

    pub fn clear(&self, context: &GL) {
        let width = context.canvas().width();
        let height = context.canvas().height();

        let (r, g, b, a) = self.clear_color;

        context.viewport(0, 0, width as i32, height as i32);
        context.clear_color(r, g, b, a);
        context.clear(GL::COLOR_BUFFER_BIT);

        context.enable(GL::BLEND);
        context.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    }
}

use ::model::Model;

use super::buffer::Buffer;
use super::program::Program;
use super::webgl::WebGLRenderingContext as GL;

pub struct Scene {
    identity: [f32; 9],
    program: Program,
    positions: Buffer,
}

impl Scene {
    pub fn new(context: &GL) -> Self {
        let identity = [1.,0.,0.,0.,1.,0.,0.,0.,1.];
        let program = Program::default(context);
        let positions = Buffer::new(context, &[-0.5, 0.0, 0.0, 0.8, 0.7, -0.5]);

        program.enable(context);

        Self { identity, program, positions }
    }

    pub fn render(&self, _model: &Model, context: &GL) {
        self.program.set_attribute_from_buffer(context, "a_position", &self.positions, 2);
        self.program.set_uniform_from_slice(context, "u_matrix", &self.identity);

        context.draw_arrays(GL::TRIANGLES, 0, 3);
    }
}

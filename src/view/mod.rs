mod buffer;
mod program;
mod shader;
mod viewport;
mod webgl;
mod webpage;

use self::buffer::Buffer;
use self::program::Program;
use self::viewport::Viewport;
use self::webpage::Webpage;

pub struct View {

}

use self::webgl::WebGLRenderingContext as GL;

impl View {
    pub fn new() -> Self {
        let webpage = Webpage::new("laika");
        let context = webpage.context;
        let viewport = Viewport::new(1.0, 1.0, 1.0, 1.0);
        let identity = [1.,0.,0.,0.,1.,0.,0.,0.,1.];

        Webpage::animate(move |_delta, _elapsed| {
            let program = Program::default(&context);
            let positions = Buffer::new(&context, &[-0.5, 0.0, 0.0, 0.8, 0.7, -0.5]);

            viewport.clear(&context);

            program.enable(&context);
            program.set_attribute_from_buffer(&context, "a_position", &positions, 2);
            program.set_uniform_from_slice(&context, "u_matrix", &identity);

            context.draw_arrays(GL::TRIANGLES, 0, 3);
        });

        View{}
    }
}

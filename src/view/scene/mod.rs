use ::model::Model;
use ::util::Circle;
use ::util::Matrix;

use super::buffer::Buffer;
use super::program::Program;
use super::webgl::WebGLRenderingContext as GL;

use std::f32::consts::PI;

pub struct Scene {
    program: Program,
    circle_points: Buffer,
}

impl Scene {
    pub fn new(context: &GL) -> Self {
        let program = Program::default(context);
        let circle_points = Self::circle_points(100, context);

        program.enable(context);

        Self { program, circle_points }
    }

    pub fn render(&self, model: &Model, context: &GL) {
        for planet in &model.galaxy.planets {
            self.render_circle(&planet.mass, context);
        }
    }

    fn render_circle(&self, circle: &Circle, context: &GL) {
        let buffer = &self.circle_points;
        let matrix = Matrix::identity()
            .scale(circle.radius, circle.radius)
            .translate(circle.center.x, circle.center.y)
            .project();

        self.program.set_attribute_from_buffer(context, "a_position", buffer, 2);
        self.program.set_uniform_from_slice(context, "u_matrix", &matrix.array);

        context.draw_arrays(GL::TRIANGLE_FAN, 0, buffer.len(2));
    }

    fn circle_points(number_of_points: usize, context: &GL) -> Buffer {
        let mut points = vec![0.0, 0.0];

        for i in 0..number_of_points {
            let ratio = i as f32 / (number_of_points - 1) as f32;
            let radians = ratio * 2.0 * PI;

            points.push(radians.cos());
            points.push(radians.sin());
        }

        Buffer::new(context, &points[..])
    }
}

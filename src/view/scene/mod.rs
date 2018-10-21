use ::util::{Circle, Color, Matrix};

use ::model::Model;

use super::buffer::Buffer;
use super::program::Program;
use super::webgl::WebGLRenderingContext as GL;

use std::f32::consts::PI;

pub struct Scene {
    program: Program,
    circle_points: Buffer,
}

pub const NUMBER_OF_POINTS: usize = 50;

impl Scene {
    pub fn new(context: &GL) -> Self {
        let program = Program::default(context);
        let circle_points = Self::circle_points(NUMBER_OF_POINTS, context);

        program.enable(context);

        Self { program, circle_points }
    }

    pub fn render(&self, model: &Model, context: &GL) {
        for planet in &model.galaxy.planets {
            let colors = planet.colors();

            self.render_circle(&planet.zone, &colors, 0.1, context);
            self.render_circle(&planet.mass, &colors, 1.0, context);
        }
    }

    fn render_circle(&self, circle: &Circle, colors: &[&Color], alpha: f32, context: &GL) {
        let point_buffer = &self.circle_points;
        let color_buffer = &Self::color_buffer(colors, alpha, context);

        self.program.set_attribute_from_buffer(context, "a_color", color_buffer, 4);
        self.program.set_attribute_from_buffer(context, "a_position", point_buffer, 2);

        let matrix = Matrix::identity()
            .scale(circle.radius, circle.radius)
            .translate(circle.center.x, circle.center.y)
            .project();

        self.program.set_uniform_from_slice(context, "u_matrix", &matrix.array);

        context.draw_arrays(GL::TRIANGLE_FAN, 0, point_buffer.len(2));
    }

    fn circle_points(number_of_points: usize, context: &GL) -> Buffer {
        let mut points = vec![0.0, 0.0];

        for i in 0..(number_of_points - 1) {
            let ratio = i as f32 / (number_of_points - 2) as f32;
            let radians = ratio * 2.0 * PI;

            points.push(radians.cos());
            points.push(radians.sin());
        }

        Buffer::new(context, &points[..])
    }

    fn color_buffer(colors: &[&Color], alpha: f32, context: &GL) -> Buffer {
        let mut values = vec![];

        for color in colors {
            values.push(color.red);
            values.push(color.green);
            values.push(color.blue);
            values.push(alpha);
        }

        Buffer::new(context, &values[..])
    }
}

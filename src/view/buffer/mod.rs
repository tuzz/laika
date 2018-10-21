use stdweb::web::TypedArray;

use super::webgl::{WebGLBuffer, WebGLRenderingContext as GL};

pub struct Buffer {
    buffer: WebGLBuffer,
    len: usize,
}

impl Buffer {
    pub fn new(context: &GL, data: &[f32]) -> Self {
        let buffer = context.create_buffer().expect("failed to create buffer");
        let vertices = TypedArray::from(data).buffer();

        context.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        context.buffer_data_1(GL::ARRAY_BUFFER, Some(&vertices), GL::STATIC_DRAW);

        Buffer { buffer, len: data.len() }
    }

    pub fn bind(&self, context: &GL) {
        context.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer));
    }

    pub fn len(&self, elements_per_point: usize) -> i32 {
        (self.len / elements_per_point) as i32
    }
}

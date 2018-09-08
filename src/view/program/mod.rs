use std::collections::HashMap;

use super::buffer::Buffer;
use super::shader::{Shader, fragment::FragmentShader, vertex::VertexShader};
use super::webgl::{WebGLRenderingContext as GL, WebGLProgram, WebGLUniformLocation};

type WebGLAttributeLocation = u32;

type Attributes = HashMap<String, WebGLAttributeLocation>;
type Uniforms = HashMap<String, WebGLUniformLocation>;

pub struct Program {
    pub compiled: WebGLProgram,
    pub attributes: Attributes,
    pub uniforms: Uniforms,
}

impl Program {
    pub fn new(context: &GL, frag: &FragmentShader, vert: &VertexShader) -> Self {
        let program = context.create_program().expect("failed to create program");

        context.attach_shader(&program, &frag.compiled);
        context.attach_shader(&program, &vert.compiled);
        context.link_program(&program);

        let attributes = Self::attributes(&program, context, vert);
        let uniforms = Self::uniforms(&program, context, vert);

        Self { compiled: program, attributes, uniforms }
    }

    pub fn default(context: &GL) -> Self {
        let frag = FragmentShader::default(context);
        let vert = VertexShader::default(context);

        Self::new(context, &frag, &vert)
    }

    pub fn enable(&self, context: &GL) {
        context.use_program(Some(&self.compiled));
    }

    pub fn set_attribute_from_buffer(&self, context: &GL, name: &str, buffer: &Buffer, slice: i32) {
        let location = self.attribute(name);

        context.enable_vertex_attrib_array(location);
        buffer.bind(context);
        context.vertex_attrib_pointer(location, slice, GL::FLOAT, false, 0, 0);
    }

    pub fn set_uniform_from_slice(&self, context: &GL, name: &str, slice: &[f32]) {
        let location = self.uniform(name);

        context.uniform_matrix3fv(Some(&location), false, slice);
    }

    fn attributes(program: &WebGLProgram, context: &GL, vert: &VertexShader) -> Attributes {
        vert.attributes.iter().map(|name|
            (name.to_owned(), context.get_attrib_location(&program, name) as WebGLAttributeLocation)
        ).collect()
    }

    fn uniforms(program: &WebGLProgram, context: &GL, vert: &VertexShader) -> Uniforms {
        vert.uniforms.iter().map(|name|
            (name.to_owned(), context.get_uniform_location(&program, name).unwrap())
        ).collect()
    }

    fn attribute(&self, name: &str) -> WebGLAttributeLocation {
        self.attributes.get(name).expect("failed to get attribute").to_owned()
    }

    fn uniform(&self, name: &str) -> WebGLUniformLocation {
        self.uniforms.get(name).expect("failed to get uniform").to_owned()
    }
}

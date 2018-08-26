use std::collections::HashMap;

use super::shader::Shader;
use super::shader::fragment::FragmentShader;
use super::shader::vertex::VertexShader;

use super::webgl::{WebGLRenderingContext, WebGLProgram, WebGLUniformLocation};

type Attributes = HashMap<String, i32>;
type Uniforms = HashMap<String, WebGLUniformLocation>;

pub struct Program {
    pub compiled: WebGLProgram,
    pub attributes: Attributes,
    pub uniforms: Uniforms,
}

impl Program {
    pub fn new(context: &WebGLRenderingContext, frag: &FragmentShader, vert: &VertexShader) -> Self {
        let program = context.create_program().expect("failed to create program");

        context.attach_shader(&program, &frag.compiled);
        context.attach_shader(&program, &vert.compiled);
        context.link_program(&program);

        let attributes = Self::attributes(&program, context, vert);
        let uniforms = Self::uniforms(&program, context, vert);

        Self { compiled: program, attributes, uniforms }
    }

    pub fn default(context: &WebGLRenderingContext) -> Self {
        let frag = FragmentShader::default(context);
        let vert = VertexShader::default(context);

        Self::new(context, &frag, &vert)
    }

    fn attributes(program: &WebGLProgram, context: &WebGLRenderingContext, vert: &VertexShader) -> Attributes {
        vert.attributes.iter().map(|name|
            (name.to_owned(), context.get_attrib_location(&program, name))
        ).collect()
    }

    fn uniforms(program: &WebGLProgram, context: &WebGLRenderingContext, vert: &VertexShader) -> Uniforms {
        vert.uniforms.iter().map(|name|
            (name.to_owned(), context.get_uniform_location(&program, name).unwrap())
        ).collect()
    }
}

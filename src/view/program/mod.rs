use super::shader::Shader;
use super::shader::fragment::FragmentShader;
use super::shader::vertex::VertexShader;

use super::webgl::{WebGLRenderingContext, WebGLProgram};

pub struct Program {
    _compiled: WebGLProgram,
}

impl Program {
    pub fn new(context: &WebGLRenderingContext, frag: &FragmentShader, vert: &VertexShader) -> Self {
        let program = context.create_program().expect("failed to create program");

        context.attach_shader(&program, &frag.compiled);
        context.attach_shader(&program, &vert.compiled);
        context.link_program(&program);

        Self { _compiled: program }
    }

    pub fn default(context: &WebGLRenderingContext) -> Self {
        let frag = FragmentShader::default(context);
        let vert = VertexShader::default(context);

        Self::new(context, &frag, &vert)
    }
}

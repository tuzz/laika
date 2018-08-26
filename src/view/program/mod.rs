use super::shader::Shader;
use super::webgl::{WebGLRenderingContext, WebGLProgram};

pub struct Program {
    _compiled: WebGLProgram,
}

impl Program {
    pub fn new(context: &WebGLRenderingContext) -> Self {
        let program = context.create_program().expect("failed to create program");

        let vert_shader = Shader::vertex(&context);
        let frag_shader = Shader::fragment(&context);

        context.attach_shader(&program, &vert_shader.compiled);
        context.attach_shader(&program, &frag_shader.compiled);
        context.link_program(&program);

        Self { _compiled: program }
    }
}

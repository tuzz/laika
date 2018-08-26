use super::webgl::{WebGLRenderingContext, WebGLShader};

pub struct Shader {
    pub compiled: WebGLShader,
}

impl Shader {
    fn new(context: &WebGLRenderingContext, kind: u32, source: &str) -> Self {
        let shader = context.create_shader(kind).expect("failed to create shader");

        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        Shader { compiled: shader }
    }

    pub fn vertex(context: &WebGLRenderingContext) -> Self {
        let kind = WebGLRenderingContext::VERTEX_SHADER;
        let source = include_str!("shader.vert");

        Self::new(context, kind, source)
    }

    pub fn fragment(context: &WebGLRenderingContext) -> Self {
        let kind = WebGLRenderingContext::FRAGMENT_SHADER;
        let source = include_str!("shader.frag");

        Self::new(context, kind, source)
    }
}

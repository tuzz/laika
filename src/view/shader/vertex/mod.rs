use super::*;

pub struct VertexShader {
    pub compiled: WebGLShader,
}

impl Shader for VertexShader {
    fn new(context: &GL, source: &str) -> Self {
        Self { compiled: Self::compile(context, GL::VERTEX_SHADER, source) }
    }

    fn default(context: &GL) -> Self {
        Self::new(context, include_str!("default.vert"))
    }
}

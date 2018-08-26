use super::*;

pub struct FragmentShader {
    pub compiled: WebGLShader,
}

impl Shader for FragmentShader {
    fn new(context: &GL, source: &str) -> Self {
        Self { compiled: Self::compile(context, GL::FRAGMENT_SHADER, source) }
    }

    fn default(context: &GL) -> Self {
        Self::new(context, include_str!("default.frag"))
    }
}

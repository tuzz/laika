use super::*;

pub struct FragmentShader {
    pub compiled: WebGLShader,
}

impl FragmentShader {
    fn new(context: &GL, source: &str) -> Self {
        Self { compiled: Self::compile(context, GL::FRAGMENT_SHADER, source) }
    }
}

impl Shader for FragmentShader {
    fn default(context: &GL) -> Self {
        Self::new(context, include_str!("default.frag"))
    }
}

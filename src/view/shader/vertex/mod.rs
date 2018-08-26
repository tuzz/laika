use super::*;

pub struct VertexShader {
    pub compiled: WebGLShader,
    pub attributes: Vec<String>,
    pub uniforms: Vec<String>,
}

impl VertexShader {
    fn new(context: &GL, source: &str, attributes: &[String], uniforms: &[String]) -> Self {
        let compiled = Self::compile(context, GL::VERTEX_SHADER, source);
        let attributes = attributes.to_owned();
        let uniforms = uniforms.to_owned();

        Self { compiled, attributes, uniforms }
    }
}

impl Shader for VertexShader {
    fn default(context: &GL) -> Self {
        let attributes = vec!["a_position".to_string(), "a_color".to_string()];
        let uniforms = vec!["u_matrix".to_string()];

        Self::new(context, include_str!("default.vert"), &attributes, &uniforms)
    }
}

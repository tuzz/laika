pub mod fragment;
pub mod vertex;

use super::webgl::{WebGLRenderingContext as GL, WebGLShader};

pub trait Shader {
    fn default(context: &GL) -> Self;

    fn compile(context: &GL, kind: u32, source: &str) -> WebGLShader {
        let shader = context.create_shader(kind).expect("failed to create shader");

        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        shader
    }
}

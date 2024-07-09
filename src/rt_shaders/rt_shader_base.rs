/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines shaders
/// =====================================================

use crate::rt_shader_globals::*;
use crate::rt_types::*;
use crate::rt_scene::*;


pub trait RtShader: Send + Sync {
    // We need a cloning function on this trait for the XML scene parsing
    fn clone_dyn(&self) -> Box<dyn RtShader>;

    // fn Init(&mut self, scene: &mut RenderScene);
    // fn Update(&mut self, scene: &mut RtRenderScene);
    fn evaluate(&self, scene: &RtScene, sg: &RtShaderGlobals) -> RtRGBA;
}

impl Clone for Box<dyn RtShader> {
    fn clone(&self) -> Box<dyn RtShader> {
        self.clone_dyn()
    }
}

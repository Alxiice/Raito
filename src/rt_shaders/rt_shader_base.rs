/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines shaders
/// =====================================================

use crate::rt_shader_globals::*;
use crate::rt_types::*;
use crate::rt_scene::*;


pub trait RtShader {
    // fn Init(&mut self, scene: &mut RenderScene);
    // fn Update(&mut self, scene: &mut RtRenderScene);
    fn evaluate(&self, scene: &RtScene, sg: &RtShaderGlobals) -> RtRGBA;
}

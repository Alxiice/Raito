/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   StateVector shader
/// =====================================================

use crate::rt_shaders::rt_shader_base::*;
use crate::rt_shader_globals::*;
use crate::rt_types::*;
use crate::rt_scene::*;


// ========================================
//  Shader structure
// ========================================

pub struct LightShader {
    pub color: RtRGBA,
    pub intensity: f32
}


// ========================================
//  Shader implementation
// ========================================

impl RtShader for LightShader {
    fn evaluate(&self, scene: &RtScene, sg: &RtShaderGlobals) -> RtRGBA {
        self.color * self.intensity
    }
}

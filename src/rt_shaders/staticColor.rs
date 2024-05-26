/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Static color shader
/// =====================================================

use crate::rt_shaders::rt_shader_base::*;
use crate::rt_shader_globals::*;
use crate::rt_types::*;
use crate::rt_scene::*;


// ========================================
//  Shader structure
// ========================================

pub struct StaticColorShader {
    pub color: RtRGBA
}


// ========================================
//  Shader implementation
// ========================================

impl RtShader for StaticColorShader {
    fn evaluate(&self, scene: &RtScene, sg: &RtShaderGlobals) -> RtRGBA {
        self.color
    }
}

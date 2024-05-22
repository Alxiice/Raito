/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   StateVector shader
/// =====================================================

use crate::rt_shaders::rt_shader_base::*;
use crate::rt_shader_globals::*;
use crate::rt_types::*;


// ========================================
//  Shader structure
// ========================================

pub struct StateVectorShader {
    pub output: String
}


// ========================================
//  Shader implementation
// ========================================

impl RtShader for StateVectorShader {
    fn evaluate(&self, sg: &RtShaderGlobals) -> RtRGBA {
        // TODO : Switch depending on the value of self.output
        // - N : sg.N
        // - P : sg.P
        // ...

        // N
        let mut normal = RtRGBA::default();
        // From [-1; 1] to [0; 256]
        normal.r = (128.0 * (1.0 + sg.N.x)) as u8;
        normal.g = (128.0 * (1.0 + sg.N.y)) as u8;
        normal.b = (128.0 * (1.0 + sg.N.z)) as u8;

        normal
    }
}

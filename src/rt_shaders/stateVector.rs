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

#[derive(Clone, Debug)]
pub struct StateVectorShader {
    pub output: String
}


// ========================================
//  Shader implementation
// ========================================

impl RtShader for StateVectorShader {
    fn clone_dyn(&self) -> Box<dyn RtShader> {
        Box::new(self.clone())
    }
    
    fn evaluate(&self, _scene: &RtScene, sg: &RtShaderGlobals) -> RtRGBA {
        // TODO : Switch depending on the value of self.output
        // - N : sg.N
        // - P : sg.P
        // ...

        // N
        // From [-1; 1] to [0; 1]
        RtRGBA::from_rgb(
            0.5 * (1.0 + sg.N.x), 
            0.5 * (1.0 + sg.N.y), 
            0.5 * (1.0 + sg.N.z)
        )
    }
}

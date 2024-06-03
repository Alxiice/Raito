use log::warn;

/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Lambert shader
/// =====================================================

use crate::rt_shaders::rt_shader_base::*;
use crate::rt_shader_globals::*;
use crate::rt_types::*;
use crate::rt_ray::*;
use crate::rt_scene::*;
use crate::rt_render::*;

const MAX_RAY_LENGTH: f32 = 10000.0;


// ========================================
//  Shader structure
// ========================================

pub struct LambertShader {
    pub color: RtRGBA
}


// ========================================
//  Shader implementation
// ========================================

impl RtShader for LambertShader {
    fn evaluate(&self, scene: &RtScene, sg: &RtShaderGlobals) -> RtRGBA {
        // Create ray
        let mut ray = RtMakeRay(sg, RtRayType::RT_RAY_UNKNOWN, RtVec3::default(), MAX_RAY_LENGTH);
        // Reflect the ray
        RtReflectRay(&mut ray, &sg.ray_dir, &sg.N, &sg);
        
        // Result
        let mut diffuse = RtRGBA::BLACK;

        // Base color
        diffuse += self.color * 0.1;

        // Trace rays
        let hit = RtTraceToLights(scene, &ray);
        if hit.is_some() {
            let hit = hit.unwrap();
            // return hit.colorOutput / (hit.P - sg.P).length();
            // return hit.colorOutput * (hit.P - sg.P).length().powf(2.0)
            // diffuse += albedo * sg->Li * sg->we * AI_ONEOVERPI * max(0, LdotN);
            diffuse += self.color * hit.colorOutput * (1.0 / (hit.P - sg.P).length());

        }
        diffuse
    }
}

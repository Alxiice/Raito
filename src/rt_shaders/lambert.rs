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
const NB_SAMPLES: u8 = 1;

// ========================================
//  Shader structure
// ========================================

pub struct LambertShader {
    pub color: RtRGBA
}


// ========================================
//  Utility functions
// ========================================

fn sample_in_hemisphere(n: RtVec3) -> RtVec3 {
    // Take a random point
    let mut vec = RtVec3::random_range(-1.0, 1.0);
    while vec.length_squared() >= 1.0 {
        vec = RtVec3::random_range(-1.0, 1.0);
    }
    vec = (n + vec).normalize();
    // Make sure the sampled ray is in the correct hemisphere
    if RtVec3::dot(vec, n) > 0.0 { vec } else { -vec }
}

// ========================================
//  Shader implementation
// ========================================

impl RtShader for LambertShader {
    fn evaluate(&self, scene: &RtScene, sg: &RtShaderGlobals) -> RtRGBA {
        // Result
        let mut out_color = RtRGBA::BLACK;
        
        // Diffuse contribution
        let mut diffuse = RtRGBA::BLACK;
        for _ in 0..NB_SAMPLES {
            // Create ray
            let mut ray = RtMakeRay(sg, RtRayType::RT_RAY_UNKNOWN, RtVec3::default(), MAX_RAY_LENGTH);
            ray.origin = ray.origin + RT_EPSILON * sg.N;  // Avoid self intersections
            ray.dir = sample_in_hemisphere(sg.N);
    
            // Trace rays
            // let hit = RtTraceToLights(scene, &ray);
            let hit = RtTraceRay(scene, &ray);
            if hit.is_some() {
                let hit = hit.unwrap();
                // return hit.colorOutput / (hit.P - sg.P).length();
                // return hit.colorOutput * (hit.P - sg.P).length().powf(2.0)
                // diffuse += albedo * sg->Li * sg->we * AI_ONEOVERPI * max(0, LdotN);
                // Ray distance : (hit.P - sg.P).length_squared()
                diffuse += self.color * hit.colorOutput * RT_ONEOVERPI * RtVec3::dot(ray.dir, sg.N);
            } else {
                diffuse += self.color * RtRGBA::ERRCOLOR * RT_ONEOVERPI * RtVec3::dot(ray.dir, sg.N);
            }
        }
        out_color += diffuse / (NB_SAMPLES as f32);

        out_color
    }
}

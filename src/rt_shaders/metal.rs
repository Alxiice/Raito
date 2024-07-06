/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Metal shader
/// =====================================================

use crate::rt_shaders::rt_shader_base::*;
use crate::rt_shader_globals::*;
use crate::rt_types::*;
use crate::rt_ray::*;
use crate::rt_scene::*;
use crate::rt_render::*;

const MAX_RAY_LENGTH: f32 = 10000.0;
const NB_SAMPLES: u8 = 10;

// ========================================
//  Shader structure
// ========================================

#[derive(Clone, Debug)]
pub struct Metal {
    pub color: RtRGBA,
    pub fuzz: f32
}


// ========================================
//  Utility functions
// ========================================

fn random_unit_vector() -> RtVec3 {
    // Take a random point
    let mut vec = RtVec3::random_range(-1.0, 1.0);
    while vec.length_squared() >= 1.0 {
        vec = RtVec3::random_range(-1.0, 1.0);
    }
    vec
}

// ========================================
//  Shader implementation
// ========================================

impl RtShader for Metal {
    fn clone_dyn(&self) -> Box<dyn RtShader> {
        Box::new(self.clone())
    }
    
    fn evaluate(&self, scene: &RtScene, sg: &RtShaderGlobals) -> RtRGBA {
        // Result
        let mut out_color = RtRGBA::BLACK;
        
        let mut ray = RtMakeRay(sg, RtRayType::RT_RAY_UNKNOWN, RtVec3::default(), MAX_RAY_LENGTH);
        ray.origin = ray.origin + RT_EPSILON * sg.N;  // Avoid self intersections
        // Reflect the ray
        RtReflectRay(&mut ray, &sg.ray_dir, &sg.N, &sg);
        ray.dir = ray.dir + self.fuzz * random_unit_vector();

        // Trace rays
        let hit = RtTraceRay(scene, &ray);
        if hit.is_some() {
            let hit = hit.unwrap();
            out_color += self.color * hit.colorOutput;
        }

        out_color
    }
}

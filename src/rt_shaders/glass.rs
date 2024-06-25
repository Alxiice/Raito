use crate::random_float;
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

pub struct Glass {
    pub ior: f32
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

fn reflectance(cosine: f32, ior: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ior) / (1.0 + ior);
    r0 + (1.0 - r0*r0) * (1.0 - cosine).powf(5.0)
}

// ========================================
//  Shader implementation
// ========================================

impl RtShader for Glass {
    fn evaluate(&self, scene: &RtScene, sg: &RtShaderGlobals) -> RtRGBA {
        // Result
        let mut out_color = RtRGBA::BLACK;

        let front_face = RtVec3::dot(sg.ray_dir, sg.N) < 0.0;
        
        let mut ray = RtMakeRay(sg, RtRayType::RT_RAY_UNKNOWN, RtVec3::default(), MAX_RAY_LENGTH);
        // Reflect the ray
        let eta = if front_face { 1.0 / self.ior } else { self.ior };
        let Nf = if front_face { sg.N } else { -sg.N };

        let unit_direction = sg.ray_dir;
        let cos_theta: f32 = RtVec3::dot(-unit_direction, Nf).min(1.0);
        let sin_theta: f32 = (1.0 - cos_theta * cos_theta).sqrt();

        let tir = (eta * sin_theta) > 1.0;

        if tir || reflectance(cos_theta, eta) > random_float() {
            RtReflectRay(&mut ray, &unit_direction, &Nf, &sg);
        } else {
            RtRefractRay(&mut ray, &unit_direction, &Nf, eta, &sg);
        }

        // Avoid self intersections
        ray.origin = ray.origin + RT_EPSILON * ray.dir;

        // Trace rays
        let hit = RtTraceRay(scene, &ray);
        if hit.is_some() {
            out_color += hit.unwrap().colorOutput;
        }

        out_color
    }
}

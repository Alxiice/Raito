/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Implements the ray tracing function.
/// =====================================================

use std::f32::NAN;

use crate::rt_ray::*;
use crate::rt_shader_globals::*;
use crate::rt_scene::*;
use crate::RtVec3;
use crate::rt_objects::rt_object_base::*;


// ========================================
//  Creating rays
// ========================================

/// Utility function to form a new ray from a shading point
pub fn RtMakeRay(sg: &RtShaderGlobals, raytype: RtRayType, dir: RtVec3, maxdist: f32) -> RtRay {
    RtRay { 
        origin: sg.P, 
        dir,
        bounces: sg.bounces + 1, 
        x: sg.x, 
        y: sg.y
    }
}

/// Launch a ray on a scene
pub fn RtReflectRay(ray: &mut RtRay, wo: &RtVec3, normal: &RtVec3, sg: &RtShaderGlobals) {
    ray.dir = *wo - 2.0 * RtVec3::dot(*wo, *normal) * *normal;
}


// ========================================
//  Launching rays
// ========================================

/// Launch a ray on a scene
pub fn RtTraceRay(scene: &mut RtScene, ray: &RtRay) -> Option<RtHit> {
    let objects = scene.get_scene_objects();

    let mut min_dist: f32 = NAN;
    let mut first_hit: Option<RtRayHit> = None;
    let mut first_hit_object: Option<&dyn RtObject> = None;

    // Find closest hit point & object
    for object in objects {
        // Compute intersections
        let hit = object.get_intersection(ray);
        // Execute shader
        if hit.is_some() {
            let hitSg = hit.unwrap();
            if hitSg.dist < min_dist || min_dist.is_nan() {
                min_dist = hitSg.dist;
                first_hit = Some(hitSg);
                first_hit_object = Some(*object);
            }
        }
    }
    
    // Execute shader on closest hit and return hit result
    if first_hit_object.is_some() {
        let hit = first_hit.unwrap();
        let hit_sg = first_hit_object.unwrap().get_sg(ray, &hit);
        let hit_point = hit.P.unwrap();
        let color = first_hit_object.unwrap().get_shader().evaluate(scene, &hit_sg);
        // TODO : attenuation

        Some( RtHit::new(true, color, hit_point) )
    } else {
        None
    }
}


/// Launch to lights
pub fn RtTraceToLights(scene: &RtScene, ray: &RtRay) -> Option<RtHit> {
    let objects = scene.get_scene_objects();

    let mut min_dist: f32 = NAN;
    let mut first_hit: Option<RtRayHit> = None;
    let mut first_hit_object: Option<&dyn RtObject> = None;

    // Find closest hit point & object
    for object in objects {
        // Compute intersections
        let hit = object.get_intersection(ray);
        // Execute shader
        if hit.is_some() {
            let hitSg = hit.unwrap();
            if hitSg.dist < min_dist || min_dist.is_nan() {
                min_dist = hitSg.dist;
                first_hit = Some(hitSg);
                first_hit_object = Some(*object);
            }
        }
    }
    
    // Execute shader on closest hit and return hit result
    if first_hit_object.is_some() {
        if first_hit_object.unwrap().get_type().as_str() == "light" {
            let hit = first_hit.unwrap();
            let hit_sg = first_hit_object.unwrap().get_sg(ray, &hit);
            let hit_point = hit.P.unwrap();
            let color = first_hit_object.unwrap().get_shader().evaluate(scene, &hit_sg);
            // TODO : attenuation
            Some( RtHit::new(true, color, hit_point) )
        } else {
            // Not a light : masked
            None
        }
    } else {
        None
    }
}

/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Implements the ray tracing function.
/// =====================================================

use crate::rt_ray::*;
use crate::rt_shader_globals::*;
use crate::rt_scene::*;
use crate::RtVec3;


/// Launch a ray on a scene
pub fn RtTraceRay(scene: &mut RtScene, ray: &RtRay) -> Option<RtHit> {
    let object = scene.get_scene_geometry();
    // TODO : from one to multiple objects

    // Compute intersections
    let hit: Option<RtShaderGlobals> = object.intersect(ray);

    // Execute shader
    if hit.is_some() {
        let hitSg = hit.unwrap();
        let color = object.get_shader().evaluate(scene, &hitSg);
        // TODO : attenuation
        return Some( RtHit::new(true, color, hitSg.P) )
    }
    None
}

/// Launch to lights
pub fn RtTraceToLights(scene: &RtScene, ray: &RtRay) -> Option<RtHit> {
    let light = scene.get_scene_light();

    // Compute intersections
    let hit: Option<RtShaderGlobals> = light.intersect(ray);

    // Execute shader
    if hit.is_some() {
        let hitSg = hit.unwrap();
        // Use hit distance and hit color to compute color here
        let color = light.get_shader().evaluate(scene, &hitSg);
        // TODO : attenuation
        return Some( RtHit::new(true, color, hitSg.P) )
    }
    None
}

/// Launch a ray on a scene
pub fn RtReflectRay(normal: &RtVec3, sg: &RtShaderGlobals) -> RtRay {
    RtRay { 
        origin: sg.P, 
        dir: sg.ray_dir - 2.0 * RtVec3::dot(sg.ray_dir, *normal) * *normal,
        bounces: sg.bounces + 1, 
        x: sg.x, 
        y: sg.y
    }
}

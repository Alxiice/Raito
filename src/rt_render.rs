/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Implements the ray tracing function.
/// =====================================================

use crate::rt_ray::*;
use crate::rt_shader_globals::*;
use crate::rt_scene::*;


/// Launch a ray on a scene
pub fn RtTraceRay(scene: &mut RtScene<'_>, ray: &RtRay) -> Option<RtHit> {
    let object = scene.get_scene_geometry();
    // TODO : from one to multiple objects

    // Compute intersections
    let hit: Option<RtShaderGlobals> = object.intersect(ray);

    // Execute shader
    if hit.is_some() {
        return Some(
            RtHit::new(true, object.get_shader().evaluate(&hit.unwrap()))
        );
    }
    None
}

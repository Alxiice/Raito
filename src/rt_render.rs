/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Implements the ray tracing function.
/// =====================================================

use std::f32::NAN;

use crate::rt_types::*;
use crate::rt_camera::*;
use crate::rt_ray::*;
use crate::rt_shader_globals::*;
use crate::rt_scene::*;
use crate::RtVec3;
use crate::rt_objects::rt_object_base::*;
use crate::rt_render_output::RtRenderResult;


// ========================================
//  Creating rays
// ========================================

/// Utility function to form a new ray from a shading point
pub fn RtMakeRay(sg: &RtShaderGlobals, _raytype: RtRayType, dir: RtVec3, _maxdist: f32) -> RtRay {
    RtRay { 
        origin: sg.P, 
        dir,
        bounces: sg.bounces + 1, 
        x: sg.x, 
        y: sg.y
    }
}

/// Launch a ray on a scene
pub fn RtReflectRay(ray: &mut RtRay, wo: &RtVec3, normal: &RtVec3, _sg: &RtShaderGlobals) {
    ray.dir = *wo - 2.0 * RtVec3::dot(*wo, *normal) * *normal;
}


// ========================================
//  Launching rays
// ========================================

/// Launch a ray on a scene
pub fn RtTraceRay(scene: &RtScene, ray: &RtRay) -> Option<RtHit> {
    let mut min_dist: f32 = NAN;
    let mut first_hit: Option<RtRayHit> = None;
    let mut first_hit_object: Option<&Box<dyn RtObject>> = None;
    
    // Shapes : Find closest hit point & object
    let geometry = scene.list_shapes();
    for shape in geometry {
        // Compute intersections
        let hit = shape.get_intersection(ray);
        // Execute shader
        if hit.is_some() {
            let hitSg = hit.unwrap();
            if hitSg.dist < min_dist || min_dist.is_nan() {
                min_dist = hitSg.dist;
                first_hit = Some(hitSg);
                first_hit_object = Some(shape);
            }
        }
    }

    // Lights : Find closest hit point & object
    let lights = scene.list_lights();
    for light in lights {
        // Compute intersections
        let hit = light.get_intersection(ray);
        // Execute shader
        if hit.is_some() {
            let hitSg = hit.unwrap();
            if hitSg.dist < min_dist || min_dist.is_nan() {
                min_dist = hitSg.dist;
                first_hit = Some(hitSg);
                first_hit_object = Some(light);
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
    
    let mut min_dist: f32 = NAN;
    let mut first_hit: Option<RtRayHit> = None;
    let mut first_hit_object: Option<&Box<dyn RtObject>> = None;
    
    // Find closest hit point & object
    let lights = scene.list_lights();
    for light in lights {
        // Compute intersections
        let hit = light.get_intersection(ray);
        // Execute shader
        if hit.is_some() {
            let hitSg = hit.unwrap();
            if hitSg.dist < min_dist || min_dist.is_nan() {
                min_dist = hitSg.dist;
                first_hit = Some(hitSg);
                first_hit_object = Some(light);
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


/// Launch render on scene
pub fn RtRenderScene(scene: &RtScene, result: &mut RtRenderResult) {
    // TODO : for now the camera
    // - center is at 0
    // - direction is towards the -y direction
    // 
    // We want to be able to change that, move and rotate the camera
    // We need to implement world and camera space

    // let mut camera = RtCamera::new(result.width as u16, 1.0);
    // camera.camera_fov = scene.get_camera().camera_fov;
    let cam_rays = RtCameraRayIterator::new(scene.get_camera());
    for camera_ray in cam_rays {
        let mut pixelColor = RtRGBA::BLACK;
        for _ in 0..NB_SUBPIXELS {
            let ray = camera_ray.get_ray(scene.get_camera());
            let hit = RtTraceRay(scene, &ray);
            if hit.is_some() {
                let hitResult = hit.unwrap();
                pixelColor += hitResult.colorOutput * INV_NB_SUBPIXELS;
            } else {
                pixelColor += RtRGBA::ERRCOLOR  * INV_NB_SUBPIXELS;
            }
        }
        // panic!("Pixel : {} {}", camera_ray.x(), camera_ray.y());
        result.set_pixel_color(camera_ray.x(), camera_ray.y(), pixelColor);
    }
}

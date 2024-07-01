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
    ray.dir = (*wo - 2.0 * RtVec3::dot(*wo, *normal) * *normal).normalize();
}

// TODO : fix to avoid front facing normal ?
pub fn RtRefractRay(ray: &mut RtRay, wo: &RtVec3, normal: &RtVec3, eta: f32, _sg: &RtShaderGlobals) {
    let cos_theta = RtVec3::dot(-*wo, *normal).min(1.0);
    let r_out_perp: RtVec3 = eta * (*wo + cos_theta * *normal);
    let r_out_parallel: RtVec3 = - (1.0 - r_out_perp.length_squared()).abs().sqrt() * *normal;
    ray.dir = (r_out_perp + r_out_parallel).normalize();
}

// ========================================
//  Launching rays
// ========================================

/// Launch a ray on a scene
pub fn RtTraceRay(scene: &RtScene, ray: &RtRay) -> Option<RtHit> {
    if ray.bounces >= scene.settings.max_bounces {
        return None
    }

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
    // let lights = scene.list_lights();
    // for light in lights {
    //     // Compute intersections
    //     let hit = light.get_intersection(ray);
    //     // Execute shader
    //     if hit.is_some() {
    //         let hitSg = hit.unwrap();
    //         if hitSg.dist < min_dist || min_dist.is_nan() {
    //             min_dist = hitSg.dist;
    //             first_hit = Some(hitSg);
    //             first_hit_object = Some(light);
    //         }
    //     }
    // }
    
    // Execute shader on closest hit and return hit result
    if first_hit_object.is_some() {
        let hit = first_hit.unwrap();
        let hit_sg = first_hit_object.unwrap().get_sg(ray, &hit);
        let hit_point = hit.P.unwrap();
        let color = first_hit_object.unwrap().get_shader().evaluate(scene, &hit_sg);

        Some( RtHit::new(true, color, hit_point) )
    } else {
        // None
        let a = 0.5 * ray.dir.y + 1.0;
        let skyColor = (1.0 - a) * RtRGBA::WHITE + a * RtRGBA::from_rgb(0.5, 0.7, 1.0);
        Some(RtHit::new(false, skyColor, RtPoint3::default()))
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


fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
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

    let inv_nb_spp: f32 = 1.0 / (scene.settings.render_spp as f32);

    let cam_rays = RtCameraRayIterator::new(scene.get_camera());
    for camera_ray in cam_rays {
        let mut pixelColor = RtRGBA::BLACK;
        for _ in 0..scene.settings.render_spp {
            let ray = camera_ray.get_ray(scene.get_camera());
            let hit = RtTraceRay(scene, &ray);
            if hit.is_some() {
                let hitResult = hit.unwrap();
                pixelColor += hitResult.colorOutput * inv_nb_spp;
            } else {
                // let a = 0.5 * ray.dir.y + 1.0;
                // let skyColor = (1.0 - a) * RtRGBA::WHITE + a * RtRGBA::from_rgb(0.5, 0.7, 1.0);
                // pixelColor += skyColor * inv_nb_spp;
                pixelColor += RtRGBA::ERRCOLOR  * inv_nb_spp;
            }
        }
        // panic!("Pixel : {} {}", camera_ray.x(), camera_ray.y());
        let outColor = RtRGBA::from_rgb(
            linear_to_gamma(pixelColor.r), 
            linear_to_gamma(pixelColor.g), 
            linear_to_gamma(pixelColor.b) 
        );

        result.set_pixel_color(camera_ray.x(), camera_ray.y(), outColor);
    }
}

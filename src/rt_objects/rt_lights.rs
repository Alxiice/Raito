/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines objects (geometries or lights) 
/// =====================================================

use crate::rt_types::*;
use crate::rt_ray::*;
use crate::rt_objects::rt_object_base::*;


// ========================================
//  Define objects
// ========================================

pub struct RtSkydomeLight {
    pub object_params: ObjectParams,
    pub color: RtRGBA,
    pub intensity: f32
}

impl RtSkydomeLight {
    const RT_TYPE: &'static str = "<RtLight : Skydome>";
}

/// Point light
pub struct RtPointLight {
    pub object_params: ObjectParams,
    pub center: RtPoint3,
    pub radius: f32
}

impl RtPointLight {
    const RT_TYPE: &'static str = "<RtLight : Point>";
}

/// Defines light types
pub enum RtLightTypes {
    Skydome(RtSkydomeLight),
    Point(RtPointLight)
}


// ========================================
//  Implement object traits
// ========================================

impl RtObject for RtPointLight {
    fn getObjectParams(&self) -> &ObjectParams {
        &self.object_params
    }
    
    fn get_intersection(&self, ray: &RtRay) -> Option<RtRayHit> {
        let a = RtVec3::dot(ray.dir, ray.dir);
        let b = 2.0 * RtVec3::dot(ray.dir, ray.origin - self.center);
        let c = (ray.origin - self.center).squared() - self.radius * self.radius;
        // Solve quadratic equation
        let delta: f32 = b * b - 4.0 * a * c;
        if delta < 0.0 {
            return None
        }
        let sqrt_delta = delta.sqrt();
        let x1 = (-b + sqrt_delta) / (2.0 * a);
        let x2 = (-b - sqrt_delta) / (2.0 * a);
        if x1 >= 0.0 && (x2 < 0.0 || x1 <= x2) {
            return Some(RtRayHit::new(true, x1, ray.origin + x1 * ray.dir))
        }
        if x2 >= 0.0 {
            return Some(RtRayHit::new(true, x2, ray.origin + x2 * ray.dir))
        }
        None
    }

    fn get_normal(&self, point: &RtPoint3) -> RtVec3 {
        (*point - self.center).normalize()
    }
}

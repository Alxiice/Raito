use log::info;

/// =====================================================
///                    Raito Render
/// 
/// Module authors : 
/// - Alice Sonolet <alice.sonolet@gmail.com>
/// 
/// Module description :
///   Defines some geometry and utility functions to use 
///   it
/// =====================================================

use crate::rt_types::*;
use crate::rt_ray::*;
use crate::rt_shader_globals::*;

// ========================================
//  Traits
// ========================================

pub trait Intersect {
    fn intersect(&self, ray: &RtRay) -> Option<RtShaderGlobals>;
}


// ========================================
//  Geometries
// ========================================

pub struct RtSphere {
    pub center: RtPoint3,
    pub radius: f32
}

impl RtSphere {
    const rt_type: &'static str = "<RtGeometry : Sphere>";
}


// ========================================
//  Lights
// ========================================

/// Skydome light
pub struct RtSkydomeLight {
    pub color: RtRGBA,
    pub intensity: f32
}

impl RtSkydomeLight {
    const rt_type: &'static str = "<RtLight : Skydome>";
}


/// Point light
pub struct RtPointLight {
    pub color: RtRGBA,
    pub intensity: f32,
    pub center: RtPoint3,
    pub radius: f32
}

impl RtPointLight {
    const rt_type: &'static str = "<RtLight : Point>";
}


// ========================================
//  Intersections
// ========================================

impl Intersect for RtSphere {
    /// Compute intersection with ray and sphere
    fn intersect(&self, ray: &RtRay) -> Option<RtShaderGlobals> {
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
            return Some(RtShaderGlobals::from_intersection(ray, ray.origin + x1 * ray.dir))
        } else if x2 >= 0.0 {
            return Some(RtShaderGlobals::from_intersection(ray, ray.origin + x2 * ray.dir))
        }
        None
    }
}


// ========================================
//  Objects
// ========================================

/// Defines all geometry types
enum RtGeometry {
    Sphere(RtSphere)
}

/// Defines all light types
enum RtLight {
    Skydome(RtSkydomeLight),
    Point(RtPointLight)
}

/// Defines all object types
enum RtObject {
    Geometry(RtGeometry),
    Light(RtLight)
}

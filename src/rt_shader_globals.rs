/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Global struct holding informations for shading
/// =====================================================

use crate::rt_types::*;
use crate::rt_ray::*;


/// Shader Globals is a struct holding
/// informations computed by the rendering engine on 
/// each shading point, and that can be used during shading.
pub struct RtShaderGlobals {
    /// X raster-space coordinate of this ray tree
    pub x: u16,
    /// Y raster-space coordinate of this ray tree
    pub y: u16,

    /// Shading point in world-space
    pub P: RtPoint3,
    // Unique name of the geometry
    pub Po: String,

    /// ray origin (typically the camera position)
    pub ray_origin: RtPoint3,
    /// ray direction (normalized)
    pub ray_dir: RtVec3,

    // TODO : Ray type
    // pub ray_type: u8,

    // Bounces
    /// recursion level for the ray that created this hit
    pub bounces: u8,

    // object being shaded
    // pub object: RtObjectNode

    // Normals
    /// shading normal
    pub N: RtVec3,
    // TODO : Nf, Ng, Ngf, Ns ...

    // UV coordinates
    // U surface parameter
    // pub u: RtVec3,
    // V surface parameter
    // pub v: RtVec3,
    
    // Shading context
    // pub shading_context: u8,
}

impl RtShaderGlobals {
    /// Camera shader globals
    pub fn default(x: u16, y: u16) -> Self {
        Self {
            x, 
            y, 
            P: RtPoint3::default(), 
            Po: String::default(),
            ray_origin: RtPoint3::default(), 
            ray_dir: RtVec3::default(), 
            bounces: 0, 
            N: RtVec3::default()
        }
    }
    
    /// Create new shader globals
    pub fn new(x: u16, 
               y: u16, 
               P: RtPoint3, 
               Po: String, 
               ray_origin: RtPoint3, 
               ray_dir: RtVec3, 
               bounces: u8, 
               N: RtVec3) -> Self {
        Self { x, y, P, Po, ray_origin, ray_dir, bounces, N }
    }

    pub fn from_intersection(ray: &RtRay, Po: String, intersection: RtPoint3) -> Self {
        Self {
            x: ray.x,
            y: ray.y,
            P: intersection,
            Po,
            ray_origin: ray.origin,
            ray_dir: ray.dir,
            bounces: ray.bounces, 
            N: RtVec3::default()
        }
    }

}

/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines a Ray
/// =====================================================

// TODO : could be moved in rt_types

use std::f32::NAN;

use crate::rt_objects::rt_object_base::RtObject;
use crate::rt_types::*;
use crate::rt_shader_globals::*;

pub enum RtRayType {
    RT_RAY_UNKNOWN
}

/// Describes a ray
pub struct RtRay {
    // Basic ray infos
    /// Ray origin
    pub origin: RtPoint3,
    /// Ray direction
    pub dir: RtVec3,

    // Bounces
    /// Number of bounces so far (0 for camera rays)
    pub bounces: u8,

    // Raster space
    /// Raster-space X coordinate 
    pub x: u16,
    /// Raster-space Y coordinate 
    pub y: u16
}

impl RtRay {
    /// Create ray
    pub fn new(sg: &RtShaderGlobals, origin: RtPoint3, dir: RtVec3) -> Self {
        Self {
            origin,
            dir,
            bounces: sg.bounces + 1,
            x: sg.x,
            y: sg.y
        }
    }

    /// Function that gives P(t) the position where we land 
    /// after tracing the ray for a distance t
    pub fn at(&mut self, t: f32) -> RtPoint3 {
        self.origin + t * self.dir
    }
}

impl std::fmt::Display for RtRay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<RtRay O=({}, {}, {}), dir=({}, {}, {})>", 
            self.origin.x, self.origin.y, self.origin.z,
            self.dir.x, self.dir.y, self.dir.z)
    }
}


/// Holds ray intersection
/// 
/// **Note** : Could also store a reference to the object if we add a lifetime
pub struct RtRayHit {
    pub hit: bool,
    pub dist: f32,
    pub P: Option<RtPoint3>
}

impl Default for RtRayHit {
    fn default() -> Self {
        Self { hit: false, dist: NAN, P: None }
    }
}

impl RtRayHit {
    pub fn new(hit: bool, dist: f32, P: RtPoint3) -> Self {
        Self { hit, dist, P: Some(P) }
    }
}


/// Holds ray hit info
pub struct RtHit {
    // Hit infos
    pub hit: bool,
    pub colorOutput: RtRGBA,
    pub P: RtPoint3
}

impl RtHit {
    pub fn new(hit: bool, colorOutput: RtRGBA, P: RtPoint3) -> Self {
        Self { hit, colorOutput, P }
    }
}

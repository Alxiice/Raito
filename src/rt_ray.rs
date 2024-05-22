/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines a Ray
/// =====================================================

use crate::rt_types::*;
use crate::rt_shader_globals::*;

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
    pub fn new(sg: RtShaderGlobals, origin: RtPoint3, dir: RtVec3) -> Self {
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


/// Holds ray hit info
pub struct RtHit {
    // Hit infos
    pub hit: bool,
    pub colorOutput: RtRGBA
}

impl RtHit {
    pub fn new(hit: bool, colorOutput: RtRGBA) -> Self {
        Self { hit, colorOutput }
    }
}

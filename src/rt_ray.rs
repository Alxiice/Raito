/// =====================================================
///                    Raito Render
/// 
/// Module authors : 
/// - Alice Sonolet <alice.sonolet@gmail.com>
/// 
/// Module description :
///   Defines a Ray
/// =====================================================

use crate::rt_types::*;

/// Describes a ray
pub struct RtRay {
    pub origin: RtPoint3,
    pub dir: RtVec3
}

impl RtRay {
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

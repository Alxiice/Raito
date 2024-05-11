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

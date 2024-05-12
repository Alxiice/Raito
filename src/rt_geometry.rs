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

// ========================================
//  Geometry
// ========================================

pub struct RtSphere {
    pub center: RtPoint3,
    pub radius: f32
}


// ========================================
//  Intersections
// ========================================

pub trait RtIntersect {
    fn intersect(&self, ray: RtRay) -> bool;
}

impl RtIntersect for RtSphere {
    fn intersect(&self, ray: RtRay) -> bool {
        // Deltas
        let deltx = ray.origin.x - self.center.x;
        let delty = ray.origin.y - self.center.y;
        let deltz = ray.origin.z - self.center.z;
        // A, B, C
        let A = ray.dir.x * ray.dir.x + ray.dir.y * ray.dir.y + ray.dir.y * ray.dir.y;
        let B = 2.0 * ray.dir.x * deltx + 2.0 * ray.dir.y * delty + 2.0 * ray.dir.z * deltz;
        let C = deltx * deltx + delty * delty + deltz * deltz - self.radius * self.radius;
        // Solve quadratic equation
        let delta = B * B - 4.0 * A * C;
        // ray.origin, ray.dir
        // self.center, self.radius
        if delta >= 0.0 {
            return true
        }
        false
    }
}

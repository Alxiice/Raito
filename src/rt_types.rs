/// =====================================================
///                    Raito Render
/// 
/// Module authors : 
/// - Alice Sonolet <alice.sonolet@gmail.com>
/// 
/// Module description :
///   Defines Raito Types (Rt) that the render engine 
///   uses.
/// =====================================================

// ========================================
//  RtRGB
// ========================================

use std::default;

#[derive(Copy, Clone)]
pub struct RtRGB {
    r: u8,
    g: u8,
    b: u8,
}

impl Default for RtRGB {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0
        }
    }
}

impl RtRGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r, g, b
        }
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
}

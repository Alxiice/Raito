/// =====================================================
///                    Raito Render
/// 
/// Module authors : 
/// - Alice Sonolet <alice.sonolet@gmail.com>
/// 
/// Module description :
///   Defines types that the render engine uses.
/// =====================================================

// ========================================
//  Colors
// ========================================

use std::default;

/// RGBA Color
#[derive(Copy, Clone)]
pub struct RtRGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Default for RtRGBA {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255
        }
    }
}

impl RtRGBA {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        let a: u8 = 255;
        Self {
            r, g, b, a
        }
    }

    #[inline]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let a: u8 = 255;
        Self { r, g, b, a }
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

    pub const BLACK: Self = Self::from_rgb(0, 0, 0);
    pub const WHITE: Self = Self::from_rgb(255, 255, 255);
    pub const RED  : Self = Self::from_rgb(255, 0, 0);
    pub const GREEN: Self = Self::from_rgb(0, 255, 0);
    pub const BLUE : Self = Self::from_rgb(0, 0, 255);
}


// ========================================
//  Vectors
// ========================================

/// 3D Point
#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct RtPoint3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/// 2D Vector
#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct RtVec2 {
    pub x: f32,
    pub y: f32
}

/// 3D Vector
#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct RtVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/// Implements Add for RtPoint3 - RtPoint3
/// Result is a vector
impl std::ops::Sub<RtPoint3> for RtPoint3 {
    type Output = RtVec3;
    fn sub(self, rhs: RtPoint3) -> Self::Output {
        Self::Output { x: rhs.x - self.x, y: rhs.y - self.y, z: rhs.z - self.z }
    }
}

/// Implements Mul for RtVec3 * f32
impl std::ops::Mul<f32> for RtVec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

/// Implements Mul for f32 * RtVec3
impl std::ops::Mul<RtVec3> for f32 {
    type Output = RtVec3;
    fn mul(self, rhs: RtVec3) -> Self::Output {
        Self::Output { x: rhs.x * self, y: rhs.y * self, z: rhs.z * self }
    }
}

/// Implements Div for RtVec3 / f32
impl std::ops::Div<f32> for RtVec3 {
    type Output = RtVec3;
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

/// Implements Add for RtVec3 + RtPoint3
impl std::ops::Add<RtPoint3> for RtVec3 {
    type Output = RtPoint3;
    fn add(self, rhs: RtPoint3) -> Self::Output {
        Self::Output { x: rhs.x + self.x, y: rhs.y + self.y, z: rhs.z + self.z }
    }
}

/// Implements Add for RtPoint3 + RtVec3
impl std::ops::Add<RtVec3> for RtPoint3 {
    type Output = RtPoint3;
    fn add(self, rhs: RtVec3) -> Self::Output {
        Self::Output { x: rhs.x + self.x, y: rhs.y + self.y, z: rhs.z + self.z }
    }
}

impl RtVec3 {
    /// Vector norm
    fn get_norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalized vector
    pub fn normalize(self) -> Self {
        self / self.get_norm()
    }
}

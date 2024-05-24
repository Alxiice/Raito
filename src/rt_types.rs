/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines types that the render engine uses.
/// =====================================================

// ========================================
//  Colors
// ========================================

use egui::Color32;


/// RGBA Color
#[derive(Copy, Clone)]
pub struct RtRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
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

    pub const fn from_color32(color: Color32) -> Self {
        Self { 
            r: color.r(),
            g: color.g(),
            b: color.b(),
            a: color.a()
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

impl RtPoint3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl std::fmt::Display for RtPoint3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<RtPoint3 ({}, {}, {})>", self.x, self.y, self.z)
    }
}

/// 3D Vector
#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct RtVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl std::fmt::Display for RtVec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<RtVec3 ({}, {}, {})>", self.x, self.y, self.z)
    }
}

impl std::ops::Sub<RtPoint3> for RtPoint3 {
    type Output = RtVec3;
    /// Implements Sub for RtPoint3 - RtPoint3
    /// 
    /// Result is a vector
    fn sub(self, rhs: RtPoint3) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl std::ops::Mul<f32> for RtVec3 {
    type Output = Self;
    /// Implements Mul for RtVec3 * f32
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl std::ops::Mul<RtVec3> for f32 {
    type Output = RtVec3;
    /// Implements Mul for f32 * RtVec3
    fn mul(self, rhs: RtVec3) -> Self::Output {
        Self::Output { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}


// Dot product
impl std::ops::Mul<RtVec3> for RtVec3 {
    type Output = f32;
    /// Implements dot product for RtVec3 * RtVec3
    fn mul(self, rhs: RtVec3) -> Self::Output {
        let product = 
            self.x * rhs.x +
            self.y * rhs.y +
            self.z * rhs.z;
    
        product
    }
}

impl std::ops::Div<f32> for RtVec3 {
    type Output = RtVec3;
    /// Implements Div for RtVec3 / f32
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl std::ops::Add<RtPoint3> for RtVec3 {
    type Output = RtPoint3;
    /// Implements Add for RtVec3 + RtPoint3
    fn add(self, rhs: RtPoint3) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl std::ops::Add<RtVec3> for RtPoint3 {
    type Output = RtPoint3;
    /// Implements Add for RtPoint3 + RtVec3
    fn add(self, rhs: RtVec3) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl RtVec3 {
    /// Vector length
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalized vector
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn squared(self) -> f32 {
        self * self
    }

    pub fn dot(u: Self, v: Self) -> f32 {
        u * v
    }

    pub fn cross(u: Self, v: Self) -> Self {
        Self {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x
        }
    }
}

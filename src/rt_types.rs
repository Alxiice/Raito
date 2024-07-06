/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines types that the render engine uses.
/// =====================================================

// ========================================
//  Colors
// ========================================

use std::ops::Neg;

use egui::Color32;

use crate::{random_float, random_float_range};

// ========================================
//  Constants
// ========================================

pub const RT_EPSILON: f32 = 0.0001;
pub const RT_PI: f32 = 3.1416;
pub const RT_ONEOVERPI: f32 = 0.3183;

// ========================================
//  Colors
// ========================================

/// RGBA Color
#[derive(Copy, Clone, Debug)]
pub struct RtRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Default for RtRGBA {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }
    }
}

impl RtRGBA {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r, g, b, a: 1.0
        }
    }

    pub fn random() -> Self {
        Self {
            r: random_float(),
            g: random_float(),
            b: random_float(),
            a: 1.0
        }
    }
    
    pub fn random_range(a: f32, b: f32) -> Self {
        Self {
            r: random_float_range(a, b),
            g: random_float_range(a, b),
            b: random_float_range(a, b),
            a: 1.0
        }
    }

    pub fn clamp(self) -> RtRGBA {
        RtRGBA {
            r: self.r.max(0.0).min(1.0),
            g: self.g.max(0.0).min(1.0),
            b: self.b.max(0.0).min(1.0),
            a: self.a.max(0.0).min(1.0)
        }
    }

    #[inline]
    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    // Would be better on const function but not supported byy rust yet
    pub fn from_color32(color: Color32) -> Self {
        Self { 
            r: color.r() as f32 / 255.0,
            g: color.g() as f32 / 255.0,
            b: color.b() as f32 / 255.0,
            a: color.a() as f32 / 255.0
        }
    }

    // Would be better on const function but not supported byy rust yet
    pub fn to_color32(&self) -> Color32 {
        // TODO : handle A ?
        Color32::from_rgb(
            (self.r * 255.0) as u8, 
            (self.g * 255.0) as u8, 
            (self.b * 255.0) as u8
        )
    }

    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn g(&self) -> f32 {
        self.g
    }

    pub fn b(&self) -> f32 {
        self.b
    }

    pub const BLACK: Self       = Self::from_rgb(0.0, 0.0, 0.0);
    pub const WHITE: Self       = Self::from_rgb(1.0, 1.0, 1.0);
    pub const RED  : Self       = Self::from_rgb(1.0, 0.0, 0.0);
    pub const GREEN: Self       = Self::from_rgb(0.0, 1.0, 0.0);
    pub const BLUE : Self       = Self::from_rgb(0.0, 0.0, 1.0);
    pub const ERRCOLOR : Self   = Self::from_rgb(0.9, 0.5, 0.6);
}

impl std::ops::Add<RtRGBA> for RtRGBA {
    type Output = Self;
    /// Implements Add for RtRGBA * RtRGBA
    fn add(self, rhs: RtRGBA) -> Self::Output {
        // let opacity_base: f32 = self.a.clamp(0.0, 1.0);
        RtRGBA {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            // a: opacity_base + (1.0 - opacity_base) * rhs.a.clamp(0.0, 1.0)
            a: self.a + (1.0 - self.a) * rhs.a
        }
    }
}

impl std::ops::AddAssign<RtRGBA> for RtRGBA {
    /// Implements Add for RtRGBA += RtRGBA
    fn add_assign(&mut self, rhs: RtRGBA) {
        // let opacity_base: f32 = self.a.clamp(0.0, 1.0);
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        // self.a = opacity_base + (1.0 - opacity_base) * rhs.a.clamp(0.0, 1.0);
        self.a = self.a + (1.0 - self.a) * rhs.a;
    }
}

impl std::ops::Mul<RtRGBA> for f32 {
    type Output = RtRGBA;
    /// Implements Mul for f32 * RtRGBA
    fn mul(self, rhs: RtRGBA) -> Self::Output {
        Self::Output { 
            r: self * rhs.r, 
            g: self * rhs.g, 
            b: self * rhs.b,
            a: self * rhs.a  // ?
        }
    }
}

impl std::ops::Mul<f32> for RtRGBA {
    type Output = Self;
    /// Implements Mul for RtRGBA * f32
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output { 
            r: self.r * rhs, 
            g: self.g * rhs, 
            b: self.b * rhs,
            a: self.a * rhs  // ?
        }
    }
}

impl std::ops::Mul<RtRGBA> for RtRGBA {
    type Output = Self;
    /// Implements Mul for RtRGBA * RtRGBA
    fn mul(self, rhs: RtRGBA) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a
        }
    }
}

impl std::ops::Div<f32> for RtRGBA {
    type Output = RtRGBA;
    /// Implements Div for RtRGBA / f32
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output { 
            r: self.r / rhs, 
            g: self.g / rhs, 
            b: self.b / rhs, 
            a: self.a / rhs  // Or don't touch to a ?
        }
    }
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

impl Neg for RtVec3 {
    type Output = RtVec3;

    /// Implements -RtVec3
    fn neg(self) -> Self::Output {
        RtVec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Add<RtVec3> for RtVec3 {
    type Output = Self;
    /// Implements Add for RtVec3 * RtVec3
    fn add(self, rhs: RtVec3) -> Self::Output {
        RtVec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
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

impl std::ops::Sub<RtVec3> for RtVec3 {
    type Output = RtVec3;
    /// Implements Sub for RtPoint3 - RtPoint3
    /// 
    /// Result is a vector
    fn sub(self, rhs: RtVec3) -> Self::Output {
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
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_point3(self) -> RtPoint3 {
        RtPoint3 { x: self.x, y: self.y, z: self.z }
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Vector length
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn squared(self) -> f32 {
        self * self
    }

    /// Normalized vector
    pub fn normalize(self) -> Self {
        self / self.length()
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

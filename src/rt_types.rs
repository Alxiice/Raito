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


pub const RT_EPSILON: f32 = 0.0001;


/// RGBA Color
#[derive(Copy, Clone)]
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

impl RtRGBA {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r, g, b, a: 1.0
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
        let opacity_base: f32 = self.a.clamp(0.0, 1.0);
        RtRGBA {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: opacity_base + (1.0 - opacity_base) * rhs.a.clamp(0.0, 1.0)
        }
    }
}

impl std::ops::AddAssign<RtRGBA> for RtRGBA {
    /// Implements Add for RtRGBA += RtRGBA
    fn add_assign(&mut self, rhs: RtRGBA) {
        let opacity_base: f32 = self.a.clamp(0.0, 1.0);
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a = opacity_base + (1.0 - opacity_base) * rhs.a.clamp(0.0, 1.0);
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
        // // Convert to normalized space
        // let r1 = self.r as f32 / 255.0;
        // let g1 = self.g as f32 / 255.0;
        // let b1 = self.b as f32 / 255.0;
        // let a1 = self.a as f32 / 255.0;
        // let r2 = rhs.r as f32 / 255.0;
        // let g2 = rhs.g as f32 / 255.0;
        // let b2 = rhs.b as f32 / 255.0;
        // let a2 = rhs.a as f32 / 255.0;
        // // Then normalize
        // Self::Output {
        //     r: (255.0 * (r1 * r2)) as u8,
        //     g: (255.0 * (g1 * g2)) as u8,
        //     b: (255.0 * (b1 * b2)) as u8,
        //     a: (255.0 * (a1 * a2)) as u8
        // }
        // Simplification
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a
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

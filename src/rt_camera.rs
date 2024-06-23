/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines a camera
/// =====================================================

use std::cell::OnceCell;

use crate::rt_types::*;
use crate::rt_ray::*;
use crate::rt_shader_globals::*;
use crate::rt_sampler::*;

pub const SUBPX_SEED: u64 = 1;
pub const NB_SUBPIXELS: u8 = 3;
pub const INV_NB_SUBPIXELS: f32 = 1.0 / (NB_SUBPIXELS as f32);

/// Describes a camera
/// 
/// Right handed system
pub struct RtCamera {
    pub center: RtPoint3,
    pub camera_fov: f32,
    pub aspect_ratio: f32,
    camera_width : OnceCell<u16>,
    camera_height: OnceCell<u16>,
}

impl RtCamera {
    /// Creates a new camera
    pub fn new(camera_width: u16, aspect_ratio: f32) -> Self {
        Self { 
            center: RtPoint3::new(0.0, 0.0, 0.0),
            camera_fov: 50.0,
            aspect_ratio,
            camera_width: OnceCell::from(camera_width),
            camera_height: OnceCell::from((camera_width as f32 / aspect_ratio) as u16)
        }
    }

    /// Get width of the camera in pixel
    #[inline]
    pub fn camera_width(&self) -> &u16 {
        self.camera_width.get().unwrap()
    }
    
    /// Get height of the camera in pixel
    #[inline]
    pub fn camera_height(&self) -> &u16 {
        self.camera_height.get().unwrap()
    }

    /// Top left point of the camera
    /// 
    /// In this first implementation the height of the camera is always 1.0
    /// and therefore the width is the aspect ratio (aspect = width / height)
    /// 
    /// **TODO :** Later we could implement a camera with top, left, right, bottom values
    #[inline]
    fn top_left(&self) -> RtPoint3 {
        RtPoint3 {
            x: -(self.aspect_ratio) / 2.0,  // Left position (aspect / 2)
            y: -0.5,  // Top position (1 / 2)
            z: - (self.aspect_ratio) / (self.camera_fov / 2.0).to_radians().tan()
        }
    }

    /// Sample camera ray
    /// 
    /// We shoot the ray at the center of the pixel for each pixel in the grid
    /// 
    /// (x: column, y: row) : pixel position
    pub fn get_camera_ray(&self, x: u16, y: u16, px: f32, py: f32) -> RtRay {
        // Get a point on the viewport
        // Start with top left
        let mut viewport_point = self.top_left();
        // Offset with the given pixel
        viewport_point.x += (self.aspect_ratio / *self.camera_width()  as f32) * (x as f32 + px);
        viewport_point.y += (1.0               / *self.camera_height() as f32) * (y as f32 + py);
        let direction = viewport_point - self.center;
        // Create shader globals
        let sg = RtShaderGlobals::default(x, y);
        // Create the ray from the center
        let mut ray = RtRay::new(&sg, self.center, direction.normalize());
        ray.bounces = 0;
        ray
    }
}


// ========================================
//  Iterator 
// (iterate on rays from the camera)
// ========================================

pub struct RtPixel {
    x: u16,
    y: u16
}

impl RtPixel {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn get_ray(&self, camera: &RtCamera) -> RtRay {
        camera.get_camera_ray(
            self.x, 
            self.y,
            random_float(),
            random_float()
        )
    }

    pub fn x(&self) -> usize {
        usize::from(self.x)
    }

    pub fn y(&self) -> usize {
        usize::from(self.y)
    }
}

/// Could be reimplemented as trait to iterate in 
/// - buckets
/// - also not top left to bottom right but allow circular, etc
pub struct RtCameraRayIterator<'a> {
    // Camera
    camera: &'a RtCamera,
    /// Current position
    current_x: u16,
    current_y: u16,
}

impl<'a> RtCameraRayIterator<'a> {
    /// Creates an iterator for camera rays
    pub fn new(camera: &'a RtCamera) -> Self {
        Self {
            camera,
            current_x: 0,
            current_y: 0
        }
    }
}

impl<'a> Iterator for RtCameraRayIterator<'a> {
    type Item = RtPixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_x >= *self.camera.camera_width() ||
           self.current_y >= *self.camera.camera_height() {
            return None
        }

        let pixel = RtPixel::new(self.current_x, self.current_y);

        // Compute next pixel position
        if self.current_x >= self.camera.camera_width() - 1 {
            // Pick first pixel of the bottom line
            self.current_x  = 0;
            self.current_y += 1;
        } else {
            self.current_x += 1;
        }
        
        Some(pixel)
    }
}

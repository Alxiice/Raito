/// =====================================================
///                    Raito Render
/// 
/// Module authors : 
/// - Alice Sonolet <alice.sonolet@gmail.com>
/// 
/// Module description :
///   Defines a camera
/// =====================================================

use crate::rt_types::*;
use crate::rt_ray::*;
use crate::rt_shader_globals::*;
use std::cell::OnceCell;
use log::*;

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
    /// In this first impelemntation the height of the camera is always 1.0
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
    fn get_camera_ray(&self, x: u16, y: u16) -> RtRay {
        // Get a point on the viewport
        // Start with top left
        let mut viewport_point = self.top_left();
        // Offset with the given pixel
        viewport_point.x += (self.aspect_ratio / *self.camera_width()  as f32) * (x as f32 + 0.5);
        viewport_point.y += (1.0               / *self.camera_height() as f32) * (y as f32 + 0.5);
        let direction = viewport_point - self.center;
        // Create shader globals
        let sg = RtShaderGlobals::default(x, y);
        // Create the ray from the center
        let mut ray = RtRay::new(sg, self.center, direction.normalize());
        ray.bounces = 0;
        ray
    }
}


/// Could be reimplemented as trait to iterate in 
/// - buckets
/// - also not top left to bottom right but allow circular, etc
pub struct RtCameraRayIterator {
    // Stop condition
    stop: bool,
    /// Column position
    current_x: u16,
    /// Row position
    current_y: u16,
    // Camera
    camera: RtCamera,
}

impl RtCameraRayIterator {
    /// Creates an iterator for camera rays
    pub fn new(camera: RtCamera) -> Self {
        Self {
            stop: false,
            current_x: 0,
            current_y: 0,
            camera,
        }
    }
}

impl Iterator for RtCameraRayIterator {
    type Item = RtRay;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None
        }
        let camera_ray = self.camera.get_camera_ray(self.current_x, self.current_y);

        // Compute next pixel position
        if self.current_x >= self.camera.camera_width() - 1 {
            // Cannot pick right pixel
            if self.current_y >= self.camera.camera_height() - 1 {
                // Cannot pick the next row
                self.stop = true;
            } else {
                // Pick first pixel of the bottom line
                self.current_x  = 0;
                self.current_y += 1;
            }
        } else {
            self.current_x += 1;
        }
        Some(camera_ray)
    }
}

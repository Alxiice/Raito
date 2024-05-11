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
use std::cell::OnceCell;

/// Describes a camera
/// 
/// Right handed system
pub struct RtCamera {
    pub center: RtPoint3,
    pub focal_length: f32,
    pub aspect_ratio: f32,
    camera_width : OnceCell<u16>,
    camera_height: OnceCell<u16>,
}

impl RtCamera {
    /// Creates a new camera
    pub fn new(camera_width: u16) -> Self {
        let aspect_ratio = 1.0;
        Self { 
            center: RtPoint3::default(),
            focal_length: 1.0,
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
            z: -(self.focal_length)
        }
    }

    /// Get camera ray
    /// 
    /// (x: column, y: row) : pixel position
    fn get_camera_ray(&self, x: u16, y: u16) -> RtRay {
        // Get a point on the viewport
        // Start with top left
        let mut viewport_point = self.top_left();
        // Offset with the given pixel
        viewport_point.x += self.aspect_ratio * (x as f32);
        viewport_point.y += 1.0 * y as f32;
        let direction = viewport_point - self.center;
        // Create the ray from the center
        RtRay {
            origin: self.center,
            dir: direction.normalize()
        }
    }
}

/// Could be reimplemented as trait to iterate in 
/// - buckets
/// - also not top left to bottom right but allow circular, etc
struct RtCameraRayIterator {
    /// Column position
    current_x: u16,
    /// Row position
    current_y: u16,
    camera: RtCamera
}

impl RtCameraRayIterator {
    /// Creates an iterator for camera rays
    fn new(camera: RtCamera) -> Self {
        Self {
            current_x: 0,
            current_y: 0,
            camera
        }
    }
}

impl Iterator for RtCameraRayIterator {
    type Item = RtRay;

    fn next(&mut self) -> Option<Self::Item> {
        // Compute next pixel position
        if self.current_x >= self.camera.camera_width() - 1 {
            // Cannot pick right pixel
            if self.current_y >= self.camera.camera_height() - 1 {
                // Cannot pick the next row
                return None
            } else {
                // Pick first pixel of the bottom line
                self.current_x  = 0;
                self.current_y += 1;
            }
        } else {
            self.current_x += 1;
        }
        Some(self.camera.get_camera_ray(self.current_x, self.current_y))
    }
}

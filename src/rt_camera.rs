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

pub const RT_MAX_BOUNCES: u8 = 20;
pub const NB_SUBPIXELS: u8 = 150;
pub const INV_NB_SUBPIXELS: f32 = 1.0 / (NB_SUBPIXELS as f32);
pub const SUBPX_SEED: u64 = 1;

/// Describes a camera
/// 
/// Right handed system
pub struct RtCamera {
    // pub aspect_ratio: f32,
    pub image_width : u16, // OnceCell<u16>
    pub image_height: u16,
    // pub lookfrom: RtPoint3,
    center: RtPoint3,
    pixel00_loc: RtPoint3,
    pixel_delta_u: RtVec3,
    pixel_delta_v: RtVec3,
    // pub vfov: f32,
    // lookat: RtPoint3,
    // vup: RtVec3,
    // u: RtVec3, v: RtVec3, w: RtVec3,
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * RT_PI / 180.0
}

impl RtCamera {
    /// Creates a new camera
    pub fn new(aspect_ratio: f32, image_width: u16, vfov: f32, 
               lookfrom: RtPoint3, lookat: RtPoint3, vup: RtVec3) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as u16;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let center = lookfrom;

        // Determine viewport dimensions.
        let focal_length = (lookfrom - lookat).length();
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (lookfrom - lookat).normalize();
        let u = (RtVec3::cross(vup, w)).normalize();
        let v = RtVec3::cross(w, u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (focal_length * w).to_point3() - viewport_u/2.0 - viewport_v/2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        let pixel00_loc = pixel00_loc.to_point3();

        // camera_width: OnceCell::from(camera_width),
        Self {
            // aspect_ratio, 
            image_width, image_height, 
            center, 
            // vfov, 
            pixel00_loc, pixel_delta_u, pixel_delta_v,
            // lookfrom, 
            // lookat, 
            // vup,
            // u, v, w
        }
    }

    /// Sample camera ray
    /// 
    /// We shoot the ray at the center of the pixel for each pixel in the grid
    /// 
    /// (x: column, y: row) : pixel position
    pub fn get_camera_ray(&self, x: u16, y: u16, px: f32, py: f32) -> RtRay {
        let pixel_center = self.pixel00_loc + 
            ((x as f32 + px) * self.pixel_delta_u) + 
            ((y as f32 + py) * self.pixel_delta_v);
        let ray_direction = pixel_center - self.center;
        // Create shader globals
        let sg = RtShaderGlobals::default(x, y);
        // Create the ray from the center
        let mut ray = RtRay::new(&sg, self.center, ray_direction.normalize());
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
        if self.current_x >= self.camera.image_width ||
           self.current_y >= self.camera.image_height {
            return None
        }

        let pixel = RtPixel::new(self.current_x, self.current_y);

        // Compute next pixel position
        if self.current_x >= self.camera.image_width - 1 {
            // Pick first pixel of the bottom line
            self.current_x  = 0;
            self.current_y += 1;
        } else {
            self.current_x += 1;
        }
        
        Some(pixel)
    }
}

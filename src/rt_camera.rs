/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines a camera
/// =====================================================

use crate::rt_types::*;
use crate::rt_ray::*;
use crate::rt_shader_globals::*;
use crate::rt_sampler::*;

/// Describes a camera
/// 
/// Right handed system
pub struct RtCamera {
    pub image_width : u16, 
    pub image_height: u16,
    center: RtPoint3,
    pixel00_loc: RtPoint3,
    pixel_delta_u: RtVec3,
    pixel_delta_v: RtVec3,

    // Additional parameters that are not used
    pub _vfov: f32,
    pub _look_from: RtPoint3,
    pub _look_at: RtPoint3,
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

        Self {
            image_width, image_height, center, 
            pixel00_loc, pixel_delta_u, pixel_delta_v,
            _vfov: vfov, _look_from: lookfrom, _look_at: lookat
        }
    }

    fn get_width(&self) -> u16 {
        self.image_width
    }

    fn get_height(&self) -> u16 {
        self.image_height
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
//  Buckets
// ========================================

#[derive(PartialEq)]
pub enum RtBucketMode {
    BUCKET_MODE_TOP,
    BUCKET_MODE_SPIRAL
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct RtRenderBucket {
    top_coordinate: u16,
    left_coordinate: u16,
    width: u16,
    height: u16,
    samples_nb: u16
}

impl RtRenderBucket {
    pub fn new(top: u16, left: u16, width: u16, height: u16, samples_nb: u16) -> RtRenderBucket {
        RtRenderBucket { top_coordinate: top, left_coordinate: left, width, height, samples_nb }
    }
}

impl Ord for RtRenderBucket {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.samples_nb == other.samples_nb {
            std::cmp::Ordering::Equal
        } else if self.samples_nb > other.samples_nb {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}

impl RtRenderBucket {
    fn get_top_buckets(camera: &RtCamera, bucket_size: [u16; 2]) -> Vec<Self> {
        let mut bucket_list = Vec::new();
        let rem_buckets = [
            camera.get_width()  % bucket_size[0],
            camera.get_height() % bucket_size[1]
        ];
        let nb_buckets = [
            camera.get_width() / bucket_size[0]  + if rem_buckets[0] > 0 { 1 } else { 0 },
            camera.get_height() / bucket_size[1] + if rem_buckets[1] > 0 { 1 } else { 0 }
        ];
        for bucket_y in 0..nb_buckets[1] {          // For each line
            for bucket_x in 0..nb_buckets[0] {      // For each column
                let mut size_x = bucket_size[0];
                let mut size_y = bucket_size[1];
                if bucket_x == nb_buckets[0] - 1 && rem_buckets[0] > 0 {
                    size_x = rem_buckets[0];
                }
                if bucket_y == nb_buckets[1] - 1 && rem_buckets[1] > 0 {
                    size_y = rem_buckets[1];
                }
                bucket_list.push(RtRenderBucket::new(
                    bucket_size[0] * bucket_x, bucket_size[1] * bucket_y, size_x, size_y, 0
                ));
            }
        }
        bucket_list
    }

    fn get_spiral_buckets(camera: &RtCamera, bucket_size: [u16; 2]) -> Vec<Self> {
        panic!("Bucket mode spiral not impelmented yet !");
        Vec::new()
    }

    pub fn get_bucket_list(camera: &RtCamera, mode: RtBucketMode, bucket_size: [u16; 2]) -> Vec<Self> {
        match mode {
            RtBucketMode::BUCKET_MODE_TOP => return Self::get_top_buckets(camera, bucket_size),
            RtBucketMode::BUCKET_MODE_SPIRAL => return Self::get_spiral_buckets(camera, bucket_size),
            _ => panic!("Bucket mode unknown")
        }
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
            self.x, self.y,
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

pub struct RtBucketRayIterator<'a> {
    bucket: &'a RtRenderBucket,
    /// Current position
    current_x: u16,
    current_y: u16,
}

impl<'a> RtBucketRayIterator<'a> {
    /// Creates an iterator for camera rays
    pub fn new( bucket: &'a RtRenderBucket) -> Self {
        Self { bucket, current_x: 0, current_y: 0 }
    }
}

impl<'a> Iterator for RtBucketRayIterator<'a> {
    type Item = RtPixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_x >= self.bucket.width ||
           self.current_y >= self.bucket.height {
            return None
        }

        let pixel = RtPixel::new(
            self.current_x + self.bucket.top_coordinate, 
            self.current_y + self.bucket.left_coordinate
        );

        // Compute next pixel position
        if self.current_x >= self.bucket.width - 1 {
            // Pick first pixel of the bottom line
            self.current_x  = 0;
            self.current_y += 1;
        } else {
            self.current_x += 1;
        }
        
        Some(pixel)
    }
}
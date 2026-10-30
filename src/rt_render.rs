use std::borrow::BorrowMut;
/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Implements the ray tracing function.
/// =====================================================

use std::f32::NAN;
use std::collections::{BinaryHeap, VecDeque};
use std::sync::{Mutex, Condvar};
use std::thread::JoinHandle;
use std::{ thread, time::Duration };
use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard, MutexGuard};

use egui::output;
use log::{ info, debug, warn };

use crate::rt_types::*;
use crate::rt_camera::*;
use crate::rt_ray::*;
use crate::rt_shader_globals::*;
use crate::rt_scene::*;
use crate::RtVec3;
use crate::rt_objects::rt_object_base::*;
use crate::rt_render_output::RtRenderResult;


// ========================================
//  Creating rays
// ========================================

/// Utility function to form a new ray from a shading point
pub fn RtMakeRay(sg: &RtShaderGlobals, _raytype: RtRayType, dir: RtVec3, _maxdist: f32) -> RtRay {
    RtRay { 
        origin: sg.P, 
        dir,
        bounces: sg.bounces + 1, 
        x: sg.x, 
        y: sg.y
    }
}

/// Launch a ray on a scene
pub fn RtReflectRay(ray: &mut RtRay, wo: &RtVec3, normal: &RtVec3, _sg: &RtShaderGlobals) {
    ray.dir = (*wo - 2.0 * RtVec3::dot(*wo, *normal) * *normal).normalize();
}

// TODO : fix to avoid front facing normal ?
pub fn RtRefractRay(ray: &mut RtRay, wo: &RtVec3, normal: &RtVec3, eta: f32, _sg: &RtShaderGlobals) {
    let cos_theta = RtVec3::dot(-*wo, *normal).min(1.0);
    let r_out_perp: RtVec3 = eta * (*wo + cos_theta * *normal);
    let r_out_parallel: RtVec3 = - (1.0 - r_out_perp.length_squared()).abs().sqrt() * *normal;
    ray.dir = (r_out_perp + r_out_parallel).normalize();
}

// ========================================
//  Launching rays
// ========================================

/// Launch a ray on a scene
pub fn RtTraceRay(scene: &RtScene, ray: &RtRay) -> Option<RtHit> {
    if ray.bounces >= scene.settings.max_bounces {
        return None
    }

    let mut min_dist: f32 = NAN;
    let mut first_hit: Option<RtRayHit> = None;
    let mut first_hit_object: Option<&Box<dyn RtObject>> = None;
    
    // Shapes : Find closest hit point & object
    let geometry = scene.list_shapes();
    for shape in geometry {
        // Compute intersections
        let hit = shape.get_intersection(ray);
        // Execute shader
        if hit.is_some() {
            let hitSg = hit.unwrap();
            if hitSg.dist < min_dist || min_dist.is_nan() {
                min_dist = hitSg.dist;
                first_hit = Some(hitSg);
                first_hit_object = Some(shape);
            }
        }
    }

    // Lights : Find closest hit point & object
    // let lights = scene.list_lights();
    // for light in lights {
    //     // Compute intersections
    //     let hit = light.get_intersection(ray);
    //     // Execute shader
    //     if hit.is_some() {
    //         let hitSg = hit.unwrap();
    //         if hitSg.dist < min_dist || min_dist.is_nan() {
    //             min_dist = hitSg.dist;
    //             first_hit = Some(hitSg);
    //             first_hit_object = Some(light);
    //         }
    //     }
    // }
    
    // Execute shader on closest hit and return hit result
    if first_hit_object.is_some() {
        let hit = first_hit.unwrap();
        let hit_sg = first_hit_object.unwrap().get_sg(ray, &hit);
        let hit_point = hit.P.unwrap();
        let color = first_hit_object.unwrap().get_shader().evaluate(scene, &hit_sg);

        Some( RtHit::new(true, color, hit_point) )
    } else {
        // None
        let a = 0.5 * ray.dir.y + 1.0;
        let skyColor = (1.0 - a) * RtRGBA::WHITE + a * RtRGBA::from_rgb(0.5, 0.7, 1.0);
        Some(RtHit::new(false, skyColor, RtPoint3::default()))
    }
}


/// Launch to lights
pub fn RtTraceToLights(scene: &RtScene, ray: &RtRay) -> Option<RtHit> {
    
    let mut min_dist: f32 = NAN;
    let mut first_hit: Option<RtRayHit> = None;
    let mut first_hit_object: Option<&Box<dyn RtObject>> = None;
    
    // Find closest hit point & object
    let lights = scene.list_lights();
    for light in lights {
        // Compute intersections
        let hit = light.get_intersection(ray);
        // Execute shader
        if hit.is_some() {
            let hitSg = hit.unwrap();
            if hitSg.dist < min_dist || min_dist.is_nan() {
                min_dist = hitSg.dist;
                first_hit = Some(hitSg);
                first_hit_object = Some(light);
            }
        }
    }
    
    // Execute shader on closest hit and return hit result
    if first_hit_object.is_some() {
        if first_hit_object.unwrap().get_type().as_str() == "light" {
            let hit = first_hit.unwrap();
            let hit_sg = first_hit_object.unwrap().get_sg(ray, &hit);
            let hit_point = hit.P.unwrap();
            let color = first_hit_object.unwrap().get_shader().evaluate(scene, &hit_sg);
            // TODO : attenuation
            Some( RtHit::new(true, color, hit_point) )
        } else {
            // Not a light : masked
            None
        }
    } else {
        None
    }
}


// ========================================
//  Render Queue
// ========================================

pub struct RtRenderQueue {
    data: Mutex<BinaryHeap<RtRenderBucket>>,
    cv: Condvar,
    remaining_buckets: u16
}

impl RtRenderQueue {
    const MAX_TRHEADS: usize = 8;

    fn new(scene: &RtScene) -> Self {
        let mode = RtBucketMode::BUCKET_MODE_TOP;
        let bucket_size = [100, 100];
        let buckets = RtRenderBucket::get_bucket_list(scene.get_camera(), mode, bucket_size);
        let nb_buckets = buckets.len();
        let mut queue = Self { 
            data: Mutex::new(BinaryHeap::from(buckets)),
            cv: Condvar::new(),
            remaining_buckets: nb_buckets as u16
        };
        queue
    }

    /// Adds an element to the back of the queue
    fn push(&self, value: RtRenderBucket) {
        let mut data = self.data.lock().unwrap();
        data.push(value);
        self.cv.notify_one();
    }
    
    /// Removes an element from the front of the queue
    fn pop(&self) -> RtRenderBucket {
        let mut data = self.data.lock().unwrap();
        // wait for the notification if the queue is empty
        while data.is_empty() {
            data = self.cv.wait(data).unwrap();
        }
        data.pop().unwrap()
    }
    
    fn len(&self) -> usize {
        let data = self.data.lock().unwrap();
        data.len()
    }
    
    fn is_empty(&self) -> bool {
        let data = self.data.lock().unwrap();
        data.is_empty()
    }
}


// ========================================
//  Render Thread Stack
// ========================================

struct RtThreadsStack {
    threads: VecDeque<(JoinHandle<()>)>,
}

impl RtThreadsStack {
    const MAX_TRHEADS: usize = 8;

    fn new(width: usize, height: usize) -> Self {
        Self {
            threads: VecDeque::new(),
        }
    }

    fn add(&mut self, handle: JoinHandle<()>) {
        self.threads.push_back(handle);
    }

    fn is_full(&self) -> bool {
        self.threads.len() > Self::MAX_TRHEADS
    }

    fn is_empty(&self) -> bool {
        self.threads.len() == 0
    }

    fn wait_for_thread(&mut self, tid: usize) -> bool {
        let thread = self.threads.remove(tid);
        if thread.is_none() {
            true
        } else {
            let thread = thread.unwrap();
            if thread.is_finished() {
                debug!("Waiting for thread {tid}");
                thread.join().unwrap();
                // debug!("Thread {tid} finished");
                true
            } else {
                // Put the thread in the queue again
                self.threads.insert(tid, thread);
                false
            }
        }
    }

    fn wait_for_free(&mut self) -> bool {
        if self.threads.len() < Self::MAX_TRHEADS {
            return true;
        }
        for tid in 0..Self::MAX_TRHEADS {
            if self.wait_for_thread(tid) {
                return true
            }
        }
        false
    }

    fn join(&mut self) {
        for tid in 0..Self::MAX_TRHEADS {
            let thread = self.threads.remove(tid);
            if thread.is_some() {
                debug!("Wait for thread {}", tid);
                let thread = thread.unwrap();
                thread.join().unwrap();
            }
        }
        debug!("Finished threads !");
    }
}


// ========================================
//  Launch render
// ========================================

fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

/// Render a bucket from the scene
fn RtRenderBucket(scene: &RtScene, mut bucket: &mut RtRenderBucket) {
    debug!("Bucket : {bucket}");
    let cam_rays = RtBucketRayIterator::new(&bucket);
    for pixel in cam_rays {
        let mut pixelColor = RtRGBA::BLACK;
        // TODO
        // If we are in progressive mode compute only 1 SPP per bucket
        // if we are not, compute all SPP on the first bucket
        let spp_nb = scene.settings.render_spp;
        for _ in 0..spp_nb {
            let ray = pixel.get_ray(
                scene.get_camera(), 
                [
                    bucket.left_coordinate, // Offset for X coordinate
                    bucket.top_coordinate   // Offset for Y coordinate
                ]
            );
            let hit = RtTraceRay(scene, &ray);
            if hit.is_some() {
                let hitResult = hit.unwrap();
                pixelColor += hitResult.colorOutput * scene.settings.inv_nb_spp;
            } else {
                pixelColor += RtRGBA::ERRCOLOR  * scene.settings.inv_nb_spp;
            }
        }
        bucket.write_pixel(pixel.pixel_x(), pixel.pixel_y(), pixelColor);
    }
}

pub fn RtRenderScene(scene: RtScene, result: &mut RtRenderResult) {
    // Build the list of buckets
    debug!("Creating bucket queue");
    let mut bucket_queue = Arc::new(RtRenderQueue::new(&scene));
    debug!("Bucket queue created with {} elements", bucket_queue.len());

    let scene = Arc::new(scene);
    
    let mut threads = RtThreadsStack::new(result.width, result.height);
    let mut final_image = Arc::new(Mutex::new(
        RtRenderResult::new(result.width, result.height, 0, 0)
    ));
    while !bucket_queue.is_empty() {
        if threads.is_full() {
            if !threads.wait_for_free() {
                continue;
            }
        }
        let now = std::time::Instant::now();
        let t_scene = scene.clone();
        let mut t_buckets_queue = bucket_queue.clone();
        let mut t_final_image = final_image.clone();
        let render_th = thread::spawn(move || {
            let mut render_bucket = t_buckets_queue.pop();
            RtRenderBucket(&t_scene, &mut render_bucket);
            let bucket_result = render_bucket.result;
            // Add result to final image
            let mut thread_output = t_final_image.lock().unwrap();
            for x in 0..bucket_result.width {
                for y in 0..bucket_result.height {
                    let add_color = bucket_result.get_pixel_color(x, y);
                    thread_output.add_pixel_color(
                        x + bucket_result.x_offset,
                        y + bucket_result.y_offset,
                        add_color
                    );
                }
            }
        });
        threads.add(render_th);
    }
    info!("Joining...");
    threads.join();

    let final_output = final_image.lock().unwrap();
    for x in 0..result.width {
        for y in 0..result.height {
            let color = final_output.get_pixel_color(x, y);
            let ccColor = RtRGBA::from_rgb(
                linear_to_gamma(color.r), 
                linear_to_gamma(color.g), 
                linear_to_gamma(color.b) 
            );
            result.set_pixel_color(x, y, ccColor);
        }
    }
}

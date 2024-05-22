/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines render scene and methods to launch render.
/// =====================================================

use egui::Color32;
use std::marker::PhantomData;
use log::*;

// use crate::{RtCamera, RtPoint3, RtRGBA};
use crate::rt_types::*;
use crate::rt_camera::*;
use crate::rt_ray::*;
use crate::rt_shader_globals::*;
use crate::rt_objects::*;
use crate::rt_objects::rt_object_base::*;
use crate::rt_shaders::*;

use self::staticColor::StaticColorShader;
use self::stateVector::StateVectorShader;
use self::rt_geometries::RtSphere;


pub struct RenderResult {
    width: u16,
    height: u16,
    /// Array of array of color
    /// To access : render_grid[col][row] -> index from top left to bottom right
    pub render_grid: Vec<Vec<RtRGBA>>,
}

impl RenderResult {
    pub fn new() -> Self {
        let width = 400;
        let height = 400;
        let mut render = Self {
            width: width,
            height: height,
            render_grid: Vec::with_capacity(usize::from(width))
        };
        for y in 0..height {
            // Add a row with known size
            render.render_grid.push(Vec::with_capacity(usize::from(render.height)));
            // For each cell add color
            for _ in 0..render.width {
                render.render_grid[usize::from(y)].push(RtRGBA::default())
            }
        }
        render
    }

    pub fn set_pixel_color(&mut self, x: usize, y: usize, color: RtRGBA) {
        let y = usize::from(self.height) - (y + 1);  // Add 1 for index 0
        // TODO : is rendergrid[y][x] correct ?
        self.render_grid[usize::from(y)][usize::from(x)] = color;
    }

    pub fn get_pixel_color(&mut self, x: usize, y: usize) -> Color32 {
        let color = self.render_grid[usize::from(y)][usize::from(x)];
        Color32::from_rgb(color.r(), color.g(), color.b())
    }
}

pub struct RenderScene<'a> {
    // Camera params
    pub camera_fov: f32,
    // Light params
    pub light_intensity: f32,
    pub light_color: RtRGBA,
    // Sphere params
    pub sphere_color: RtRGBA,
    pub sphere_center: RtPoint3,
    pub sphere_radius: f32,

    /// Stores result
    pub result: RenderResult,

    // To use lifetime
    _marker: PhantomData<&'a ()>,
}

impl<'a> Default for RenderScene<'a> {
    fn default() -> Self {
        Self {
            // Camera params
            camera_fov: 1.0,
            // Light params
            light_intensity: 1.0,
            light_color: RtRGBA::default(),
            // Sphere params
            sphere_color: RtRGBA::default(),
            sphere_center: RtPoint3::default(),
            sphere_radius: 0.0,

            // Result
            result: RenderResult::new(),

            _marker: Default::default()
        }
    }
}

impl<'a> RenderScene<'a> {
    /// Update scene parameters
    pub fn setup_scene(&mut self, 
        camera_fov: f32, 
        light_intensity: f32, 
        light_color: RtRGBA,
        sphere_color: RtRGBA,
        sphere_center: RtPoint3,
        sphere_radius: f32)
    {
        // Camera params
        self.camera_fov = camera_fov;
        // Light params
        self.light_intensity = light_intensity;
        self.light_color     = light_color;
        // Sphere params
        self.sphere_color  = sphere_color;
        self.sphere_center = sphere_center;
        self.sphere_radius = sphere_radius;
    }

    // TODO : from one to multiple objects
    pub fn get_scene_geometry(&self) -> Box<dyn RtObject<'static>> {
        let shader = StateVectorShader { output: "N".to_string() };
        // let shader = StaticColorShader { color: self.sphere_color };
        // Define geometry
        let sphere = RtSphere {
            object_params: ObjectParams {
                name: "sphere".to_string(),
                shader: Box::new(shader)
            },
            center: self.sphere_center,
            radius: self.sphere_radius
        };

        Box::new(sphere)
    }

    fn RtTraceRay(&mut self, ray: &RtRay) -> Option<RtHit> {
        let object = self.get_scene_geometry();
        // TODO : from one to multiple objects

        // Compute intersections
        let hit: Option<RtShaderGlobals> = object.intersect(ray);

        // Execute shader
        if hit.is_some() {
            return Some(
                RtHit::new(true, object.get_shader().evaluate(&hit.unwrap()))
            );
        }
        None
    }

    /// Launch render
    pub fn render(&mut self) {
        // TODO : for now the camera
        // - center is at 0
        // - direction is towards the -y direction
        // 
        // We want to be able to change that, move and rotate the camera
        // We need to implement world and camera space

        let mut camera = RtCamera::new(self.result.width, 1.0);
        camera.camera_fov = self.camera_fov;
        let cam_rays = RtCameraRayIterator::new(camera);
        for camera_ray in cam_rays {
            let ray = &camera_ray;  // Reference to the camera ray
            let hit = self.RtTraceRay(ray);
            if hit.is_some() {
                let hitResult = hit.unwrap();
                self.result.set_pixel_color(
                    usize::from(ray.x), usize::from(ray.y), hitResult.colorOutput);
            } else {
                self.result.set_pixel_color(
                    usize::from(ray.x), usize::from(ray.y), self.light_color);
            }
        }
    }
}

/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Launch render on scene 
/// =====================================================

use egui::Color32;
use log::warn;

use crate::rt_types::*;
use crate::rt_scene::*;
use crate::rt_camera::*;
use crate::rt_render::*;


/// Holds the result from the render
pub struct RenderResult {
    width: u16,
    height: u16,
    /// Array of array of color
    /// To access : render_grid[col][row] -> index from top left to bottom right
    pub render_grid: Vec<Vec<RtRGBA>>,
}

impl RenderResult {
    /// Creates a new render result with black pixels
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

    /// Utility function to set the color of a pixel
    pub fn set_pixel_color(&mut self, x: usize, y: usize, color: RtRGBA) {
        let y = usize::from(self.height) - (y + 1);  // Add 1 for index 0
        // TODO : is rendergrid[y][x] correct ?
        self.render_grid[usize::from(y)][usize::from(x)] = color;
    }

    /// Utility function to query the color of a pixel
    pub fn rt_get_pixel_color(&mut self, x: usize, y: usize) -> RtRGBA {
        self.render_grid[usize::from(y)][usize::from(x)]
    }
    
    /// Utility function to query the color of a pixel
    pub fn get_pixel_color(&mut self, x: usize, y: usize) -> Color32 {
        let color = self.render_grid[usize::from(y)][usize::from(x)];
        color.to_color32()
    }
}


/// Launch render on scene
pub fn RtRenderScene(scene: &mut RtScene, result: &mut RenderResult) {
    // TODO : for now the camera
    // - center is at 0
    // - direction is towards the -y direction
    // 
    // We want to be able to change that, move and rotate the camera
    // We need to implement world and camera space

    let mut camera = RtCamera::new(result.width, 1.0);
    camera.camera_fov = scene.camera_fov;
    let cam_rays = RtCameraRayIterator::new(camera);
    for camera_ray in cam_rays {
        let x = camera_ray.x();
        let y = camera_ray.y();
        let mut pixelColor = RtRGBA::BLACK;
        for ray in camera_ray {
            let hit = RtTraceRay(scene, &ray);
            if hit.is_some() {
                let hitResult = hit.unwrap();
                pixelColor += hitResult.colorOutput / (NB_SUBPIXELS as f32);
            } else {
                pixelColor += RtRGBA::ERRCOLOR / (NB_SUBPIXELS as f32);
            }
        }
        result.set_pixel_color(x, y, pixelColor);
    }
}

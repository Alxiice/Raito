/// =====================================================
///                    Raito Render
/// 
/// Module authors : 
/// - Alice Sonolet <alice.sonolet@gmail.com>
/// 
/// Module description :
///   Defines render scene and methods to launch render.
/// =====================================================

use crate::RtRGB;
use egui::Color32;
use log::*;

pub struct RenderResult {
    width: u16,
    height: u16,
    /// Array of array of color
    /// To access : render_grid[col][row] -> index from top left to bottom right
    pub render_grid: Vec<Vec<RtRGB>>,
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
        for y in 0..width {
            // Add a row with known size
            render.render_grid.push(Vec::with_capacity(usize::from(render.height)));
            // For each cell add color
            for _ in 0..render.height {
                render.render_grid[usize::from(y)].push(RtRGB::default())
            }
        }
        render
    }

    pub fn set_pixel_color(&mut self, y: usize, x: usize, color: RtRGB) {
        self.render_grid[usize::from(y)][usize::from(x)] = color;
    }

    pub fn get_pixel_color(&mut self, y: usize, x: usize) -> Color32 {
        let color = self.render_grid[usize::from(y)][usize::from(x)];
        Color32::from_rgb(color.r(), color.g(), color.b())
    }
}

pub struct RenderScene {
    // Declare here attributes 

    // Tmp parameters (implementation step one)
    pub color: RtRGB,
    pub light_intensity: f32,

    /// Stores result
    pub result: RenderResult
}

impl Default for RenderScene {
    fn default() -> Self {
        Self {
            color: RtRGB::default(),
            light_intensity: 0.0,
            result: RenderResult::new()
        }
    }
}

impl RenderScene {
    /// Update scene parameters
    pub fn setup_scene(&mut self, color: RtRGB, light_intensity: f32) {
        self.color = color;
        self.light_intensity = light_intensity;
    }

    /// Launch render
    pub fn render(&mut self) {
        // Fill to color
        for y in 0..self.result.width {
            for x in 0..self.result.height {
                self.result.set_pixel_color(usize::from(y), usize::from(x), self.color);
            }
        }
    }
}

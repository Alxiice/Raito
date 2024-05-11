/// =====================================================
///                    Raito Render
/// 
/// Module authors : 
/// - Alice Sonolet <alice.sonolet@gmail.com>
/// 
/// Module description :
///   Defines render scene and methods to launch render.
/// =====================================================

use crate::{RtPoint3, RtRGBA};
use egui::Color32;
use log::*;

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

    pub fn set_pixel_color(&mut self, y: usize, x: usize, color: RtRGBA) {
        self.render_grid[usize::from(y)][usize::from(x)] = color;
    }

    pub fn get_pixel_color(&mut self, y: usize, x: usize) -> Color32 {
        let color = self.render_grid[usize::from(y)][usize::from(x)];
        Color32::from_rgb(color.r(), color.g(), color.b())
    }
}

pub struct RenderScene {
    // Tmp parameters (implementation step one)
    pub center: RtPoint3,
    pub radius: f32,
    pub color: RtRGBA,

    /// Stores result
    pub result: RenderResult
}

impl Default for RenderScene {
    fn default() -> Self {
        Self {
            center: RtPoint3::default(),
            radius: 0.0,
            color: RtRGBA::default(),
            result: RenderResult::new()
        }
    }
}

impl RenderScene {
    /// Update scene parameters
    pub fn setup_scene(&mut self, center: RtPoint3, radius: f32, color: RtRGBA) {
        self.center = center;
        self.radius = radius;
        self.color = color;
    }

    fn render_pixel(&mut self, y: usize, x: usize) {
        self.result.set_pixel_color(y, x, self.color);
        let center = [self.result.width as f32 / 2.0, self.result.height as f32 / 2.0];
        let radius = self.radius;
        if (y as f32 - center[0]).powf(2.0) + (x as f32 - center[1]).powf(2.0) < radius.powf(2.0) {
            self.result.set_pixel_color(y, x, RtRGBA::RED);
        }
    }

    /// Launch render
    pub fn render(&mut self) {
        // Fill to color
        for y in 0..self.result.height {
            for x in 0..self.result.width {
                self.render_pixel(usize::from(y), usize::from(x));
            }
        }
    }
}

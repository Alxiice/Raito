/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Handle render result & output
///   We can write tools to export image here 
/// =====================================================

use egui::Color32;
use log::error;

use crate::rt_types::*;

/// Holds the result from the render
#[derive(PartialEq, Eq, PartialOrd)]
pub struct RtRenderResult {
    pub width: usize,
    pub height: usize,
    /// Array of array of color
    /// To access : render_grid[col][row] -> index from top left to bottom right
    render_grid: Vec<Vec<RtRGBA>>,
}

impl RtRenderResult {
    /// Creates a new render result with black pixels
    pub fn new(width: usize, height: usize) -> Self {
        let mut render = Self {
            width,
            height,
            render_grid: Vec::with_capacity(usize::from(width))
        };
        for y in 0..height {
            // Add a row with known size
            render.render_grid.push(Vec::with_capacity(height));
            // For each cell add color
            for _ in 0..width {
                render.render_grid[y].push(RtRGBA::default())
            }
        }
        render
    }

    /// Utility function to set the color of a pixel
    pub fn set_pixel_color(&mut self, x: usize, y: usize, color: RtRGBA) {
        self.render_grid[y][x] = color;
    }

    /// Utility function to query the color of a pixel
    pub fn rt_get_pixel_color(&self, x: usize, y: usize) -> RtRGBA {
        self.render_grid[y][x]
    }
    
    /// Utility function to query the color of a pixel
    pub fn get_pixel_color(&self, x: usize, y: usize) -> Color32 {
        let color = self.render_grid[y][x];
        color.to_color32()
    }

    pub fn export_as_ppm(&self) {
        // TODO
        error!("Function export_as_ppm not implemented yet");
    }
}

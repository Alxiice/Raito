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
    // Top left offset
    pub x_offset: usize,
    pub y_offset: usize,
    /// Array of array of color
    /// To access : render_grid[col][row] -> index from top left to bottom right
    render_grid: Vec<Vec<RtRGBA>>,
}

impl RtRenderResult {
    /// Creates a new render result with black pixels
    pub fn new(width: usize, height: usize, x_offset: usize, y_offset: usize) -> Self {
        let mut render = Self {
            width, height, x_offset, y_offset, 
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
    
    pub fn add_pixel_color(&mut self, x: usize, y: usize, color: RtRGBA) {
        self.render_grid[y][x] += color;
    }

    /// Utility function to query the color of a pixel
    /// - inputs x & y are relative to the full image
    /// - However here we can store only a portion of the image
    /// - Therefore the offset is applied, but users must beware of using this fn
    pub fn get_offset_pixel_color(&self, x: usize, y: usize) -> RtRGBA {
        self.render_grid[y - self.y_offset][x - self.x_offset]
    }

    /// Get the pixel color
    /// Warning ! Use only when the image top & left is 0
    pub fn get_pixel_color(&self, x: usize, y: usize) -> RtRGBA {
        self.render_grid[y][x]
    }
    
    /// Write the render as a binary PPM (P6) image
    /// Warning ! Use only when the image top & left is 0
    pub fn export_as_ppm(&self, path: &str) -> std::io::Result<()> {
        let mut data = format!("P6\n{} {}\n255\n", self.width, self.height).into_bytes();
        data.reserve(self.width * self.height * 3);
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.get_pixel_color(x, y).to_color32();
                data.extend_from_slice(&[color.r(), color.g(), color.b()]);
            }
        }
        std::fs::write(path, data)
    }
}

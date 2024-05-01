use std::borrow::Borrow;

use crate::RtRGB;

/// =====================================================
///                    Raito Render
/// 
/// Module authors : 
/// - Alice Sonolet <alice.sonolet@gmail.com>
/// 
/// Module description :
///   Defines render scene and methods to launch render.
/// =====================================================


pub struct RenderScene {
    // Declare here attributes 
    // _scene_descriptor_path: String,

    // View size
    width: u16,
    height: u16,

    // Tmp parameters (implementation step one)
    pub color: RtRGB,
    pub light_intensity: f32,
}

impl Default for RenderScene {
    fn default() -> Self {
        Self {
            // _scene_descriptor_path: "".to_string(),
            width: 400,
            height: 400,
            color: RtRGB::default(),
            light_intensity: 0.0
        }
    }
}

pub struct RenderResult {
    /// Array of array of color
    /// To access : render_grid[col][row] -> index from top left to bottom right
    pub render_grid: Vec<Vec<RtRGB>>,
}

impl RenderResult {
    fn new() -> Self {
        Self {
            render_grid: Vec::new()
        }
    }

    fn init(&mut self, globals: &mut RenderScene) {
        // Realloc with known size
        self.render_grid = Vec::with_capacity(usize::from(globals.width));
        // For each column add row
        for y in 0..globals.width {
            // Add a row with known size
            self.render_grid.push(Vec::with_capacity(usize::from(globals.height)));
            // For each cell add color
            for _ in 0..globals.height {
                self.render_grid[usize::from(y)].push(RtRGB::default())
            }
        }
    }

    fn set_pixel_color(&mut self, y: usize, x: usize, color: RtRGB) {
        self.render_grid[usize::from(y)][usize::from(x)] = color;
    }
}

impl RenderScene {
    pub fn render(&mut self) -> RenderResult {
        let mut result = RenderResult::new();
        result.init(self);

        // Fill to color
        for y in 0..self.width {
            for x in 0..self.height {
                result.set_pixel_color(usize::from(y), usize::from(x), self.color);
            }
        }
        return result
    }
}

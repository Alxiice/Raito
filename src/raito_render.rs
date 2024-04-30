use std::borrow::Borrow;

use env_logger::fmt::style::Color;

pub struct RenderGlobals {
    // Declare here attributes 
    // _scene_descriptor_path: String,

    // View size
    length_h: u16,
    length_v: u16,

    // Tmp parameters (implementation step one)
    pub color: Vec<u8>,
    pub light_intensity: f32,
}

impl Default for RenderGlobals {
    fn default() -> Self {
        Self {
            // _scene_descriptor_path: "".to_string(),
            length_h: 400,
            length_v: 400,
            color: vec![0, 0, 0],
            light_intensity: 0.0
        }
    }
}

pub struct RenderResult {
    pub color: Vec<Vec<Vec<u8>>>  // array of array of color
}

impl RenderResult {
    fn new() -> Self {
        Self {
            color: Vec::new()
        }
    }

    fn init(&mut self, globals: &mut RenderGlobals) {
        // let mut result = Self {
        //     color: Vec::new()
        // };
        for y in 0..globals.length_h {
            self.color.push(Vec::new());
            for _ in 0..globals.length_v {
                self.color[usize::from(y)].push(vec![0, 0, 0])
            }
        }
    }

    fn set_pixel_color(&mut self, y: usize, x: usize, color: &Vec<u8>) {
        self.color[usize::from(y)][usize::from(x)][0] = color[0];
        self.color[usize::from(y)][usize::from(x)][1] = color[1];
        self.color[usize::from(y)][usize::from(x)][2] = color[2];
    }
}

impl RenderGlobals {
    pub fn render(&mut self) -> RenderResult {
        let mut result = RenderResult::new();
        result.init(self);

        // Fill to color
        for y in 0..self.length_h {
            for x in 0..self.length_v {
                result.set_pixel_color(usize::from(y), usize::from(x), &self.color);
            }
        }

        return result
    }
}
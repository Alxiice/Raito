/// =====================================================
///                    Raito Render
/// 
/// Module authors : 
/// - Alice Sonolet <alice.sonolet@gmail.com>
/// 
/// Module description :
///   Opens a render window to load a scene and launch a 
///   render.
/// =====================================================

use egui::*;
use log::*;

pub struct RtParameters {
    pub color: Color32,
    pub light_intensity: f32,
}

impl Default for RtParameters {
    fn default() -> Self {
        Self {
            color: Color32::from_rgb(50, 100, 150),
            light_intensity: 1.0,
        }
    }
}

pub fn setup_params_ui(ui: &mut egui::Ui, params: &mut RtParameters) {
    egui::CollapsingHeader::new("Parameters")
        .default_open(true)
        .show(ui, |ui| {
            Grid::new("render_params")
            .num_columns(2)
            .spacing([12.0, 8.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Color");
                ui.color_edit_button_srgba(&mut params.color);
                ui.end_row();
                
                ui.label("Light intensity");
                ui.add(egui::Slider::new(&mut params.light_intensity, 0.0..=100.0));
                ui.end_row();
            });
        });
}

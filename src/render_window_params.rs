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
use raito::RtPoint3;

pub struct RtParameters {
    pub color: Color32,
    pub center: RtPoint3,
    pub radius: f32,
}

impl Default for RtParameters {
    fn default() -> Self {
        Self {
            color: Color32::from_rgb(50, 100, 150),
            center: RtPoint3::default(),
            radius: 10.0
        }
    }
}

fn camera_ui(ui: &mut egui::Ui, params: &mut RtParameters) {
    ui.label("Center");
    // TODO : add camera parameters
    ui.label("TODO");
    ui.end_row();
}

fn sphere_ui(ui: &mut egui::Ui, params: &mut RtParameters) {
    ui.label("Center");
    ui.horizontal(|ui| {
        ui.add(
            DragValue::new(&mut params.center.x)
                .speed(1.0)
                .clamp_range(-100.0..=100.0)
                .prefix("x: ")
        );
        ui.add(
            DragValue::new(&mut params.center.y)
                .speed(1.0)
                .clamp_range(-100.0..=100.0)
                .prefix("y: ")
        );
        ui.add(
            DragValue::new(&mut params.center.z)
                .speed(1.0)
                .clamp_range(-100.0..=100.0)
                .prefix("z: ")
        );
    });
    ui.end_row();

    ui.label("Radius");
    ui.add(egui::Slider::new(&mut params.radius, 0.0..=100.0));
    ui.end_row();
    
    ui.label("Color");
    ui.color_edit_button_srgba(&mut params.color);
    ui.end_row();
}


pub fn setup_params_ui(ui: &mut egui::Ui, params: &mut RtParameters) {
    egui::CollapsingHeader::new("Camera Parameters")
        .default_open(true)
        .show(ui, |ui| {
            Grid::new("camera_params")
            .num_columns(2)
            .spacing([12.0, 8.0])
            .striped(true)
            .show(ui, |ui| camera_ui(ui, params));
    });

    egui::CollapsingHeader::new("Sphere Parameters")
        .default_open(true)
        .show(ui, |ui| {
            Grid::new("sphere_params")
            .num_columns(2)
            .spacing([12.0, 8.0])
            .striped(true)
            .show(ui, |ui| sphere_ui(ui, params));
    });
}

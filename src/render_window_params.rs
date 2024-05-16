/// =====================================================
///                    Raito Render
/// 
/// Module authors : 
/// - Alice Sonolet <alice.sonolet@gmail.com>
/// 
/// Module description :
///   Parameters exposed in the render view
/// =====================================================

use egui::*;
use log::*;
use raito::RtPoint3;

pub struct RtParameters {
    // Camera params
    pub camera_fov: f32,
    // Light params
    pub light_intensity: f32,
    pub light_color: Color32,
    // Sphere params
    pub sphere_color: Color32,
    pub sphere_center: RtPoint3,
    pub sphere_radius: f32,
}

impl Default for RtParameters {
    fn default() -> Self {
        Self {
            // Camera params
            camera_fov: 47.0,
            // Light params
            light_intensity: 1.0,
            light_color: Color32::from_rgb(50, 50, 50),
            // Sphere params
            sphere_color: Color32::from_rgb(150, 50, 150),
            sphere_center: RtPoint3::new(0.0, 0.0, -5.0),
            sphere_radius: 6.0
        }
    }
}

fn camera_ui(ui: &mut egui::Ui, params: &mut RtParameters) {
    ui.label("FOV");
    // 20~=250mm, 150~=6mm
    ui.add(egui::Slider::new(&mut params.camera_fov, 10.0..=150.0)
        .drag_value_speed(1.0));
    ui.end_row();
}

fn light_ui(ui: &mut egui::Ui, params: &mut RtParameters) {
    ui.label("Intensity");
    ui.add(egui::Slider::new(&mut params.light_intensity, 0.0..=10.0));
    ui.end_row();

    ui.label("Color");
    ui.color_edit_button_srgba(&mut params.light_color);
    ui.end_row();
}

fn sphere_ui(ui: &mut egui::Ui, params: &mut RtParameters) {
    ui.label("Center");
    ui.horizontal(|ui| {
        ui.add(
            DragValue::new(&mut params.sphere_center.x)
                .speed(0.1)
                .min_decimals(1)
                .max_decimals(5)
                .clamp_range(-50.0..=50.0)
                .prefix("x: ")
        );
        ui.add(
            DragValue::new(&mut params.sphere_center.y)
                .speed(0.1)
                .min_decimals(1)
                .max_decimals(5)
                .clamp_range(-50.0..=50.0)
                .prefix("y: ")
        );
        ui.add(
            DragValue::new(&mut params.sphere_center.z)
                .speed(0.1)
                .min_decimals(1)
                .max_decimals(5)
                .clamp_range(-50.0..=50.0)
                .prefix("z: ")
        );
    });
    ui.end_row();

    ui.label("Radius");
    ui.add(egui::Slider::new(&mut params.sphere_radius, 0.0..=10.0)
        .drag_value_speed(0.1));
    ui.end_row();
    
    ui.label("Color");
    ui.color_edit_button_srgba(&mut params.sphere_color);
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

    egui::CollapsingHeader::new("Light Parameters")
        .default_open(true)
        .show(ui, |ui| {
            Grid::new("light_params")
            .num_columns(2)
            .spacing([12.0, 8.0])
            .striped(true)
            .show(ui, |ui| light_ui(ui, params));
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

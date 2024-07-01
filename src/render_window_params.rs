/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Parameters exposed in the render view
/// =====================================================

use egui::*;
use raito::RtPoint3;


fn DragFloatWidget(ui: &mut egui::Ui, updated: &mut bool,
                   name: &str, var: &mut f32, min: f32, max: f32)
{
    if ui.add(
        DragValue::new(var)
            .speed(0.1)
            .min_decimals(1)
            .max_decimals(5)
            .clamp_range(min..=max)
            .prefix(format!("{name}: "))
        ).changed() {
        *updated = true
    }
}

fn Point3Widget(ui: &mut egui::Ui, updated: &mut bool,
                x: &mut f32, y: &mut f32, z: &mut f32)
{
    ui.horizontal(|ui| {
        DragFloatWidget(ui, updated, "x", x, -50.0, 50.0);
        DragFloatWidget(ui, updated, "y", y, -50.0, 50.0);
        DragFloatWidget(ui, updated, "z", z, -50.0, 50.0);
    });
}

/// Render parameters in the UI
pub struct RtParameters {
    pub ipr_enabled: bool,
    // Render settings
    pub render_spp: u8,
    pub max_bounces: u8,
    // Camera params
    pub camera_fov: f32,
    pub look_from: RtPoint3,
    pub look_at: RtPoint3,
    // Light params
    pub light_position: RtPoint3,
    pub light_radius: f32,
    pub light_intensity: f32,
    pub light_color: Color32,
    // Sphere params
    pub sphere_color: Color32,
    pub sphere_center: RtPoint3,
    pub sphere_radius: f32
}

impl Default for RtParameters {
    /// Setup the parameters UI to default value
    fn default() -> Self {
        Self {
            ipr_enabled: false, 
            // Render settings
            render_spp: 3,
            max_bounces: 3,
            // Camera params
            // camera_fov: 47.0,
            camera_fov: 20.0,
            look_from: RtPoint3::new(13.0, 2.0, 3.0),
            look_at: RtPoint3::new(0.0, 0.0, 0.0),
            // Light params
            light_position: RtPoint3::new(0.0, 2.0, 0.0),
            light_radius: 1.0,
            light_intensity: 1.0,
            light_color: Color32::from_rgb(50, 50, 50),
            // Sphere params
            sphere_color: Color32::from_rgb(150, 50, 150),
            sphere_center: RtPoint3::new(0.0, 0.0, -5.0),
            sphere_radius: 1.0
        }
    }
}

fn render_settings_ui(ui: &mut egui::Ui, params: &mut RtParameters, updated: &mut bool) {
    ui.label("SPP");
    if ui.add(egui::Slider::new(&mut params.render_spp, 0..=100)
    .drag_value_speed(1.0)).changed() {
        *updated = true
    }
    ui.end_row();
    
    ui.label("Maximum bounces");
    if ui.add(egui::Slider::new(&mut params.max_bounces, 0..=100)
    .drag_value_speed(1.0)).changed() {
        *updated = true
    }
    ui.end_row();
}

fn camera_ui(ui: &mut egui::Ui, params: &mut RtParameters, updated: &mut bool) {
    ui.label("FOV");
    // 20~=250mm, 150~=6mm
    if ui.add(egui::Slider::new(&mut params.camera_fov, 10.0..=100.0)
    .drag_value_speed(1.0)).changed() {
        *updated = true
    }
    ui.end_row();
    
    ui.label("Look from");
    Point3Widget(
        ui, updated, 
        &mut params.look_from.x, 
        &mut params.look_from.y, 
        &mut params.look_from.z, 
    );
    ui.end_row();

    ui.label("Look at");
    Point3Widget(
        ui, updated, 
        &mut params.look_at.x, 
        &mut params.look_at.y, 
        &mut params.look_at.z, 
    );
    ui.end_row();
}

fn light_ui(ui: &mut egui::Ui, params: &mut RtParameters, updated: &mut bool) {
    ui.label("Center");
    ui.horizontal(|ui| {
        if ui.add(
            DragValue::new(&mut params.light_position.x)
                .speed(0.1)
                .min_decimals(1)
                .max_decimals(5)
                .clamp_range(-50.0..=50.0)
                .prefix("x: ")
            ).changed() {
            *updated = true
        }
        if ui.add(
            DragValue::new(&mut params.light_position.y)
                .speed(0.1)
                .min_decimals(1)
                .max_decimals(5)
                .clamp_range(-50.0..=50.0)
                .prefix("y: ")
            ).changed() {
            *updated = true
        }
        if ui.add(
            DragValue::new(&mut params.light_position.z)
                .speed(0.1)
                .min_decimals(1)
                .max_decimals(5)
                .clamp_range(-50.0..=50.0)
                .prefix("z: ")
            ).changed() {
            *updated = true
        }
    });
    ui.end_row();

    ui.label("Radius");
    if ui.add(egui::Slider::new(&mut params.light_radius, 0.0..=100.0))
        .changed() {
        *updated = true;
    }
    ui.end_row();

    ui.label("Intensity");
    if ui.add(egui::Slider::new(&mut params.light_intensity, 0.0..=10.0))
        .changed() {
        *updated = true;
    }
    ui.end_row();

    ui.label("Color");
    if ui.color_edit_button_srgba(&mut params.light_color)
        .changed() {
        *updated = true;
    }
    ui.end_row();
}

fn sphere_ui(ui: &mut egui::Ui, params: &mut RtParameters, updated: &mut bool) {
    ui.label("Center");
    ui.horizontal(|ui| {
        if ui.add(
            DragValue::new(&mut params.sphere_center.x)
                .speed(0.1)
                .min_decimals(1)
                .max_decimals(5)
                .clamp_range(-50.0..=50.0)
                .prefix("x: ")
            ).changed() {
            *updated = true;
        }
        if ui.add(
            DragValue::new(&mut params.sphere_center.y)
                .speed(0.1)
                .min_decimals(1)
                .max_decimals(5)
                .clamp_range(-50.0..=50.0)
                .prefix("y: ")
            ).changed() {
            *updated = true;
        }
        if ui.add(
            DragValue::new(&mut params.sphere_center.z)
                .speed(0.1)
                .min_decimals(1)
                .max_decimals(5)
                .clamp_range(-50.0..=50.0)
                .prefix("z: ")
            ).changed() {
            *updated = true;
        }
    });
    ui.end_row();

    ui.label("Radius");
    if ui.add(egui::Slider::new(&mut params.sphere_radius, 0.0..=10.0)
        .drag_value_speed(0.1))
        .changed() {
        *updated = true;
    }
    ui.end_row();
    
    ui.label("Color");
    if ui.color_edit_button_srgba(&mut params.sphere_color)
        .changed() {
        *updated = true;
    }
    ui.end_row();
}


pub fn setup_params_ui(ui: &mut egui::Ui, params: &mut RtParameters, updated: &mut bool) {
    egui::CollapsingHeader::new("Render settings")
        .default_open(true)
        .show(ui, |ui| {
            Grid::new("render_settings")
            .num_columns(2)
            .spacing([12.0, 8.0])
            .striped(true)
            .show(ui, |ui| render_settings_ui(ui, params, updated));
    });

    egui::CollapsingHeader::new("Camera Parameters")
        .default_open(true)
        .show(ui, |ui| {
            Grid::new("camera_params")
            .num_columns(2)
            .spacing([12.0, 8.0])
            .striped(true)
            .show(ui, |ui| camera_ui(ui, params, updated));
    });

    // egui::CollapsingHeader::new("Light Parameters")
    //     .default_open(false)
    //     .show(ui, |ui| {
    //         Grid::new("light_params")
    //         .num_columns(2)
    //         .spacing([12.0, 8.0])
    //         .striped(true)
    //         .show(ui, |ui| light_ui(ui, params, updated));
    // });

    // egui::CollapsingHeader::new("Sphere Parameters")
    //     .default_open(false)
    //     .show(ui, |ui| {
    //         Grid::new("sphere_params")
    //         .num_columns(2)
    //         .spacing([12.0, 8.0])
    //         .striped(true)
    //         .show(ui, |ui| sphere_ui(ui, params, updated));
    // });
}

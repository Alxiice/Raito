use egui::*;

pub struct RaitoRenderApp {
    // Declare here attributes 
    scene_descriptor_path: String,

    // Basic render color
    color: Color32,
    light_intensity: f32,

}

impl Default for RaitoRenderApp {
    fn default() -> Self {
        Self {
            // Set here default values for declared attributes
            scene_descriptor_path: "".to_owned(),
            // color: Color32::from_rgb(50, 100, 150).linear_multiply(0.25),
            color: Color32::from_rgb(50, 100, 150),
            light_intensity: 1.0
        }
    }
}

impl RaitoRenderApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }

    pub fn renderview_update(&mut self, ui: &mut Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(ui.available_width(), 300.0), Sense::hover());

        response
    }
}

impl eframe::App for RaitoRenderApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Raito Render");

            // Render view
            Frame::canvas(ui.style()).show(ui, |ui| {
                self.renderview_update(ui);
            });

            // Parameters
            ui.collapsing("Parameters", |ui| {
                Grid::new("render_params")
                    .num_columns(2)
                    .spacing([12.0, 8.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Color");
                        ui.color_edit_button_srgba(&mut self.color);
                        ui.end_row();
                        
                        ui.label("Light intensity");
                        // ui.text_edit_singleline(&mut self.light_intensity);
                        ui.add(egui::Slider::new(&mut self.light_intensity, 0.0..=10.0));
                        ui.end_row();
                    });
            });
        });
    }
}

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
use raito::*;
use log::*;

const WIDTH : usize = 400;
const HEIGHT: usize = 400;

pub struct RaitoRenderApp {
    // Declare here attributes 

    // Basic render color
    color: Color32,
    light_intensity: f32,
    scene: RenderScene,

    // Window size
    color_image: ColorImage,
}

impl Default for RaitoRenderApp {
    fn default() -> Self {
        Self {
            // Set here default values for declared attributes

            // color: Color32::from_rgb(50, 100, 150).linear_multiply(0.25),
            color: Color32::from_rgb(50, 100, 150),
            light_intensity: 1.0,
            scene: RenderScene::default(),
            // Size
            color_image: ColorImage::new([WIDTH, HEIGHT], Color32::from_rgb(50, 100, 150)),
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

    fn update_image(&mut self) {
        for y in 0..WIDTH {
            for x in 0..HEIGHT {
                self.color_image[(y, x)] = self.scene.result.get_pixel_color(y, x);
            }
        }
    }

    pub fn start_render(&mut self) {
        info!("> Start render");
        
        // Setup scene
        info!("> Update render scene");
        let color = RtRGB::new(self.color.r(), self.color.g(), self.color.b());
        self.scene.setup_scene(color, self.light_intensity);

        // Launch render
        self.scene.render();
        info!("> Render finished");

        self.update_image();

    }

    pub fn stop_render(&mut self) {
        warn!("> Stop render : Not implemented yet (no IPR)");
    }

}

impl eframe::App for RaitoRenderApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            let img = ui.ctx().load_texture(
                "renderview-img",
                ImageData::from(self.color_image.clone()),
                Default::default()
            );
            ui.add(egui::Image::new(&img));

            // ui.button("Start rendering")
            ui.horizontal(|ui| {
                if ui.button("Start render").clicked() {
                    self.start_render();
                };
                if ui.button("Stop render").clicked() {
                    self.stop_render();
                };
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

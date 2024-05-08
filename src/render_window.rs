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
            // Displayed image
            // Pixels are ordered row by row, from top to bottom
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
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
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
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // Make sure we don't paint anything behind the rounded corners
        egui::Rgba::TRANSPARENT.to_array()
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        custom_window_frame(ctx, "Raito RenderView", |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            // ui.heading("Raito Render");

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
                        ui.add(egui::Slider::new(&mut self.light_intensity, 0.0..=100.0));
                        ui.end_row();
                    });
            });
        })
    }
}

fn custom_window_frame(ctx: &egui::Context, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    let panel_frame = egui::Frame {
        fill: ctx.style().visuals.window_fill(),
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };

    // TopBottomPanel::top("top_panel").show(ctx, |ui| {
    //     egui::menu::bar(ui, |ui| {
    //         ui.menu_button("File", |ui| {
    //             if ui.button("Quit").clicked() {
    //                 ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    //             }
    //         });
    //         ui.add_space(16.0);
    //         egui::widgets::global_dark_light_mode_buttons(ui);
    //     });
    // });

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, title_bar_rect, title);

        // Add the contents:
        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}

fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
    }

    if title_bar_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui);
        });
    });
}

/// Show some close/maximize/minimize buttons for the native window.
fn close_maximize_minimize(ui: &mut egui::Ui) {
    let button_height = 20.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    if is_maximized {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Restore window");
        if maximized_response.clicked() {
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(false));
        }
    } else {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Maximize window");
        if maximized_response.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
        }
    }

    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}
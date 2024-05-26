/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Opens a render window to load a scene and launch a 
///   render.
/// =====================================================

use egui::*;
// use raito::*;
use log::*;

use crate::render_window_params::*;

use raito::rt_types::*;
use raito::rt_scene::RtScene;
use raito::rt_render_scene::{RenderResult, RtRenderScene};


const WIDTH : usize = 400;
const HEIGHT: usize = 400;
const DEFAULT_COLOR: Color32 = Color32::from_rgb(0, 0, 0);

/// Create app structure
pub struct RaitoRenderApp {
    // Parameters
    parameters: RtParameters,
    // Render Scene
    scene: RtScene,
    result: RenderResult,
    // Displayed image
    color_image: ColorImage,
}

impl Default for RaitoRenderApp {
    /// Init app values to default
    fn default() -> Self {
        Self {
            parameters: RtParameters::default(),
            scene: RtScene::default(),
            result: RenderResult::new(),
            color_image: ColorImage::new([WIDTH, HEIGHT], DEFAULT_COLOR),
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

    /// Update the current image cache
    fn update_image(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.color_image[(x, y)] = self.result.get_pixel_color(x, y);
            }
        }
    }

    /// Start the render
    pub fn start_render(&mut self) {
        info!("> Start render");
        
        // Setup scene
        info!("> Setup render scene");
        self.scene = RtScene::new(
            self.parameters.camera_fov,
            RtRGBA::from_color32(self.parameters.sphere_color),
            self.parameters.sphere_center,
            self.parameters.sphere_radius,
            self.parameters.light_position,
            self.parameters.light_radius,
            RtRGBA::from_color32(self.parameters.light_color),
            self.parameters.light_intensity
        );

        // Launch render
        RtRenderScene(&mut self.scene, &mut self.result);
        info!("> Render finished");

        // Update display image
        self.update_image();
    }

    /// Stops the render
    pub fn stop_render(&mut self, ctx: &egui::Context) {
        // TODO : When IPR is implemented, stops the IPR
        info!("> Closes Raito RenderView");
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let available_size = [(WIDTH as f32) / 2.0 - 4.0, 25.0];
                        let start_button = Button::new("Start Render");
                        let stop_button = Button::new("Stop Render");
                        if ui.add_sized(available_size, start_button).clicked() {
                            self.start_render();
                        };
                        if ui.add_sized(available_size, stop_button).clicked() {
                            self.stop_render(ctx);
                        };
                    });

                    // Render view
                    let img = ui.ctx().load_texture(
                        "renderview-img",
                        ImageData::from(self.color_image.clone()),
                        Default::default()
                    );
                    ui.add(egui::Image::new(&img));
                });

                // Parameters
                let mut updated = false;
                ui.vertical(|ui| {
                    setup_params_ui(ui, &mut self.parameters, &mut updated);
                });
                if updated {
                    self.start_render();
                } 
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
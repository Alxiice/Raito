use eframe::glow::Shader;
/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Opens a render window to load a scene and launch a 
///   render.
/// =====================================================

use egui::*;
use log::*;

use raito::{random_float, random_float_range, RtRenderScene, RT_DEFAULT_WINDOW_HEIGHT, RT_DEFAULT_WINDOW_WIDTH};
const DEFAULT_COLOR: Color32 = Color32::from_rgb(0, 0, 0);
use raito::rt_types::*;
use crate::render_window_params::*;
use raito::rt_camera::RtCamera;
use raito::rt_objects::rt_object_base::ObjectParams;
use raito::rt_objects::rt_geometries::RtSphere;
use raito::rt_objects::rt_lights::RtPointLight;
use raito::rt_shaders::rt_shader_base::RtShader;
use raito::rt_shaders::stateVector::StateVectorShader;
use raito::rt_shaders::lambert::LambertShader;
use raito::rt_shaders::metal::Metal;
use raito::rt_shaders::glass::Glass;
use raito::rt_shaders::lightShader::LightShader;
use raito::rt_scene::RtScene;
use raito::rt_render_output::RtRenderResult;


/// Create app structure
pub struct RaitoRenderApp {
    // Parameters
    parameters: RtParameters,
    // Render Scene
    scene: Option<RtScene>,
    result: RtRenderResult,
    // Displayed image
    color_image: ColorImage,
}

impl Default for RaitoRenderApp {
    /// Init app values to default
    fn default() -> Self {
        Self {
            parameters: RtParameters::default(),
            scene: None,
            result: RtRenderResult::new(
                RT_DEFAULT_WINDOW_WIDTH, RT_DEFAULT_WINDOW_HEIGHT),
            color_image: ColorImage::new(
                [RT_DEFAULT_WINDOW_WIDTH, RT_DEFAULT_WINDOW_HEIGHT], DEFAULT_COLOR),
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
        for y in 0..self.result.height {
            for x in 0..self.result.width {
                // self.color_image[(x, self.result.height - y - 1)] = self.result.get_pixel_color(x, y);
                self.color_image[(x, y)] = self.result.get_pixel_color(x, y);
            }
        }
    }

    pub fn setup_scene(&self, camera: RtCamera) -> RtScene {
        let mut scene = RtScene::new(camera);

        // Ground
        scene.add_shape(Box::new(RtSphere { 
            object_params: ObjectParams::new(
                String::from(""), String::from(""),
                Box::new(LambertShader {
                    color: RtRGBA::from_rgb(0.5, 0.5, 0.5)
                })
            ),
            center: RtPoint3::new(0.0, -1000.0, 0.0),
            radius: 1000.0
        }));

        // Mini spheres
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_float();
                let center = RtPoint3::new(a as f32 + 0.9*random_float(), 0.2, b as f32 + 0.9*random_float());
                if (center - RtPoint3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let mut sphere_material: Box<dyn RtShader>;
                    if choose_mat < 0.8 { // diffuse
                        sphere_material = Box::new(LambertShader {
                            color: RtRGBA::random() * RtRGBA::random()
                        });
                    } else if choose_mat < 0.95 { // metal
                        sphere_material = Box::new(Metal {
                            color: RtRGBA::random_range(0.5, 1.0),
                            fuzz: random_float_range(0.0, 0.5)
                        });
                    } else { // glass
                        sphere_material = Box::new(Glass {
                            ior: 1.5
                        });
                    }
                    // Create new sphere
                    scene.add_shape(Box::new(RtSphere { 
                        object_params: ObjectParams::new(
                            String::from(""), String::from(""), 
                            sphere_material),
                        center,
                        radius: 0.2
                    }));
                }
            }
        }

        // Sphere left
        scene.add_shape(Box::new(RtSphere { 
            object_params: ObjectParams::new(
                String::from(""), String::from(""),
                Box::new(LambertShader {
                    color: RtRGBA::from_rgb(0.4, 0.2, 0.1)
                })
            ),
            center: RtPoint3::new(-4.0, 1.0, 0.0),
            radius: 1.0
        }));
        // Sphere center
        scene.add_shape(Box::new(RtSphere { 
            object_params: ObjectParams::new(
                String::from(""), String::from(""),
                Box::new(Glass {
                    ior: 1.5
                })
            ),
            center: RtPoint3::new(0.0, 1.0, 0.0),
            radius: 1.0
        }));
        // Sphere right
        scene.add_shape(Box::new(RtSphere { 
            object_params: ObjectParams::new(
                String::from(""), String::from(""),
                Box::new(Metal {
                    color: RtRGBA::from_rgb(0.7, 0.6, 0.5),
                    fuzz: 0.0
                })
            ),
            center: RtPoint3::new(4.0, 1.0, 0.0),
            radius: 1.0
        }));

        scene
    }

    fn update_params(&mut self) {
        // TODO
        // Instead of recreating the scene
        // Use object IDs to update values
        // Also try to be smart and keep tracked on updated 
        // objects & parameters so that we don't update everything
        // on the object

        // ===== Create camera =====
        let camera = RtCamera::new(
            1.0, 400, self.parameters.camera_fov, 
            self.parameters.camera_position,
            RtPoint3::new(0.0, 0.0, 0.0), 
            RtVec3::new(0.0, 1.0, 0.0)
        );
        // ===== Add geometry =====
        let scene = self.setup_scene(camera);

        self.scene = Some(scene);
    }

    fn re_render(&mut self) -> Option<std::time::Duration> {
        // Launch render
        // debug!("> Start render");
        let scene = self.scene.as_ref();
        let mut elapsed_time = None;
        if scene.is_some() {
            let now = std::time::Instant::now();
            RtRenderScene(scene.unwrap(), &mut self.result);
            elapsed_time = Some(now.elapsed());
        } else {
            error!("No scene to render !");
        }
        // if self.scene.is_some() {
        // }
        // debug!("> Render finished");
        // Update display image
        self.update_image();
        elapsed_time
    }

    /// Start the render
    pub fn start_render(&mut self) {
        // Setup scene
        info!("> Setup render scene");

        self.update_params();

        // self.parameters.ipr_enabled = true;

        // Launch render
        // info!("Starting IPR");
        info!("Starting render");
        let duration = self.re_render();
        info!("Render finished in !");
        if duration.is_some() {
            info!("-> took {} sec", duration.unwrap().as_secs_f64());
        }
    }

    /// Stops the render
    pub fn stop_render(&mut self, ctx: &egui::Context) {
        // TODO : When IPR is implemented, stops the IPR
        info!("> Stopping IPR");
        self.parameters.ipr_enabled = false;
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
                        let available_size = [(RT_DEFAULT_WINDOW_WIDTH as f32) / 2.0 - 4.0, 25.0];
                        let button_color = if self.parameters.ipr_enabled {
                            Color32::from_rgb(0, 190, 0)
                        } else {
                            Color32::from_rgb(60, 60, 60)
                        };
                        let start_button = Button::new("Start Render")
                            .fill(button_color);
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
                if updated && self.parameters.ipr_enabled {
                    self.update_params();
                    self.re_render();
                } else {
                    self.update_image();
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
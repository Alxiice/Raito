/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Opens a render window to load a scene and launch a 
///   render.
/// =====================================================

use egui::*;
use log::*;

use raito::{RtRenderScene, RT_DEFAULT_WINDOW_HEIGHT, RT_DEFAULT_WINDOW_WIDTH};
const DEFAULT_COLOR: Color32 = Color32::from_rgb(0, 0, 0);
use raito::rt_types::*;
use crate::render_window_params::*;
use raito::rt_camera::RtCamera;
use raito::rt_objects::rt_object_base::ObjectParams;
use raito::rt_objects::rt_geometries::RtSphere;
use raito::rt_objects::rt_lights::RtPointLight;
use raito::rt_shaders::stateVector::StateVectorShader;
// use raito::rt_shaders::lambert::LambertShader;
use raito::rt_shaders::lightShader::LightShader;
use raito::rt_scene::RtScene;
use raito::rt_render_output::RtRenderResult;


// ========================================
//  RtScene contains the scene setup
//  and is doomed to disappear to be 
//  replaced by a XML file that we will 
//  load
// ========================================

/// Describes a render scene
struct RenderScene {
    // Viewport
    width: u16,
    height: u16,
    // Camera params
    camera_fov: f32,
    _camera_position: RtPoint3,
    _camera_rotation: RtPoint3,
    // Sphere params
    _sphere_color: RtRGBA,
    sphere_center: RtPoint3,
    sphere_radius: f32,
    // Light params
    light_center: RtPoint3,
    light_radius: f32,
    light_color: RtRGBA,
    light_intensity: f32
    
}

impl Default for RenderScene {
    fn default() -> Self {
        Self {
            height: RT_DEFAULT_WINDOW_HEIGHT as u16,
            width: RT_DEFAULT_WINDOW_WIDTH as u16,
            // Camera params
            camera_fov: 1.0,
            _camera_position: RtPoint3::default(),
            _camera_rotation: RtPoint3::default(),
            // Sphere params
            _sphere_color: RtRGBA::BLACK,
            sphere_center: RtPoint3::default(),
            sphere_radius: 1.0,
            // Light params
            light_center: RtPoint3::default(),
            light_radius: 1.0,
            light_color: RtRGBA::WHITE,
            light_intensity: 1.0
        }
    }
}

impl RenderScene {
    /// Update scene parameters
    pub fn new(width: u16, height: u16,
               camera_fov: f32, _camera_position: RtPoint3, _camera_rotation: RtPoint3,
               _sphere_color: RtRGBA, sphere_center: RtPoint3, sphere_radius: f32,
               light_center: RtPoint3, light_radius: f32, light_color: RtRGBA, light_intensity: f32
    ) -> Self {
        Self {
            // Viewport
            width, height, 
            // Camera
            camera_fov, _camera_position, _camera_rotation,
            // Sphere
            _sphere_color, sphere_center, sphere_radius,
            // Light
            light_center, light_radius, light_color, light_intensity
        }
    }

    pub fn to_render_scene(self) -> RtScene {
        // Create camera
        let aspect = (self.width as f32) / (self.height as f32);
        let mut camera = RtCamera::new(self.width, aspect);
        camera.camera_fov = self.camera_fov;
        // camera.center = self.camera_position
        // camera.rotation = self.camera_rotation

        // Create render scene
        let mut render_scene = RtScene::new(camera);

        // Add shapes
        let sphere = RtSphere { 
            object_params: ObjectParams::new(
                String::from("/root/geo/sphere"),
                String::from("geometry"),
                Box::new(StateVectorShader {
                    output: String::from("N")
                })
                // Box::new(LambertShader {
                //     color: self.sphere_color
                // })
            ),
            center: self.sphere_center,
            radius: self.sphere_radius
        };
        render_scene.add_shape(Box::new(sphere));

        // Add lights
        let light = RtPointLight {
            object_params: ObjectParams::new(
                String::from("/root/lights/point_light"),
                String::from("light"),
                Box::new(LightShader {
                    color: self.light_color,
                    intensity: self.light_intensity
                })),
            center: self.light_center,
            radius: self.light_radius
        };
        render_scene.add_light(Box::new(light));

        render_scene
    }
}


// ========================================
//  Now the window code
// ========================================

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
                self.color_image[(x, self.result.height - y - 1)] = self.result.get_pixel_color(x, y);
            }
        }
    }

    fn update_params(&mut self) {
        // TODO
        // Instead of recreating the scene
        // Use object IDs to update values
        // Also try to be smart and keep tracked on updated 
        // objects & parameters so that we don't update everything
        // on the object

        let scene = RenderScene::new(
            self.result.width as u16, self.result.height as u16, 
            self.parameters.camera_fov,
            self.parameters.camera_position,
            self.parameters.camera_rotation,
            RtRGBA::from_color32(self.parameters.sphere_color),
            self.parameters.sphere_center,
            self.parameters.sphere_radius,
            self.parameters.light_position,
            self.parameters.light_radius,
            RtRGBA::from_color32(self.parameters.light_color),
            self.parameters.light_intensity
        );

        // From our RtScene we create a render scene
        self.scene = Some(scene.to_render_scene());
    }

    fn re_render(&mut self) {
        // Launch render
        debug!("> Start render");
        let scene = self.scene.as_ref();
        if scene.is_some() {
            RtRenderScene(scene.unwrap(), &mut self.result);
        } else {
            error!("No scene to render !");
        }
        // if self.scene.is_some() {
        // }
        debug!("> Render finished");
        // Update display image
        self.update_image();
    }

    /// Start the render
    pub fn start_render(&mut self) {
        // Setup scene
        info!("> Setup render scene");
        let scene = RenderScene::new(
            RT_DEFAULT_WINDOW_WIDTH as u16, RT_DEFAULT_WINDOW_HEIGHT as u16, 
            self.parameters.camera_fov,
            self.parameters.camera_position,
            self.parameters.camera_rotation,
            RtRGBA::from_color32(self.parameters.sphere_color),
            self.parameters.sphere_center,
            self.parameters.sphere_radius,
            self.parameters.light_position,
            self.parameters.light_radius,
            RtRGBA::from_color32(self.parameters.light_color),
            self.parameters.light_intensity
        );

        // From our RtScene we create a render scene
        self.scene = Some(scene.to_render_scene());

        self.parameters.ipr_enabled = true;

        // Launch render
        info!("Starting IPR");
        self.re_render();
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
/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Opens a render window to load a scene and launch a 
///   render.
/// =====================================================

use egui::*;
use std::path::PathBuf;
use eframe::egui;
use egui_file_dialog::{FileDialog, DialogState};

use log::*;

use raito::{random_float, random_float_range, RtRenderScene, RtRenderSettings, RT_DEFAULT_WINDOW_HEIGHT, RT_DEFAULT_WINDOW_WIDTH};
use raito::rt_types::*;
use crate::render_window_params::*;
use raito::rt_camera::RtCamera;
use raito::rt_objects::rt_object_base::ObjectParams;
use raito::rt_objects::rt_geometries::RtSphere;
use raito::rt_shaders::rt_shader_base::RtShader;
use raito::rt_shaders::lambert::LambertShader;
use raito::rt_shaders::metal::Metal;
use raito::rt_shaders::glass::Glass;
use raito::rt_scene::RtScene;
use raito::rt_scene::open_xml_scene;
use raito::rt_render_output::RtRenderResult;

const DEFAULT_COLOR: Color32 = Color32::from_rgb(0, 0, 0);


// ========================================
//  Get default scene
//  will disappear
// ========================================

pub fn get_default_scene_0(settings: RtRenderSettings, camera: RtCamera) -> RtScene {
    let mut scene = RtScene::new(settings, camera);

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
                let sphere_material: Box<dyn RtShader>;
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

pub fn get_default_scene_1(settings: RtRenderSettings, camera: RtCamera) -> RtScene {
    let mut scene = RtScene::new(settings, camera);

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

    // Sphere left
    scene.add_shape(Box::new(RtSphere { 
        object_params: ObjectParams::new(
            String::from(""), String::from(""),
            Box::new(LambertShader {
                color: RtRGBA::from_rgb(0.4, 0.2, 0.1)
            })
        ),
        center: RtPoint3::new(-2.5, 1.0, 0.0),
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
        center: RtPoint3::new(2.5, 1.0, 0.0),
        radius: 1.0
    }));

    scene
}


// ========================================
//  Create GUI
// ========================================

#[derive(Debug, PartialEq, Eq)]
enum OpeningFileStatus {
    None,
    ToOpen,
    ChoosingFile,
}

/// Create app structure
pub struct RaitoRenderApp {
    // Parameters
    parameters: RtParameters,
    // Render Scene
    scene: Option<RtScene>,
    result: RtRenderResult,
    // Displayed image
    color_image: ColorImage,

    // file dialog
    opening_file_status: OpeningFileStatus,
    file_dialog: FileDialog,
}

impl Default for RaitoRenderApp {
    /// Init app values to default
    fn default() -> Self {
        Self {
            parameters: RtParameters::default(),
            scene: None,
            result: RtRenderResult::new(
                RT_DEFAULT_WINDOW_WIDTH, RT_DEFAULT_WINDOW_HEIGHT, 0, 0),
            color_image: ColorImage::new(
                [RT_DEFAULT_WINDOW_WIDTH, RT_DEFAULT_WINDOW_HEIGHT], DEFAULT_COLOR),
            opening_file_status: OpeningFileStatus::None,
            file_dialog: FileDialog::new()
                .resizable(false)
                .movable(false)
                .title_bar(false)
                .show_top_panel(false)
                .show_new_folder_button(false),
        }
    }
}

impl RaitoRenderApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `_cc.egui_ctx.set_visuals` and `_cc.egui_ctx.set_fonts`.
        Default::default()
    }

    /// Update the current image cache
    fn update_image(&mut self) {
        for y in 0..self.result.height {
            for x in 0..self.result.width {
                self.color_image[(x, y)] = self.result.get_pixel_color(x, y).to_color32();
            }
        }
    }

    pub fn setup_default_scene(&mut self) {
        let settings = RtRenderSettings::new(
            self.parameters.render_spp, self.parameters.max_bounces);
        let camera = RtCamera::new(
            1.0, 400, self.parameters.camera_fov, 
            self.parameters.look_from,
            self.parameters.look_at,
            RtVec3::new(0.0, 1.0, 0.0));
        self.scene = Some(get_default_scene_0(settings, camera));
    }

    pub fn open_scene(&mut self, path: PathBuf) -> bool {
        let xml_scene = open_xml_scene(path.to_str().unwrap());
        self.scene = xml_scene;
        
        // Setup UI from scene parameters
        //   render settings
        self.parameters.render_spp = self.scene.as_ref().unwrap().settings.render_spp;
        self.parameters.max_bounces = self.scene.as_ref().unwrap().settings.max_bounces;
        //   camera
        self.parameters.camera_fov = self.scene.as_ref().unwrap().get_camera()._vfov;
        self.parameters.look_from = self.scene.as_ref().unwrap().get_camera()._look_from;
        self.parameters.look_at = self.scene.as_ref().unwrap().get_camera()._look_at;

        true
    }

    fn update_params(&mut self) {
        let scene = self.scene.as_mut();
        if scene.is_none() {
            // Panic because we shouldn't get here
            // -> only update if the IPR is launched, i.e. the scene exists
            panic!("No scene to update !");
        }

        // Settings
        let settings = RtRenderSettings::new(
            self.parameters.render_spp, self.parameters.max_bounces);
            scene.unwrap().set_settings(settings);

        // Camera
        let camera = RtCamera::new(
            1.0, 400, self.parameters.camera_fov, 
            self.parameters.look_from,
            RtPoint3::new(0.0, 0.0, 0.0), 
            RtVec3::new(0.0, 1.0, 0.0));
        let scene = self.scene.as_mut();
        scene.unwrap().set_camera(camera);

        // TODO : update other things ?
    }

    fn render(&mut self) -> bool {
        // let scene = self.scene.as_ref();
        // if scene.is_some() {
        //     RtRenderScene(scene.unwrap(), &mut self.result);
        // } else {
        //     error!("No scene to render !");
        //     self.parameters.ipr_enabled = false;  // Make sure to disable IPR
        //     return false;
        // }
        // self.update_image();
        if self.scene.is_some() {
            let scene = self.scene.as_ref().unwrap().clone();
            RtRenderScene(scene, &mut self.result);
        } else {
            error!("No scene to render !");
            self.parameters.ipr_enabled = false;  // Make sure to disable IPR
            return false;
        }
        self.update_image();
        true
    }

    /// Start the render
    pub fn launch_render(&mut self) {
        // Setup scene
        if self.scene.is_none() {
            error!("No scene loaded to render !");
            self.setup_default_scene();
            // return;
        } else {
            self.update_params();
        }
        
        // Launch render
        info!("Starting render");
        let now = std::time::Instant::now();
        if self.render() {
            info!("Render finished in {} sec", now.elapsed().as_secs_f64());
        }  // Else : error happened
    }

    /// Stops the render
    pub fn toggle_ipr(&mut self) {
        if self.parameters.ipr_enabled {
            info!("> Stopping IPR");
            self.parameters.ipr_enabled = false;
        } else {
            info!("> Starting IPR");
            self.parameters.ipr_enabled = true;
            self.launch_render();
        }
    }
}

fn main_ui(app: &mut RaitoRenderApp, ui: &mut Ui, ctx: &egui::Context) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let available_size = [(RT_DEFAULT_WINDOW_WIDTH as f32) / 2.0 - 4.0, 25.0];
                let start_button = Button::new("Launch Render");
                let ipr_button = if app.parameters.ipr_enabled {
                    Button::new("Stop IPR").fill(Color32::from_rgb(0, 190, 0))
                } else {
                    Button::new("Start IPR")
                };
                if ui.add_sized(available_size, start_button).clicked() {
                    app.launch_render();
                };
                if ui.add_sized(available_size, ipr_button).clicked() {
                    app.toggle_ipr();
                };
            });

            // Render view
            // let now = std::time::Instant::now();
            let img = ui.ctx().load_texture(
                "renderview-img",
                ImageData::from(app.color_image.clone()),
                Default::default()
            );
            ui.add(egui::Image::new(&img));
            // info!("Image display took : {} sec", now.elapsed().as_secs_f64());
        });

        // Parameters
        let mut updated = false;
        ui.vertical(|ui| {
            setup_params_ui(ui, &mut app.parameters, &mut updated);
        });
        if updated && app.parameters.ipr_enabled {
            app.update_params();
            app.render();
        }
    });
}

impl eframe::App for RaitoRenderApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // Make sure we don't paint anything behind the rounded corners
        egui::Rgba::TRANSPARENT.to_array()
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open Scene").clicked() {
                        self.opening_file_status = OpeningFileStatus::ToOpen;
                    }
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            if self.opening_file_status == OpeningFileStatus::None {
                main_ui(self, ui, ctx)
            } else {
                // TODO : This behaviour for the open scene is not great,
                // I would prefer an external window.
                // Other solution is to rewrite the explorer and integrate it 
                // correctly inside the main window

                if self.opening_file_status == OpeningFileStatus::ToOpen {
                    self.file_dialog.select_file();
                    self.opening_file_status = OpeningFileStatus::ChoosingFile;
                }

                if self.file_dialog.state() == DialogState::Cancelled || 
                   self.file_dialog.state() == DialogState::Closed {
                    self.opening_file_status = OpeningFileStatus::None;
                }
    
                // Update the dialog and check if the user selected a file
                if let Some(path) = self.file_dialog.update(ctx).selected() {
                    let selected_file = path.to_path_buf();
                    self.open_scene(selected_file);
                    self.opening_file_status = OpeningFileStatus::None;
                }
            }
        });
    }
}

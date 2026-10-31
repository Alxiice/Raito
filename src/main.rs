#![allow(non_snake_case)]
#![allow(unused)]

#![warn(clippy::all, rust_2018_idioms)]
// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// =====================================================
///                    Raito Render
/// 
/// Module description :
///   Defines executable that will launch a render
///   window. 
/// =====================================================

mod render_window_params;
mod render_window;
mod rt_test;
pub use rt_test::rt_test;
pub use render_window::RaitoRenderApp;
use log::{info, error};
use clap::{Parser, ValueEnum};
use raito::rt_scene::open_xml_scene;
use raito::rt_render::RtRenderScene;
use raito::rt_render_output::RtRenderResult;

const WINDOW_WIDTH: f32 = 710.0;
const MIN_WINDOW_HEIGHT: f32 = 470.0;
const MAX_WINDOW_HEIGHT: f32 = 470.0;

/// Raito Rendering Engine
#[derive(Debug, Parser)]
#[clap(name = "raito", version = "0.1.0", author = "Alice Sonolet")]
pub struct RaitoArgs {
    /// Verbosity level
    #[arg(short, long, value_enum, default_value_t = RtLevel::Info)]
    log_level: RtLevel,
    
    /// Execute tests
    #[arg(short, long)]
    tests: bool,

    /// Render an XML scene without opening the render window
    #[arg(short, long, requires = "output")]
    scene: Option<String>,

    /// Output image (PPM) written by a headless render
    #[arg(short, long, requires = "scene")]
    output: Option<String>,
}

/// Render a scene file to an image without UI
fn render_headless(scene_path: &str, output_path: &str) -> i32 {
    let scene = match open_xml_scene(scene_path) {
        Some(scene) => scene,
        None => {
            error!("Could not open scene {scene_path}");
            return 1;
        }
    };
    let camera = scene.get_camera();
    let mut result = RtRenderResult::new(
        camera.get_width() as usize, camera.get_height() as usize, 0, 0);

    info!("Starting render");
    let now = std::time::Instant::now();
    RtRenderScene(scene, &mut result);
    info!("Render finished in {} sec", now.elapsed().as_secs_f64());

    match result.export_as_ppm(output_path) {
        Ok(_) => { info!("Image written to {output_path}"); 0 },
        Err(e) => { error!("Could not write {output_path} : {e}"); 1 }
    }
}

/// Log levels
#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum RtLevel { Debug, Info, Warning, Error }

fn main() -> Result<(), eframe::Error> {
    let args = RaitoArgs::parse();

    match args.log_level {
        RtLevel::Debug   => std::env::set_var("RUST_LOG", "debug"),
        RtLevel::Info    => std::env::set_var("RUST_LOG", "info"),
        RtLevel::Warning => std::env::set_var("RUST_LOG", "warning"),
        RtLevel::Error   => std::env::set_var("RUST_LOG", "error"),
    }

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    info!("====================== RAITO RENDER ======================");

    if args.tests {
        rt_test();
        return Ok(());
    }

    if let (Some(scene), Some(output)) = (&args.scene, &args.output) {
        std::process::exit(render_headless(scene, output));
    }

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Raito")
            .with_title_shown(true)
            .with_inner_size([WINDOW_WIDTH, MIN_WINDOW_HEIGHT])
            .with_min_inner_size([WINDOW_WIDTH, MIN_WINDOW_HEIGHT])
            .with_max_inner_size([WINDOW_WIDTH, MAX_WINDOW_HEIGHT]),
        // follow_system_theme: true,
        default_theme: eframe::Theme::Dark,
        ..Default::default()
    };
    eframe::run_native(
        "Raito Render",
        native_options,
        Box::new(|_cc| Box::<RaitoRenderApp>::default()),
    )
}

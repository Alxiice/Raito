#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

/// =====================================================
///                    Raito Render
/// 
/// Module authors : 
/// - Alice Sonolet <alice.sonolet@gmail.com>
/// 
/// Module description :
///   Defines executable that will launch a render
///   window. 
/// =====================================================

mod render_window;
pub use render_window::RaitoRenderApp;
use log::info;
use clap::{Parser, ValueEnum};

const WINDOW_WIDTH: f32 = 410.0;
const MIN_WINDOW_HEIGHT: f32 = 560.0;
const MAX_WINDOW_HEIGHT: f32 = 800.0;

/// Raito Rendering Engine
#[derive(Debug, Parser)]
#[clap(name = "raito", version = "0.1.0", author = "Alice Sonolet")]
pub struct RaitoArgs {
    /// Verbosity level
    #[arg(short, long, value_enum, default_value_t = RtLevel::Info)]
    log_level: RtLevel,
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

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false) // Hide the OS-specific "chrome" around the window
            .with_inner_size([WINDOW_WIDTH, MIN_WINDOW_HEIGHT])
            .with_min_inner_size([WINDOW_WIDTH, MIN_WINDOW_HEIGHT])
            .with_max_inner_size([WINDOW_WIDTH, MAX_WINDOW_HEIGHT])
            .with_transparent(true), // To have rounded corners we need transparency
        ..Default::default()
    };
    eframe::run_native(
        "Raito Render",
        native_options,
        Box::new(|_cc| Box::<RaitoRenderApp>::default()),
    )
}

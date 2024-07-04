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
use log::info;
use clap::{Parser, ValueEnum};

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

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

const WINDOW_WIDTH: f32 = 415.0;
const MIN_WINDOW_HEIGHT: f32 = 520.0;
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

fn main() -> eframe::Result<()> {
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
            .with_inner_size([WINDOW_WIDTH, MIN_WINDOW_HEIGHT])
            .with_min_inner_size([WINDOW_WIDTH, MIN_WINDOW_HEIGHT])
            .with_max_inner_size([WINDOW_WIDTH, MAX_WINDOW_HEIGHT])
            .with_resizable(false)
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Raito Render",
        native_options,
        Box::new(|cc| Box::new(RaitoRenderApp::new(cc))),
    )

    // fn name(&self) -> &'static str {
    //     "） Window Name"
    // }
    
    // fn show(&mut self, ctx: &Context, open: &mut bool) {
    //     use super::View as _;
    //     Window::new(self.name())
    //         .open(open)
    //         .vscroll(false)
    //         .resizable(false)
    //         .default_size([300.0, 350.0])
    //         .show(ctx, |ui| self.ui(ui));
    // }
}

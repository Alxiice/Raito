#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod render_window;
pub use render_window::RaitoRenderApp;

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 500.0])
            .with_min_inner_size([420.0, 500.0])
            .with_max_inner_size([420.0, 600.0])
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

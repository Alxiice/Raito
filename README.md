# Raito
Raito Rendering engine

## Dependencies

The GUI is created with [egui](https://github.com/emilk/egui) and [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) using the [eframe template](https://github.com/emilk/eframe_template/)

## Run

1. [Install rust](https://www.rust-lang.org/tools/install)
2. Install eframe dependencies :
  - Linux : `sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`
  - Fedora : `sudo dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`
3. Build and run : `cargo run --release`

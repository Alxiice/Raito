# Raito

Raito Rendering engine

<p align="center">
  <img src="./docs/source/videos/pres-1.gif" /><br/>
  <i>Raito RenderView UI with a sphere shaded with normals</i>
</p>

## Dependencies

> [!WARNING]
> egui UI in this project is deprecated and is being replaced with a Python / QML interface, see [`ui/`](./ui/README.md).

- The new scene editor (`ui/`) is written in Python with [PySide6](https://doc.qt.io/qtforpython-6/) and QML
- The legacy GUI is created with [egui](https://github.com/emilk/egui) and [eframe](https://github.com/emilk/egui/tree/master/crates/eframe)
- The documentation runs on mkdocs with several plugins required :
  - `pip install mkdocs-material` (theme)
  - `pip install mkdocs-glightbox` (to zoom on images)

Here are also additional dependencies :
- `log` : logging library
- `env_logger` : used to setup the log level
- [`clap`](https://github.com/clap-rs/clap) : to parse command line argumments
- `rand` & `rand_chacha` : random numbers
- `quick-xml` : Read XML files

## Run

1. [Install rust](https://www.rust-lang.org/tools/install)
2. You might need to install specific packages for eframe :
  - Linux : `sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`
  - Fedora : `sudo dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`
3. Build and run : `cargo run --release`
4. Run the scene editor : `pip install -r ui/requirements.txt` then `python -m ui` (see [`ui/README.md`](./ui/README.md))
5. To build and run doc launch : `python -m mkdocs serve`. That will serve the site on localhost

## Command Line Arguments
```
Usage: raito [OPTIONS]

Options:
  -l, --log-level <LOG_LEVEL>  Verbosity level [default: info] [possible values: debug, info, warning, error]
  -t, --tests                  Execute tests
  -s, --scene <SCENE>          Render an XML scene without opening the render window
  -o, --output <OUTPUT>        Output image (PPM) written by a headless render
  -h, --help                   Print help
  -V, --version                Print version
```

For example `raito --scene tests/scenes/scene_000.xml --output render.ppm` renders a scene without UI. The scene editor uses this mode to render.

## Next steps

### Release v0.1

- GUI: egui brings a lot of constraints therefore the UI is migrated to a Python + QML UI (first version in [`ui/`](./ui/README.md)) that integrates :
  - A nodal graph for scene building
  - A node editor to edit node parameters
  - A scenegraph and an attribute viewer to view scenegraph location attributes
  - A viewer that integrate render with Raito but also scene previsualization through rasterized render (though for now only Raito is implemented)
  - Dockable panels that can be isolated in their own windows
- Rendering
  - Implement polymesh nodes for complete set of geometry
  - Implement BVH and acceleration structures
- Render integration
  - Implement Raito inside the new GUI (no IPR mode yet)

### Release v0.2

- Add Raito IPR mode: Exchange render data / scene and packets of rendered buckets through gRPC
- Integrate a rasterizer as alternative render method

### Release v1.0

- Rework the render engine to improve performances
- Add new shaders and geometries

### Release v2.0

- Develop new iterators and optimizations, like photon mapping, VCM, etc
 
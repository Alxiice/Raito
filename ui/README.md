# Raito UI

Node-based scene editor for the Raito render engine, written in Python with a
**QML** (Qt Quick) interface. You build scenes as node graphs, Katana style:
each node adds locations to a scene graph, and the viewer renders the scene
produced by the *viewed* node.

## Features

- **Node Graph**: create, connect and arrange nodes that build the scene
- **Node Editor**: edit the parameters of the selected node
- **Scene Graph**: browse the locations produced by the viewed node
- **Attributes**: inspect the attributes of the selected scene graph location
- **Viewer**: render with Raito. A combobox switches render engines (only Raito
  is available for now)
- **Docking**: every panel can be docked, tabbed, moved, closed, or isolated in
  its own window. The layout is saved between sessions

## Requirements

- Python 3.9+
- PySide6 6.12+ (`pip install -r ui/requirements.txt`)
- A Raito build. The viewer looks for the executable in this order:
  `RAITO_BIN` environment variable, `target/release`, `target/debug`, then `PATH`

```bash
cargo build --release
pip install -r ui/requirements.txt
```

## Running

From the repository root:

```bash
python -m ui                     # opens Raito's reference scene
python -m ui my_scene.json       # opens a saved node graph
python -m ui --reset-layout      # ignores the saved panel layout
```

`ui/run.bat` and `ui/run.sh` do the same and install PySide6 if it is missing.

## Using the editor

### Node Graph

| Action | Input |
| --- | --- |
| Create a node | Right click, or `Tab` to search |
| Connect | Drag from a port to another port (or onto a node) |
| Move / remove a wire | Drag it away from its input port |
| Select / move a node | Click / drag |
| View a node | Click its square flag, double click it, or press `V` |
| Delete the selected node | `Delete` |
| Pan / zoom | Middle drag or `Alt` + drag / mouse wheel |
| Frame all nodes | `F` |

Port colors give their type: white ports carry scenes, purple ports carry
materials. Only ports of the same type can be connected.

### Number fields

Type a value, or drag with the **middle mouse button** to scrub it (`Shift`
for larger steps). Bounded parameters also show a slider.

### Viewer

`F5` renders the viewed node, `Esc` cancels. Enable **Auto** to render again
after every change. Wheel zooms, drag pans, `F` fits the image, `1` shows it at
actual size. The arrow in the status bar shows the render log.

### Docking

- Drag a panel tab onto another area to add it as a tab there, or onto the
  highlighted zones at the window edges or center to dock it in an empty area
- Release a tab outside any zone, double click it, or use the `⧉` button to
  float the panel in its own window. Close that window (or press `↵`) to dock
  it back
- Close panels with `✕` or a middle click on their tab, reopen them from the
  **View** menu. **View > Reset Layout** restores the default layout

## Nodes

| Node | Category | Produces |
| --- | --- | --- |
| Sphere | Geometry | `/root/world/geo/<name>`, with the connected material assigned |
| Lambert, Metal, Glass | Shading | `/root/materials/<name>` |
| Camera | Scene | `/root/world/cam/<name>` |
| Merge | Scene | Union of up to 4 input scenes |
| RenderSettings | Render | Samples, bounces and render camera on `/root` |

The node name is also the name of its location. Files are saved as JSON
(**File > Save**).

## Architecture

```
ui/
├── app.py               # Entry point: Qt application and QML engine
├── core/                # Qt independent model
│   ├── nodes.py         # Node types: ports, parameters, cook functions
│   ├── graph.py         # Graph editing, cooking, JSON serialization
│   └── scene.py         # Scene graph: locations and typed attributes
├── render/              # Render engines
│   ├── engine.py        # RenderEngine interface and engine registry
│   └── raito.py         # Raito XML export, runs `raito --scene --output`
├── backend/             # Qt models exposed to QML as the `App` singleton
│   ├── app_controller.py
│   ├── graph_controller.py
│   ├── params_model.py
│   ├── scenegraph_model.py
│   └── render_controller.py
├── qml/
│   ├── Main.qml         # Main window, menus, panel declarations
│   ├── docking/         # DockManager, DockArea, DockPanel, FloatingWindow
│   ├── panels/          # Node graph, node editor, scene graph, attributes, viewer
│   └── controls/        # Theme and parameter editors
└── tests/               # Core unit tests
```

Data flows one way: QML calls slots on the backend, the backend edits the
`core` graph, cooks the viewed node into a `Scene`, then notifies the views.
To render, the Raito engine writes the scene as Raito XML and runs the
executable headless. The image is loaded back as a PPM.

### Adding a node type

Declare a `NodeType` in `core/nodes.py` with its ports, `ParamSpec`s and a
cook function. The node editor builds its widgets from the parameter types
(`float`, `int`, `string`, `bool`, `point`, `color`, `enum`). If Raito should
render it, export it in `render/raito.py`.

### Adding a render engine

Subclass `RenderEngine` (`render/engine.py`), implement `render`, `cancel` and
`is_running`, and decorate it with `@register_engine`. It then appears in the
viewer's engine combobox.

## Tests

```bash
python -m unittest discover -s ui/tests
```

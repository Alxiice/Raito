# Raito UI - Human Handover Guide

> **Document Status:** Local only - Not for commit to repository  
> **Purpose:** Comprehensive guide for new developers to understand, extend, and maintain the Raito GUI  
> **Target Audience:** Developers who need to add new nodes, render methods, or understand the architecture

---

## Table of Contents

1. [Overview](#1-overview)
2. [Architecture](#2-architecture)
3. [Core Components](#3-core-components)
4. [Node System](#4-node-system)
5. [Adding New Nodes](#5-adding-new-nodes)
6. [Adding New Render Methods](#6-adding-new-render-methods)
7. [UI Components](#7-ui-components)
8. [Data Flow](#8-data-flow)
9. [Design Choices](#9-design-choices)
10. [Best Practices](#10-best-practices)

---

## 1. Overview

The Raito UI is a **node-based scene editor** for the Raito render engine, built with:
- **Python 3.9+** - Core logic and business model
- **PySide6 (Qt for Python)** - Qt bindings
- **QML (Qt Quick)** - Declarative UI language for the interface

The editor follows a **Katana-style** workflow: you build scenes as node graphs where each node adds locations to a scene graph, and the viewer renders the scene produced by the *viewed* node.

### Key Features

- Node Graph: Create, connect, and arrange nodes
- Node Editor: Edit parameters of selected nodes
- Scene Graph: Browse locations produced by the viewed node
- Attributes Panel: Inspect attributes of selected scene graph locations
- Viewer: Render with Raito (or other engines)
- Docking System: Fully customizable panel layout

---

## 2. Architecture

The UI follows a **strict separation of concerns** with clear layer boundaries:

```
┌─────────────────────────────────────────────────────────────┐
│                        QML Layer (View)                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │ NodeGraph   │  │ NodeEditor  │  │ SceneGraph/Attributes│ │
│  │ Panel.qml   │  │ Panel.qml   │  │ Panel.qml           │ │
│  └─────────────┘  └─────────────┘  └─────────────────────┘ │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │  Controls   │  │   Docking   │  │      Main.qml       │ │
│  │ (custom)    │  │  System     │  │   (window, menus)   │ │
│  └─────────────┘  └─────────────┘  └─────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                         │  Qt Signals/Slots
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                     Backend Layer (Qt Models)                │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  AppController    - Main application controller           │ │
│  │  GraphController  - Node graph editing & management       │ │
│  │  ParamsModel      - Node parameter editing               │ │
│  │  SceneGraphModel  - Scene graph tree & attributes         │ │
│  │  RenderController - Render engine management              │ │
│  └─────────────────────────────────────────────────────────┘ │
│                     │  Qt Properties & Signals
│                     ▼
└─────────────────────────────────────────────────────────────┘
                         │  Pure Python (no Qt)
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                      Core Layer (Business Logic)               │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  nodes.py    - Node type definitions & cook functions     │ │
│  │  graph.py    - Node graph, connections, serialization    │ │
│  │  scene.py    - Scene graph data structure               │ │
│  │  history.py  - Undo/redo system                          │ │
│  └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                         │  Uses Raito Engine
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                     Render Layer                              │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  engine.py   - RenderEngine base class & registry        │ │
│  │  raito.py    - Raito-specific render engine              │ │
│  └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### Directory Structure

```
ui/
├── __init__.py          # Package init with version
├── __main__.py          # Entry point for python -m ui
├── app.py               # Main application entry point
├── README.md            # UI documentation
├── requirements.txt     # Python dependencies
├── run.bat / run.sh     # Convenience scripts
├── setup.py             # Package installation
│
├── core/                # Qt-independent business logic
│   ├── __init__.py      # Exports all core classes
│   ├── nodes.py         # Node type definitions
│   ├── graph.py         # Node graph management
│   ├── history.py       # Undo/redo system
│   └── scene.py         # Scene graph data structure
│
├── backend/             # Qt models exposed to QML
│   ├── __init__.py      # Exports all backend classes
│   ├── app_controller.py    # Main app controller (App singleton)
│   ├── graph_controller.py  # Node graph controller
│   ├── params_model.py      # Node parameter model
│   ├── render_controller.py # Render engine controller
│   └── scenegraph_model.py # Scene graph tree model
│
├── render/              # Render engines
│   ├── __init__.py      # Exports engines
│   ├── engine.py        # RenderEngine base class
│   └── raito.py         # Raito render engine
│
├── qml/                 # QML UI files
│   ├── Main.qml         # Main window with menus
│   ├── controls/        # Custom QML controls
│   │   ├── ColorField.qml
│   │   ├── FloatField.qml
│   │   ├── Theme.qml
│   │   ├── ToolIconButton.qml
│   │   ├── Vec3Field.qml
│   │   └── qmldir
│   ├── docking/         # Docking system
│   │   ├── DockArea.qml
│   │   ├── DockManager.qml
│   │   ├── DockPanel.qml
│   │   ├── FloatingWindow.qml
│   │   └── qmldir
│   └── panels/          # Main UI panels
│       ├── AttributesPanel.qml
│       ├── NodeEditorPanel.qml
│       ├── NodeGraphPanel.qml
│       ├── NodeItem.qml
│       ├── SceneGraphPanel.qml
│       ├── ViewerPanel.qml
│       ├── Wire.qml
│       └── qmldir
│
└── tests/               # Unit tests
    ├── __init__.py
    ├── test_core.py      # Core layer tests
    └── test_history.py   # History system tests
```

### Key Design Principles

1. **Separation of Concerns**: Core logic has NO Qt dependencies
2. **Data Flow**: One-way flow from QML → Backend → Core
3. **Reactivity**: Qt Signals/Slots for UI updates
4. **Testability**: Core layer can be tested without Qt
5. **Extensibility**: New nodes and engines can be added without modifying existing code

---

## 3. Core Components

### 3.1 Scene Graph (`core/scene.py`)

The **Scene** is the fundamental data structure that represents the 3D scene. It's a flat dictionary of **Location** objects indexed by path.

```python
# Key classes:
- ROOT = "/root"  # The root location
- Attribute      # A typed value (float, int, string, point, color)
- Location       # A scene graph node with a path, type, and attributes
- Scene          # Collection of locations with merge capabilities
```

**Example:**
```python
scene = Scene()
loc = scene.location("/root/world/geo/sphere1", "sphere")
loc.set("geometry.radius", "float", 1.0)
loc.set("geometry.center", "point", [0.0, 0.0, 0.0])
```

### 3.2 Node Types (`core/nodes.py`)

Each node type defines:
- Input/output **ports** (with types)
- **Parameters** with validation rules
- A **cook function** that builds a Scene from parameters and input scenes

**Port Types:**
- `SCENE` - Carries scene data (white color #d8d8d8)
- `MATERIAL` - Carries material data (purple color #c77dff)

**Parameter Types:**
- `float`, `int`, `string`, `bool`
- `point` (3D vector)
- `color` (RGB vector)
- `enum` (dropdown selection)

**Key Classes:**
```python
@dataclass
class ParamSpec:
    name: str          # Internal name
    type: str          # Data type
    default: Any       # Default value
    label: str = ""    # Display label
    min: float = None  # Minimum value (for floats/ints)
    max: float = None  # Maximum value
    options: List[str] = []  # For enum types
    tooltip: str = ""  # Help text

@dataclass
class Port:
    name: str
    type: str          # SCENE or MATERIAL

@dataclass
class NodeType:
    type_id: str       # Unique identifier
    label: str         # Display name
    category: str      # Category in UI menu
    color: str         # Node color in graph
    inputs: List[Port]
    outputs: List[Port]
    params: List[ParamSpec]
    cook: Callable     # Function to build Scene
    description: str   # Tooltip text
```

### 3.3 Node Graph (`core/graph.py`)

Manages the node graph structure with:
- Nodes with positions
- Connections between ports
- View node selection
- Cooking (scene generation)
- Serialization (save/load .rt files)

**Key Methods:**
```python
class Graph:
    # Node management
    add_node(type_id, x, y, name, node_id, params)
    remove_node(node_id)
    rename_node(node_id, name)
    set_param(node_id, name, value)
    
    # Connection management
    can_connect(from_node, from_port, to_node, to_port) -> (bool, str)
    connect(from_node, from_port, to_node, to_port)
    disconnect(to_node, to_port)
    
    # Scene generation
    cook(node_id) -> Scene
    
    # Serialization
    save(path)
    load(path) -> Graph
    to_dict() -> dict
    from_dict(data) -> Graph
```

**File Format:** `.rt` files are JSON documents:
```json
{
  "version": 1,
  "viewNode": "node_id_here",
  "nodes": [
    {
      "id": "uuid",
      "type": "sphere",
      "name": "sphere1",
      "x": 100.0,
      "y": 200.0,
      "params": { "radius": 1.0, "center": [0, 0, 0] }
    }
  ],
  "connections": [
    { "from": ["node1", "out"], "to": ["node2", "in"] }
  ]
}
```

### 3.4 History System (`core/history.py`)

Provides undo/redo functionality through **snapshots** of the graph state.

**Key Concepts:**
- **Merge Key**: Edits with the same merge key within a time delay are grouped
- **Merge Delay**: Default 0.6 seconds (configurable)
- **History Limit**: Default 200 snapshots (configurable)

**How it works:**
1. Before each edit, `GraphController.push()` stores the current graph state
2. On undo, the previous state is restored
3. Node drags and parameter scrubs are merged into single undo steps

---

## 4. Node System

### Current Node Types

| Type | Category | Output | Description |
|------|----------|--------|-------------|
| `sphere` | Geometry | Scene | Creates a sphere at `/root/world/geo/<name>` |
| `lambert_material` | Shading | Material | Diffuse material at `/root/materials/<name>` |
| `metal_material` | Shading | Material | Reflective material |
| `glass_material` | Shading | Material | Dielectric material |
| `camera` | Scene | Scene | Camera at `/root/world/cam/<name>` |
| `merge` | Scene | Scene | Combines up to 4 input scenes |
| `render_settings` | Render | Scene | Sets spp, bounces, camera on `/root` |

### Node Cooking Process

When a node is cooked, its `cook` function receives a `CookContext`:

```python
@dataclass
class CookContext:
    name: str              # Node name
    params: Dict[str, Any] # Parameter values
    inputs: Dict[str, Optional[Scene]]  # Input scenes by port name
    
    def input(self, name: str) -> Optional[Scene]:
        return self.inputs.get(name)
```

**Example: Sphere Node Cooking**
```python
def _cook_sphere(ctx: CookContext) -> Scene:
    scene = Scene()
    path = f"{ROOT}/world/geo/{ctx.name}"
    loc = scene.location(path, "sphere")
    loc.set("geometry.center", "point", ctx.params["center"])
    loc.set("geometry.radius", "float", ctx.params["radius"])
    
    # Handle material connection
    material = ctx.input("material")
    if material is not None:
        scene.merge(material)
        if material.primary:
            loc.set("material.assign", "string", material.primary)
    
    scene.primary = path
    return scene
```

### Scene Graph Structure

The scene graph follows a specific hierarchy:
```
/root
├── renderSettings
│   ├── spp (int)
│   ├── max_bounces (int)
│   └── camera (string, optional)
├── world
│   ├── cam
│   │   └── <camera_name>
│   │       ├── camera.look_from (point)
│   │       ├── camera.look_at (point)
│   │       └── camera.v_fov (float)
│   └── geo
│       └── <shape_name>
│           ├── geometry.type (string)
│           ├── geometry.radius (float)
│           ├── geometry.center (point)
│           └── material.assign (string)
└── materials
    └── <material_name>
        ├── material.shader (string)
        └── material.<param_name> (various types)
```

---

## 5. Adding New Nodes

### Step-by-Step Guide

To add a new node type, you need to:

#### 1. Define the Node Type in `core/nodes.py`

Add a new `NodeType` to the `NODE_TYPES` registry:

```python
NODE_TYPES: Dict[str, NodeType] = _register(
    # ... existing nodes ...
    
    NodeType(
        "plane",                    # type_id - unique identifier
        "Plane",                    # label - display name
        "Geometry",                # category - menu grouping
        "#3f8f5a",                  # color - node color in graph
        inputs=[Port("material", MATERIAL)],  # input ports
        outputs=[Port("out", SCENE)],         # output ports
        params=[
            ParamSpec("center", "point", [0.0, 0.0, 0.0]),
            ParamSpec("normal", "point", [0.0, 1.0, 0.0]),
            ParamSpec("size", "float", 10.0, min=0.0001),
        ],
        cook=_cook_plane,            # cook function
        description="Infinite plane at /root/world/geo",
    ),
)
```

#### 2. Implement the Cook Function

Add the cook function above the registry:

```python
def _cook_plane(ctx: CookContext) -> Scene:
    """Cook a plane geometry node"""
    scene = Scene()
    path = f"{ROOT}/world/geo/{ctx.name}"
    loc = scene.location(path, "plane")
    
    # Set plane parameters
    loc.set("geometry.center", "point", ctx.params["center"])
    loc.set("geometry.normal", "point", ctx.params["normal"])
    loc.set("geometry.size", "float", ctx.params["size"])
    
    # Handle material connection
    material = ctx.input("material")
    if material is not None:
        scene.merge(material)
        if material.primary:
            loc.set("material.assign", "string", material.primary)
    
    scene.primary = path
    return scene
```

#### 3. (Optional) Update Render Export

If the Raito engine needs to export this new node type, update `render/raito.py`:

```python
# In scene_to_xml(), add handling for plane shapes:
def scene_to_xml(scene: Scene) -> str:
    # ... existing code ...
    
    for plane in scene.of_type("plane"):
        material = plane.get("material.assign", "")
        lines.append(f'    <shape type="plane" name={quoteattr(plane.path)}>')
        lines.append("        " + _param("node", "shader", material))
        lines.append("        " + _param("point", "center", plane.get("geometry.center")))
        lines.append("        " + _param("point", "normal", plane.get("geometry.normal")))
        lines.append("        " + _param("float", "size", plane.get("geometry.size")))
        lines.append("    </shape>")
    
    # ... rest of code ...
```

#### 4. Test Your Node

Create a test in `tests/test_core.py`:

```python
class PlaneNodeTest(unittest.TestCase):
    def test_plane_node(self):
        g = Graph()
        plane = g.add_node("plane", params={"size": 5.0})
        material = g.add_node("lambert_material", params={"color": [1.0, 0.0, 0.0]})
        g.connect(material.node_id, "out", plane.node_id, "material")
        
        scene = g.cook(plane.node_id)
        loc = scene.get("/root/world/geo/plane1")
        self.assertEqual(loc.type, "plane")
        self.assertEqual(loc.get("geometry.size"), 5.0)
        self.assertEqual(loc.get("material.assign"), "/root/materials/lambert_material1")
```

Run tests:
```bash
python -m unittest discover -s ui/tests
```

### Complete Example: Adding a Light Node

Here's a complete example of adding a light node:

**1. In `core/nodes.py`:**

```python
def _cook_light(ctx: CookContext) -> Scene:
    """Cook a point light node"""
    scene = Scene()
    path = f"{ROOT}/world/lights/{ctx.name}"
    loc = scene.location(path, "light")
    
    loc.set("light.type", "string", "point")
    loc.set("light.position", "point", ctx.params["position"])
    loc.set("light.intensity", "float", ctx.params["intensity"])
    loc.set("light.color", "color", ctx.params["color"])
    
    scene.primary = path
    return scene

# Add to NODE_TYPES registry:
NodeType(
    "point_light",
    "Point Light",
    "Light",
    "#ffcc00",
    inputs=[],
    outputs=[Port("out", SCENE)],
    params=[
        ParamSpec("position", "point", [0.0, 5.0, 0.0]),
        ParamSpec("intensity", "float", 1.0, min=0.0),
        ParamSpec("color", "color", [1.0, 1.0, 1.0]),
    ],
    cook=_cook_light,
    description="Point light source",
),
```

**2. In `render/raito.py`:**

```python
# Update scene_to_xml to handle lights:
for light in scene.of_type("light"):
    lines.append(f'    <light type={quoteattr(light.get("light.type", "point"))} '
                 f'name={quoteattr(light.path)}>')
    lines.append("        " + _param("point", "position", light.get("light.position")))
    lines.append("        " + _param("float", "intensity", light.get("light.intensity")))
    lines.append("        " + _param("rgb", "color", light.get("light.color")))
    lines.append("    </light>")
```

### Parameter Validation

The `ParamSpec.validate()` method automatically handles type coercion and clamping:

```python
spec = ParamSpec("radius", "float", 1.0, min=0.0001, max=100.0)
result = spec.validate(50.0)      # Returns 50.0
result = spec.validate(-5.0)      # Returns 0.0001 (clamped to min)
result = spec.validate(150.0)     # Returns 100.0 (clamped to max)
result = spec.validate("10")      # Returns 10.0 (coerced from string)
```

---

## 6. Adding New Render Methods

The render system is designed to be **extensible** - you can add new render engines without modifying existing code.

### Step-by-Step Guide

#### 1. Create a New Render Engine Class

Subclass `RenderEngine` and implement the required methods:

```python
# ui/render/my_engine.py
from PySide6.QtCore import Signal
from PySide6.QtGui import QImage
from ui.core import Scene
from .engine import RenderEngine, register_engine

@register_engine
class MyEngine(RenderEngine):
    """My custom render engine"""
    name = "My Engine"  # Displayed in the engine combobox
    
    def __init__(self, parent=None):
        super().__init__(parent)
        self._running = False
        # Add any initialization here
    
    def is_running(self) -> bool:
        return self._running
    
    def render(self, scene: Scene):
        """Start rendering and emit finished/failed when done"""
        if self._running:
            return
        
        self._running = True
        self.started.emit()
        self.log.emit("Starting My Engine render...")
        
        # Run rendering in a separate thread or process
        # When done:
        try:
            image = self._do_render(scene)
            self.finished.emit(image)
        except Exception as e:
            self.failed.emit(str(e))
        finally:
            self._running = False
    
    def cancel(self):
        """Cancel the current render"""
        self._running = False
        self.log.emit("Render cancelled")
    
    def _do_render(self, scene: Scene) -> QImage:
        """Actual rendering logic"""
        # Convert scene to your engine's format
        # Run the render
        # Return QImage
        pass
```

#### 2. The Engine Registry

The `@register_engine` decorator automatically adds your engine to the `ENGINES` registry:

```python
# In ui/render/engine.py
ENGINES: Dict[str, Type[RenderEngine]] = {}

def register_engine(cls: Type[RenderEngine]) -> Type[RenderEngine]:
    ENGINES[cls.name] = cls
    return cls
```

#### 3. Update Imports

Ensure your new engine is imported in `ui/render/__init__.py`:

```python
from .engine import ENGINES, RenderEngine
from .raito import RaitoEngine
from .my_engine import MyEngine  # Add this line
```

#### 4. Engine Requirements

Your render engine must:
1. Emit `started()` when rendering begins
2. Emit `finished(QImage)` when rendering succeeds
3. Emit `failed(str)` when rendering fails
4. Emit `log(str)` for progress/debug messages
5. Implement `is_running()` to return current state
6. Implement `cancel()` to stop rendering

### Complete Example: Simple Preview Engine

Here's a complete example of a simple rasterizer for preview:

```python
# ui/render/preview.py
import numpy as np
from PySide6.QtCore import Qt, QThread, Signal
from PySide6.QtGui import QImage, QPixmap
from ui.core import Scene, ROOT
from .engine import RenderEngine, register_engine

@register_engine
class PreviewEngine(RenderEngine):
    """Simple rasterizer for quick previews"""
    name = "Preview"
    
    def __init__(self, parent=None):
        super().__init__(parent)
        self._running = False
        self._thread = None
    
    def is_running(self) -> bool:
        return self._running
    
    def render(self, scene: Scene):
        if self._running:
            return
        
        self._running = True
        self.started.emit()
        self.log.emit("Starting preview render...")
        
        # Get camera settings
        camera = next(scene.of_type("camera"), None)
        if camera is None:
            self.failed.emit("No camera in scene")
            self._running = False
            return
        
        width, height = 640, 480
        fov = camera.get("camera.v_fov", 60.0)
        
        # Run in a thread to avoid blocking UI
        self._thread = PreviewThread(scene, width, height, fov)
        self._thread.finished.connect(self._on_finished)
        self._thread.failed.connect(self._on_failed)
        self._thread.start()
    
    def cancel(self):
        self._running = False
        if self._thread:
            self._thread.quit()
            self._thread.wait()
        self.log.emit("Preview cancelled")
    
    def _on_finished(self, image: QImage):
        self._running = False
        self.finished.emit(image)
    
    def _on_failed(self, message: str):
        self._running = False
        self.failed.emit(message)


class PreviewThread(QThread):
    finished = Signal(QImage)
    failed = Signal(str)
    
    def __init__(self, scene: Scene, width: int, height: int, fov: float):
        super().__init__()
        self.scene = scene
        self.width = width
        self.height = height
        self.fov = fov
    
    def run(self):
        try:
            image = self._render()
            self.finished.emit(image)
        except Exception as e:
            self.failed.emit(str(e))
    
    def _render(self) -> QImage:
        # Simple rasterization logic
        image = QImage(self.width, self.height, QImage.Format.Format_RGB32)
        image.fill(Qt.black)
        
        # TODO: Implement actual rasterization
        # For now, just draw a gradient
        for y in range(self.height):
            for x in range(self.width):
                # Simple gradient based on position
                r = int(255 * x / self.width)
                g = int(255 * y / self.height)
                b = 128
                image.setPixel(x, y, (r << 16) | (g << 8) | b)
        
        return image
```

### Accessing Scene Data

The `Scene` object provides several methods for querying data:

```python
# Get a specific location
loc = scene.get("/root/world/geo/sphere1")
if loc:
    radius = loc.get("geometry.radius", 1.0)
    
# Iterate over all locations
for path, loc in scene.items():
    print(f"{path}: {loc.type}")
    for name, attr in loc.attributes.items():
        print(f"  {name} = {attr.value} ({attr.type})")

# Find locations by type
for sphere in scene.of_type("sphere"):
    print(f"Sphere at {sphere.path}")

# Get children of a location
for child in scene.children("/root/world"):
    print(f"Child: {child.name}")
```

---

## 7. UI Components

### 7.1 Main Window (`qml/Main.qml`)

The main window contains:
- **Menu Bar**: File, Edit, View, Render menus
- **DockManager**: Manages all dockable panels
- **Status Bar**: Shows messages and render status

**Key Properties:**
- `App.currentFileName` - Current scene file name
- `App.graph` - GraphController instance
- `App.nodeEditor` - ParamsModel instance
- `App.sceneGraph` - SceneGraphModel instance
- `App.render` - RenderController instance

### 7.2 Node Graph Panel (`qml/panels/NodeGraphPanel.qml`)

Interactive canvas for creating and connecting nodes.

**Features:**
- Right-click or Tab: Create node
- Drag from port: Create connection
- Middle drag / Alt+drag: Pan
- Mouse wheel: Zoom
- F: Frame all nodes
- V: View selected node
- Delete: Delete selected node

**Implementation Details:**
- Uses a `Canvas` for drawing the grid
- `NodeItem` components for each node
- `Wire` components for connections
- Custom wire dragging logic
- Merge key system for grouping edits

### 7.3 Node Editor Panel (`qml/panels/NodeEditorPanel.qml`)

Edits parameters of the selected node.

**Features:**
- Automatic parameter type detection
- Sliders for bounded parameters
- Scrubbing with middle mouse button
- Type-specific editors (float, color, vector, enum, etc.)

**Parameter Types and Editors:**
| Type | QML Component | Features |
|------|--------------|----------|
| float/int | `FloatField` + `Slider` | Scrubbing, clamping, slider for bounded values |
| point/color | `Vec3Field` / `ColorField` | 3-component vector editor with color picker |
| bool | `CheckBox` | Toggle switch |
| enum | `ComboBox` | Dropdown selection |
| string | `TextField` | Free text input |

### 7.4 Scene Graph Panel (`qml/panels/SceneGraphPanel.qml`)

Displays the scene graph tree structure.

**Features:**
- Tree view of locations
- Shows location types
- Selecting a location shows its attributes
- Expandable/collapsible tree

### 7.5 Attributes Panel (`qml/panels/AttributesPanel.qml`)

Shows attributes of the selected scene graph location.

**Features:**
- Displays all attributes with their types
- Color swatches for color attributes
- Formatted values (float precision, etc.)

### 7.6 Viewer Panel (`qml/panels/ViewerPanel.qml`)

Displays the rendered image.

**Features:**
- Auto-render toggle
- F5: Manual render
- Esc: Cancel render
- Wheel: Zoom
- Drag: Pan
- F: Fit image
- 1: Actual size

**Image Display:**
Uses `RenderImageProvider` to serve the rendered image as `image://render/<revision>` where revision increments on each render.

### 7.7 Docking System (`qml/docking/`)

Custom docking system that allows:
- Dragging panels to dock/undock
- Tabbing panels together
- Floating panels in separate windows
- Layout saving/restoring

**Key Files:**
- `DockManager.qml` - Main manager
- `DockArea.qml` - Container for docked panels
- `DockPanel.qml` - Individual panel wrapper
- `FloatingWindow.qml` - Window for floating panels

### 7.8 Controls (`qml/controls/`)

Custom QML controls for consistent styling:

- **Theme.qml**: Color palette and styling constants
- **FloatField.qml**: Numeric input with scrubbing
- **Vec3Field.qml**: 3D vector input
- **ColorField.qml**: Color picker with RGB inputs
- **ToolIconButton.qml**: Styled icon buttons

---

## 8. Data Flow

### 8.1 User Interaction Flow

```
User Action (QML)
    ↓ (Qt Signal)
Backend Controller (Python/Qt)
    ↓ (Method call)
Core Logic (Pure Python)
    ↓ (Return value)
Backend Controller
    ↓ (Qt Property/Signal)
QML View (Update)
```

### 8.2 Scene Update Flow

```
1. User edits node graph (adds node, connects, etc.)
    ↓
2. GraphController modifies Graph
    ↓
3. GraphController.emit(changed) signal
    ↓
4. AppController starts cook timer (0ms delay, single shot)
    ↓
5. Cook timer fires → AppController._cook()
    ↓
6. Graph.cook(view_node_id) → Scene
    ↓
7. SceneGraphModel.set_scene(scene)
    ↓
8. RenderController.scene_changed() → triggers render if auto-enabled
    ↓
9. RenderEngine.render(scene) → QImage
    ↓
10. RenderController.finished → RenderImageProvider.image = QImage
    ↓
11. ViewerPanel displays new image
```

### 8.3 Undo/Redo Flow

```
1. User action (e.g., move node)
    ↓
2. GraphController._record("Move Node", graph.to_dict(), merge_key)
    ↓
3. History.push(label, state, merge_key, merge_delay)
    ↓
4. If merge_key matches previous and within delay: skip (merge)
   Else: add to undo stack, clear redo stack
    ↓
5. On undo:
   a. History.undo(current_state) → previous_state
   b. GraphController._restore(previous_state)
   c. Load graph from previous_state
   d. Move current_state to redo stack
    ↓
6. GraphController.emit(changed) → triggers re-cook
```

### 8.4 Render Flow

```
1. AppController._cook() produces new Scene
    ↓
2. AppController._scenegraph.set_scene(scene)
    ↓
3. AppController._render.scene_changed()
    ↓
4. If autoRender=True: RenderController.render()
    ↓
5. RenderController calls RenderEngine.render(scene)
    ↓
6. RenderEngine:
   a. Emit started()
   b. Convert Scene to engine format
   c. Run render process
   d. Emit finished(QImage) or failed(str)
    ↓
7. RenderController:
   a. Update RenderImageProvider.image
   b. Increment revision
   c. Emit imageChanged()
    ↓
8. ViewerPanel displays new image via image://render/<revision>
```

---

## 9. Design Choices

### 9.1 Why QML?

**Pros:**
- Declarative UI language
- Hardware-accelerated rendering
- Smooth animations
- Built-in Qt Quick Controls
- Easy custom component creation
- Good for complex interactive UIs

**Cons:**
- JavaScript (not Python) for UI logic
- Steeper learning curve for Python developers
- Debugging can be challenging

### 9.2 Why PySide6 (vs PyQt)?

- More permissive license (LGPL vs GPL)
- Better Python 3 support
- Official Qt for Python project

### 9.3 Core/Backend Separation

**Benefits:**
- Core can be tested without Qt
- Core can be reused in non-Qt contexts
- Clear boundaries between logic and presentation
- Easier to maintain and extend

### 9.4 Scene Graph Design

**Why a flat dictionary?**
- Simple and efficient
- Easy serialization
- Fast lookups by path
- Easy merging of scenes

**Why location types?**
- Enables type-specific UI and behavior
- Validation of scene structure
- Clear semantics for each location

### 9.5 Undo/Redo Implementation

**Why snapshot-based?**
- Simple to implement
- Handles any change (not just specific operations)
- Easy to extend
- Memory efficient with limits

**Why merge keys?**
- Better user experience (drag = one undo step)
- Reduces undo stack clutter
- Intuitive behavior for continuous actions

### 9.6 Cook Function Design

**Why cook functions instead of imperative updates?**
- Declarative: describe what, not how
- Easy to test
- Easy to extend
- Automatic dependency handling
- Pure functions (no side effects)

---

## 10. Best Practices

### 10.1 Adding New Features

1. **Start with the core**: Implement the business logic first
2. **Write tests**: Test the core logic thoroughly
3. **Add backend**: Create Qt models for the new functionality
4. **Add UI**: Create QML components
5. **Connect**: Wire up signals and slots

### 10.2 Modifying Existing Code

1. **Understand the data flow**: Trace how data moves through the system
2. **Maintain separation**: Don't mix Qt code into core
3. **Use signals/slots**: Don't poll for changes
4. **Keep it reactive**: Update only what changed

### 10.3 Performance Considerations

1. **Batch updates**: Use `beginResetModel()`/`endResetModel()` for list models
2. **Debounce**: Use timers for rapid updates (like parameter scrubbing)
3. **Lazy loading**: Don't compute until needed
4. **Caching**: Cache cooked scenes when possible

### 10.4 Testing

1. **Test core logic**: All business logic should be testable
2. **Test serialization**: Ensure save/load works correctly
3. **Test edge cases**: Invalid inputs, empty graphs, etc.

### 10.5 Debugging

1. **Qt signals**: Use `print()` or logging in signal handlers
2. **QML debugging**: Use `console.log()` in QML
3. **Scene inspection**: Use `scene.to_dict()` to inspect scene structure
4. **Memory profiling**: Use Qt's memory tools for UI issues

### 10.6 Coding Style

**Python:**
- Follow PEP 8
- Use type hints
- Use dataclasses for data containers
- Keep functions small and focused

**QML:**
- Use consistent naming (camelCase for variables, PascalCase for types)
- Keep components small and reusable
- Use `readonly property` for constants
- Use `function` for reusable logic

---

## Appendix A: Quick Reference

### Common Commands

```bash
# Build Raito
cargo build --release

# Install UI dependencies
pip install -r ui/requirements.txt

# Run UI
python -m ui                    # Opens default scene
python -m ui my_scene.rt         # Opens specific scene
python -m ui --reset-layout      # Resets panel layout

# Run tests
python -m unittest discover -s ui/tests

# Run UI with debug output
PYTHONPATH=. python -m ui
```

### File Extensions

- `.rt` - Raito scene files (JSON format)
- `.qml` - Qt Quick UI files
- `.xml` - Raito scene format (for rendering)
- `.ppm` - Rendered image format

### Environment Variables

- `RAITO_BIN` - Path to Raito executable (default: looks in target/release, target/debug, PATH)

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Tab | Search/create node |
| F | Frame all nodes |
| V | View selected node |
| Delete | Delete selected node |
| Ctrl+Z | Undo |
| Ctrl+Y | Redo |
| F5 | Render |
| Esc | Cancel render |
| Middle drag | Pan |
| Alt + drag | Pan |
| Wheel | Zoom |

---

## Appendix B: Glossary

| Term | Definition |
|------|------------|
| **Node** | A processing unit in the node graph |
| **Port** | An input or output connection point on a node |
| **Connection** | A link between an output port and an input port |
| **Cook** | The process of generating a Scene from a node |
| **Scene** | The 3D scene data structure (locations and attributes) |
| **Location** | A node in the scene graph with a path, type, and attributes |
| **Attribute** | A typed value associated with a location |
| **View Node** | The node whose output is displayed in the scene graph and viewer |
| **Graph** | The collection of nodes and connections |
| **Primary** | The main location created by a node (used for material assignment) |

---

## Appendix C: Example Workflows

### Creating a Simple Scene

1. Right-click in Node Graph → Geometry → Sphere
2. Right-click → Shading → Lambert
3. Drag from Lambert's output to Sphere's material input
4. Right-click → Scene → Camera
5. Right-click → Scene → Merge
6. Connect Sphere and Camera to Merge's inputs
7. Select Merge → Press V to view it
8. Press F5 to render

### Adding a New Node Type (Complete Workflow)

1. Edit `ui/core/nodes.py`:
   - Add cook function
   - Add NodeType to registry
2. (Optional) Edit `ui/render/raito.py`:
   - Add XML export for new node type
3. Add test in `ui/tests/test_core.py`
4. Run tests: `python -m unittest discover -s ui/tests`
5. Run UI: `python -m ui`
6. Test new node in UI

### Adding a New Render Engine (Complete Workflow)

1. Create new file in `ui/render/`: `my_engine.py`
2. Implement RenderEngine subclass with @register_engine
3. Update `ui/render/__init__.py` to import new engine
4. Run UI: `python -m ui`
5. Select new engine from Render → Engine menu
6. Test rendering

---

## Final Notes

This document provides a comprehensive overview of the Raito UI architecture. The system is designed to be:

- **Extensible**: Easy to add new nodes and render engines
- **Maintainable**: Clear separation of concerns
- **Testable**: Core logic can be tested independently
- **User-friendly**: Intuitive node-based workflow

For questions or issues not covered in this document:
1. Check the existing code for similar examples
2. Look at the unit tests for usage patterns
3. Read the README.md files in each directory
4. Experiment with the UI to understand the behavior

**Remember:** The key to understanding this system is to follow the data flow: User → QML → Backend → Core → Scene → Render → Image → QML

---

*Document created for developer handover. Last updated: See file modification date.*

"""Node type definitions

Each node type declares its ports, its parameters and a ``cook`` function
building a :class:`Scene` from the node parameters and the scenes cooked by
its connected inputs. Node types match what the Raito engine can render.
"""

from dataclasses import dataclass, field
from typing import Any, Callable, Dict, List, Optional

from .scene import ROOT, Scene


# Port types: a connection is only valid between ports of the same type
SCENE = "scene"
MATERIAL = "material"

PORT_COLORS = {
    SCENE: "#d8d8d8",
    MATERIAL: "#c77dff",
}


@dataclass
class ParamSpec:
    """Declaration of a node parameter

    Types: float, int, string, bool, point, color, enum
    """
    name: str
    type: str
    default: Any
    label: str = ""
    min: Optional[float] = None
    max: Optional[float] = None
    options: List[str] = field(default_factory=list)
    tooltip: str = ""

    def __post_init__(self):
        if not self.label:
            self.label = self.name.replace("_", " ").capitalize()

    def validate(self, value: Any) -> Any:
        """Coerce a value coming from the UI or a file to the param type"""
        if self.type in ("float", "int"):
            value = float(value)
            if self.min is not None:
                value = max(self.min, value)
            if self.max is not None:
                value = min(self.max, value)
            return int(round(value)) if self.type == "int" else value
        if self.type in ("point", "color"):
            values = [float(v) for v in list(value)[:3]]
            if len(values) != 3:
                raise ValueError(f"{self.name} expects 3 values")
            return values
        if self.type == "bool":
            return bool(value)
        if self.type == "enum":
            value = str(value)
            if value not in self.options:
                raise ValueError(f"{value} is not a valid option for {self.name}")
            return value
        return str(value)


@dataclass
class Port:
    name: str
    type: str


@dataclass
class NodeType:
    type_id: str
    label: str
    category: str
    color: str
    inputs: List[Port]
    outputs: List[Port]
    params: List[ParamSpec]
    cook: Callable[["CookContext"], Scene]
    description: str = ""

    def param(self, name: str) -> Optional[ParamSpec]:
        return next((p for p in self.params if p.name == name), None)

    def default_params(self) -> Dict[str, Any]:
        return {p.name: p.validate(p.default) for p in self.params}


@dataclass
class CookContext:
    """What a node sees while cooking"""
    name: str
    params: Dict[str, Any]
    inputs: Dict[str, Optional[Scene]]

    def input(self, name: str) -> Optional[Scene]:
        return self.inputs.get(name)


# ========================================
#  Cook functions
# ========================================

def _cook_sphere(ctx: CookContext) -> Scene:
    scene = Scene()
    path = f"{ROOT}/world/geo/{ctx.name}"
    loc = scene.location(path, "sphere")
    loc.set("geometry.center", "point", ctx.params["center"])
    loc.set("geometry.radius", "float", ctx.params["radius"])
    material = ctx.input("material")
    if material is not None:
        scene.merge(material)
        if material.primary:
            loc.set("material.assign", "string", material.primary)
    scene.primary = path
    return scene


def _material_cook(shader: str, *params: str):
    def cook(ctx: CookContext) -> Scene:
        scene = Scene()
        path = f"{ROOT}/materials/{ctx.name}"
        loc = scene.location(path, "material")
        loc.set("material.shader", "string", shader)
        for name in params:
            spec = NODE_TYPES[f"{shader}_material"].param(name)
            loc.set(f"material.{name}", spec.type, ctx.params[name])
        scene.primary = path
        return scene
    return cook


def _cook_camera(ctx: CookContext) -> Scene:
    scene = Scene()
    path = f"{ROOT}/world/cam/{ctx.name}"
    loc = scene.location(path, "camera")
    loc.set("camera.look_from", "point", ctx.params["look_from"])
    loc.set("camera.look_at", "point", ctx.params["look_at"])
    loc.set("camera.v_fov", "float", ctx.params["v_fov"])
    scene.primary = path
    return scene


def _cook_merge(ctx: CookContext) -> Scene:
    scene = Scene()
    for name in sorted(ctx.inputs):
        if ctx.inputs[name] is not None:
            scene.merge(ctx.inputs[name])
    return scene


def _cook_render_settings(ctx: CookContext) -> Scene:
    scene = Scene()
    upstream = ctx.input("in")
    if upstream is not None:
        scene.merge(upstream)
    root = scene.location(ROOT)
    root.set("renderSettings.spp", "int", ctx.params["spp"])
    root.set("renderSettings.max_bounces", "int", ctx.params["max_bounces"])
    if ctx.params["camera"]:
        root.set("renderSettings.camera", "string", ctx.params["camera"])
    return scene


# ========================================
#  Registry
# ========================================

def _register(*node_types: NodeType) -> Dict[str, NodeType]:
    return {nt.type_id: nt for nt in node_types}


NODE_TYPES: Dict[str, NodeType] = _register(
    NodeType(
        "sphere", "Sphere", "Geometry", "#3f8f5a",
        inputs=[Port("material", MATERIAL)],
        outputs=[Port("out", SCENE)],
        params=[
            ParamSpec("center", "point", [0.0, 0.0, 0.0]),
            ParamSpec("radius", "float", 1.0, min=0.0001),
        ],
        cook=_cook_sphere,
        description="Sphere created under /root/world/geo",
    ),
    NodeType(
        "lambert_material", "Lambert", "Shading", "#7b4fa8",
        inputs=[],
        outputs=[Port("out", MATERIAL)],
        params=[ParamSpec("color", "color", [0.5, 0.5, 0.5])],
        cook=_material_cook("lambert", "color"),
        description="Diffuse material",
    ),
    NodeType(
        "metal_material", "Metal", "Shading", "#7b4fa8",
        inputs=[],
        outputs=[Port("out", MATERIAL)],
        params=[
            ParamSpec("color", "color", [0.7, 0.6, 0.5]),
            ParamSpec("fuzz", "float", 0.0, min=0.0, max=1.0,
                      tooltip="Roughness of the reflection"),
        ],
        cook=_material_cook("metal", "color", "fuzz"),
        description="Reflective material",
    ),
    NodeType(
        "glass_material", "Glass", "Shading", "#7b4fa8",
        inputs=[],
        outputs=[Port("out", MATERIAL)],
        params=[ParamSpec("ior", "float", 1.5, label="IOR", min=1.0, max=3.0,
                          tooltip="Index of refraction")],
        cook=_material_cook("glass", "ior"),
        description="Dielectric material",
    ),
    NodeType(
        "camera", "Camera", "Scene", "#2f7f9f",
        inputs=[],
        outputs=[Port("out", SCENE)],
        params=[
            ParamSpec("look_from", "point", [13.0, 2.0, 3.0]),
            ParamSpec("look_at", "point", [0.0, 0.0, 0.0]),
            ParamSpec("v_fov", "float", 60.0, label="Vertical FOV", min=1.0, max=179.0),
        ],
        cook=_cook_camera,
        description="Camera created under /root/world/cam",
    ),
    NodeType(
        "merge", "Merge", "Scene", "#6b6b6b",
        inputs=[Port(f"in{i}", SCENE) for i in range(1, 5)],
        outputs=[Port("out", SCENE)],
        params=[],
        cook=_cook_merge,
        description="Combine several scenes",
    ),
    NodeType(
        "render_settings", "RenderSettings", "Render", "#a0522d",
        inputs=[Port("in", SCENE)],
        outputs=[Port("out", SCENE)],
        params=[
            ParamSpec("spp", "int", 16, label="Samples per pixel", min=1, max=4096),
            ParamSpec("max_bounces", "int", 8, min=1, max=64),
            ParamSpec("camera", "string", "",
                      tooltip="Camera location to render, first camera if empty"),
        ],
        cook=_cook_render_settings,
        description="Set render settings on /root",
    ),
)

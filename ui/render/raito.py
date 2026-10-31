"""Raito render engine: export the scene to Raito XML and run the renderer"""

import os
import shutil
import sys
import tempfile
from typing import List, Optional
from xml.sax.saxutils import quoteattr

from PySide6.QtCore import QProcess
from PySide6.QtGui import QImage

from ..core import ROOT, Scene
from .engine import RenderEngine, register_engine


REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

DEFAULT_SPP = 16
DEFAULT_MAX_BOUNCES = 8

# Material attributes exported for each Raito shader
SHADER_PARAMS = {
    "lambert": ["color"],
    "metal": ["color", "fuzz"],
    "glass": ["ior"],
}


class ExportError(Exception):
    pass


def _param(param_type: str, name: str, value) -> str:
    if param_type in ("int", "float", "string"):
        return f'<parameter type="{param_type}" name="{name}" value={quoteattr(str(value))}/>'
    if param_type == "point":
        x, y, z = value
        return f'<parameter type="point" name="{name}" x="{x}" y="{y}" z="{z}"/>'
    if param_type == "color":
        r, g, b = value
        return f'<parameter type="rgb" name="{name}" r="{r}" g="{g}" b="{b}"/>'
    if param_type == "node":
        return f'<parameter type="node" name="{name}" id={quoteattr(value)}/>'
    raise ExportError(f"Cannot export parameter type {param_type}")


def scene_to_xml(scene: Scene) -> str:
    """Convert a cooked scene to Raito's XML scene format"""
    root = scene.get(ROOT)
    cameras = list(scene.of_type("camera"))
    camera_path = root.get("renderSettings.camera")
    if camera_path:
        camera = scene.get(camera_path)
        if camera is None or camera.type != "camera":
            raise ExportError(f"Render camera {camera_path} is not a camera location")
    elif cameras:
        camera = cameras[0]
    else:
        raise ExportError("The scene has no camera")

    lines = ['<scene version="0.0.0">', "    <render_settings>"]
    lines.append("        " + _param("int", "spp", root.get("renderSettings.spp", DEFAULT_SPP)))
    lines.append("        " + _param("int", "max_bounces",
                                     root.get("renderSettings.max_bounces", DEFAULT_MAX_BOUNCES)))
    lines += ["    </render_settings>", "    <camera>"]
    lines.append("        " + _param("float", "v_fov", camera.get("camera.v_fov")))
    lines.append("        " + _param("point", "look_from", camera.get("camera.look_from")))
    lines.append("        " + _param("point", "look_at", camera.get("camera.look_at")))
    lines.append("    </camera>")

    materials = set()
    for material in scene.of_type("material"):
        shader = material.get("material.shader")
        if shader not in SHADER_PARAMS:
            raise ExportError(f"Raito has no {shader} shader ({material.path})")
        materials.add(material.path)
        lines.append(f"    <shader type={quoteattr(shader)} name={quoteattr(material.path)}>")
        for name in SHADER_PARAMS[shader]:
            attr = material.attributes[f"material.{name}"]
            lines.append("        " + _param(attr.type, name, attr.value))
        lines.append("    </shader>")

    for sphere in scene.of_type("sphere"):
        # Raito requires a shader parameter, unknown ids use the default shader
        material = sphere.get("material.assign", "")
        lines.append(f"    <shape type=\"sphere\" name={quoteattr(sphere.path)}>")
        lines.append("        " + _param("node", "shader", material if material in materials else ""))
        lines.append("        " + _param("float", "radius", sphere.get("geometry.radius")))
        lines.append("        " + _param("point", "center", sphere.get("geometry.center")))
        lines.append("    </shape>")

    lines.append("</scene>")
    return "\n".join(lines) + "\n"


def find_raito_binary() -> Optional[str]:
    """Locate the raito executable (RAITO_BIN, cargo target dirs, then PATH)"""
    exe = "raito.exe" if sys.platform == "win32" else "raito"
    candidates: List[str] = []
    if os.environ.get("RAITO_BIN"):
        candidates.append(os.environ["RAITO_BIN"])
    candidates += [os.path.join(REPO_ROOT, "target", profile, exe) for profile in ("release", "debug")]
    for candidate in candidates:
        if os.path.isfile(candidate):
            return candidate
    return shutil.which("raito")


@register_engine
class RaitoEngine(RenderEngine):
    name = "Raito"

    def __init__(self, parent=None):
        super().__init__(parent)
        self._process: Optional[QProcess] = None
        self._workdir = tempfile.mkdtemp(prefix="raito_ui_")
        self._scene_path = os.path.join(self._workdir, "scene.xml")
        self._image_path = os.path.join(self._workdir, "render.ppm")

    def is_running(self) -> bool:
        return self._process is not None

    def render(self, scene: Scene):
        if self.is_running():
            return
        binary = find_raito_binary()
        if binary is None:
            self.failed.emit("Raito executable not found: build it with "
                             "'cargo build --release' or set RAITO_BIN")
            return
        try:
            xml = scene_to_xml(scene)
        except ExportError as e:
            self.failed.emit(str(e))
            return
        with open(self._scene_path, "w", encoding="utf-8") as f:
            f.write(xml)
        if os.path.exists(self._image_path):
            os.remove(self._image_path)

        process = QProcess(self)
        process.setProcessChannelMode(QProcess.ProcessChannelMode.MergedChannels)
        process.readyReadStandardOutput.connect(self._read_output)
        process.finished.connect(self._on_finished)
        process.errorOccurred.connect(self._on_error)
        self._process = process
        self.log.emit(f"$ {binary} --scene {self._scene_path} --output {self._image_path}")
        self.started.emit()
        process.start(binary, ["--scene", self._scene_path, "--output", self._image_path])

    def cancel(self):
        if self._process is not None:
            self._process.kill()

    def _read_output(self):
        data = bytes(self._process.readAllStandardOutput()).decode(errors="replace")
        for line in data.splitlines():
            if line.strip():
                self.log.emit(line)

    def _on_error(self, error):
        if error == QProcess.ProcessError.FailedToStart:
            self._process = None
            self.failed.emit("Could not start Raito")

    def _on_finished(self, exit_code, exit_status):
        self._read_output()
        self._process = None
        if exit_status != QProcess.ExitStatus.NormalExit:
            self.failed.emit("Render cancelled")
            return
        image = QImage(self._image_path)
        if exit_code != 0 or image.isNull():
            self.failed.emit(f"Raito failed (exit code {exit_code}), see the render log")
            return
        self.finished.emit(image)

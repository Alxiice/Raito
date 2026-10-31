"""Drives the render engines for the viewer"""

from typing import Callable, Dict, Optional

from PySide6.QtCore import QElapsedTimer, QObject, Property, Signal, Slot, QSize
from PySide6.QtGui import QImage
from PySide6.QtQuick import QQuickImageProvider

from ..core import Scene
from ..render import ENGINES, RenderEngine


class RenderImageProvider(QQuickImageProvider):
    """Serves the last rendered image as image://render/<revision>"""

    def __init__(self):
        super().__init__(QQuickImageProvider.ImageType.Image)
        self.image = QImage()

    def requestImage(self, image_id: str, size: QSize, requested_size: QSize) -> QImage:
        return self.image


class RenderController(QObject):
    enginesChanged = Signal()
    currentEngineChanged = Signal()
    runningChanged = Signal()
    statusChanged = Signal()
    imageChanged = Signal()
    logChanged = Signal()
    autoRenderChanged = Signal()

    def __init__(self, scene_getter: Callable[[], Scene], parent=None):
        super().__init__(parent)
        self.provider = RenderImageProvider()
        self._scene_getter = scene_getter
        self._engines: Dict[str, RenderEngine] = {}
        for name, cls in ENGINES.items():
            engine = cls(self)
            engine.started.connect(self._on_started)
            engine.finished.connect(self._on_finished)
            engine.failed.connect(self._on_failed)
            engine.log.connect(self._append_log)
            self._engines[name] = engine
        self._current = next(iter(self._engines), "")
        self._status = "Ready"
        self._revision = 0
        self._log = []
        self._auto = False
        self._pending = False
        self._timer = QElapsedTimer()

    def _engine(self) -> Optional[RenderEngine]:
        return self._engines.get(self._current)

    def _set_status(self, status: str):
        self._status = status
        self.statusChanged.emit()

    def _append_log(self, line: str):
        self._log.append(line)
        del self._log[:-500]
        self.logChanged.emit()

    # === Engine callbacks ===

    def _on_started(self):
        self._timer.start()
        self._set_status(f"Rendering with {self._current}...")
        self.runningChanged.emit()

    def _on_finished(self, image: QImage):
        self.provider.image = image
        self._revision += 1
        self.imageChanged.emit()
        self.runningChanged.emit()
        self._set_status(f"{image.width()}x{image.height()} rendered in "
                         f"{self._timer.elapsed() / 1000.0:.2f} s")
        self._render_pending()

    def _on_failed(self, message: str):
        self._append_log(f"ERROR: {message}")
        self._set_status(message)
        self.runningChanged.emit()
        self._render_pending()

    def _render_pending(self):
        if self._pending:
            self._pending = False
            self.render()

    # === Properties ===

    @Property("QStringList", notify=enginesChanged)
    def engines(self):
        return list(self._engines)

    def _get_current(self):
        return self._current

    def _set_current(self, name: str):
        if name != self._current and name in self._engines:
            self.cancel()
            self._current = name
            self.currentEngineChanged.emit()

    currentEngine = Property(str, _get_current, _set_current, notify=currentEngineChanged)

    def _get_auto(self):
        return self._auto

    def _set_auto(self, value: bool):
        if value != self._auto:
            self._auto = value
            self.autoRenderChanged.emit()
            if value:
                self.render()

    autoRender = Property(bool, _get_auto, _set_auto, notify=autoRenderChanged)

    @Property(bool, notify=runningChanged)
    def running(self):
        engine = self._engine()
        return engine is not None and engine.is_running()

    @Property(str, notify=statusChanged)
    def status(self):
        return self._status

    @Property(int, notify=imageChanged)
    def imageRevision(self):
        return self._revision

    @Property(QSize, notify=imageChanged)
    def imageSize(self):
        return self.provider.image.size()

    @Property(str, notify=logChanged)
    def logText(self):
        return "\n".join(self._log)

    # === Slots ===

    @Slot()
    def render(self):
        engine = self._engine()
        if engine is None:
            return
        if engine.is_running():
            self._pending = True
            return
        engine.render(self._scene_getter())

    @Slot()
    def cancel(self):
        self._pending = False
        engine = self._engine()
        if engine is not None and engine.is_running():
            engine.cancel()

    @Slot()
    def clearLog(self):
        self._log = []
        self.logChanged.emit()

    def scene_changed(self):
        if self._auto:
            self.render()

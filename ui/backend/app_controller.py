"""Application controller, registered in QML as the ``App`` singleton"""

import os

from PySide6.QtCore import QObject, QTimer, QUrl, Property, Signal, Slot

from ..core import FILE_EXTENSION, Graph, Scene, default_graph
from .graph_controller import GraphController
from .params_model import ParamsModel
from .render_controller import RenderController
from .scenegraph_model import SceneGraphModel


class AppController(QObject):
    currentFileChanged = Signal()
    message = Signal(str, bool, arguments=["text", "isError"])

    def __init__(self, parent=None):
        super().__init__(parent)
        self._graph = GraphController(self)
        self._params = ParamsModel(self._graph, self)
        self._scenegraph = SceneGraphModel(self)
        self._scene = Scene()
        self._render = RenderController(lambda: self._scene, self)
        self._file = ""

        # Cook once per event loop iteration, whatever the number of edits
        self._cook_timer = QTimer(self)
        self._cook_timer.setSingleShot(True)
        self._cook_timer.setInterval(0)
        self._cook_timer.timeout.connect(self._cook)
        self._graph.changed.connect(self._cook_timer.start)
        self._graph.error.connect(lambda text: self.message.emit(text, True))

        self.newScene()

    def _cook(self):
        view = self._graph.viewNodeId
        try:
            self._scene = self._graph.graph.cook(view) if view else Scene()
        except Exception as e:  # a broken node must not take the UI down
            self.message.emit(f"Cook failed: {e}", True)
            self._scene = Scene()
        self._scenegraph.set_scene(self._scene)
        self._render.scene_changed()

    def _set_file(self, path: str):
        self._file = path
        self.currentFileChanged.emit()

    # === Properties ===

    @Property(QObject, constant=True)
    def graph(self):
        return self._graph

    @Property(QObject, constant=True)
    def nodeEditor(self):
        return self._params

    @Property(QObject, constant=True)
    def sceneGraph(self):
        return self._scenegraph

    @Property(QObject, constant=True)
    def render(self):
        return self._render

    @Property(str, notify=currentFileChanged)
    def currentFile(self):
        return self._file

    @Property(str, notify=currentFileChanged)
    def currentFileName(self):
        return os.path.basename(self._file) if self._file else "untitled"

    # === Scene files ===

    @Slot()
    def newScene(self):
        self._graph.set_graph(default_graph())
        self._set_file("")

    @Slot()
    def clearScene(self):
        self._graph.set_graph(Graph())
        self._set_file("")

    @Slot(QUrl)
    def openScene(self, url: QUrl):
        path = url.toLocalFile()
        try:
            graph = Graph.load(path)
        except Exception as e:
            self.message.emit(f"Could not open {path}: {e}", True)
            return
        self._graph.set_graph(graph)
        self._set_file(path)
        self.message.emit(f"Opened {path}", False)

    @Slot(QUrl, result=bool)
    def saveScene(self, url: QUrl) -> bool:
        path = url.toLocalFile() if not url.isEmpty() else self._file
        if not path:
            return False
        if not os.path.splitext(path)[1]:
            path += FILE_EXTENSION
        try:
            self._graph.graph.save(path)
        except OSError as e:
            self.message.emit(f"Could not save {path}: {e}", True)
            return False
        self._set_file(path)
        self.message.emit(f"Saved {path}", False)
        return True

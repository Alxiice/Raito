"""Parameters of the selected node, edited in the node editor"""

from typing import Any, List

from PySide6.QtCore import QAbstractListModel, QModelIndex, Property, Qt, Signal, Slot

from ..core import ParamSpec
from .graph_controller import GraphController, _roles


class ParamsModel(QAbstractListModel):
    ROLES = _roles("name", "label", "type", "value", "min", "max",
                   "hasRange", "options", "tooltip")

    nodeChanged = Signal()

    def __init__(self, graph: GraphController, parent=None):
        super().__init__(parent)
        self._graph = graph
        self._node_id = ""
        self._specs: List[ParamSpec] = []
        graph.selectedNodeIdChanged.connect(self._on_selection)
        graph.paramChanged.connect(self._on_param_changed)
        graph.nodeRenamed.connect(self._on_renamed)

    def _node(self):
        return self._graph.graph.nodes.get(self._node_id)

    def _on_selection(self):
        self.beginResetModel()
        self._node_id = self._graph.selectedNodeId
        node = self._node()
        self._specs = list(node.node_type.params) if node else []
        self.endResetModel()
        self.nodeChanged.emit()

    def _on_param_changed(self, node_id: str, name: str):
        if node_id != self._node_id:
            return
        for row, spec in enumerate(self._specs):
            if spec.name == name:
                index = self.index(row)
                self.dataChanged.emit(index, index)

    def _on_renamed(self, node_id: str):
        if node_id == self._node_id:
            self.nodeChanged.emit()

    # === Model ===

    def roleNames(self):
        return self.ROLES

    def rowCount(self, parent=QModelIndex()):
        return 0 if parent.isValid() else len(self._specs)

    def data(self, index, role=Qt.ItemDataRole.DisplayRole):
        node = self._node()
        if not index.isValid() or role not in self.ROLES or node is None:
            return None
        spec = self._specs[index.row()]
        name = bytes(self.ROLES[role]).decode()
        if name == "value":
            return node.params[spec.name]
        if name == "hasRange":
            return spec.min is not None and spec.max is not None
        if name in ("min", "max"):
            value = getattr(spec, name)
            return float(value) if value is not None else 0.0
        return getattr(spec, name)

    # === Selected node ===

    @Property(bool, notify=nodeChanged)
    def hasNode(self):
        return self._node() is not None

    @Property(str, notify=nodeChanged)
    def nodeId(self):
        return self._node_id

    @Property(str, notify=nodeChanged)
    def nodeName(self):
        node = self._node()
        return node.name if node else ""

    @Property(str, notify=nodeChanged)
    def nodeType(self):
        node = self._node()
        return node.node_type.label if node else ""

    @Property(str, notify=nodeChanged)
    def nodeColor(self):
        node = self._node()
        return node.node_type.color if node else "#000000"

    @Property(str, notify=nodeChanged)
    def nodeDescription(self):
        node = self._node()
        return node.node_type.description if node else ""

    @Slot(int, "QVariant")
    def setValue(self, row: int, value: Any):
        if self._node() is not None and 0 <= row < len(self._specs):
            self._graph.setParam(self._node_id, self._specs[row].name, value)

    @Slot(str)
    def rename(self, name: str):
        if self._node() is not None:
            self._graph.renameNode(self._node_id, name)
            self.nodeChanged.emit()

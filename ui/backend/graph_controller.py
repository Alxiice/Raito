"""Node graph exposed to QML"""

from typing import Any, Dict, List, Optional

from PySide6.QtCore import (QAbstractListModel, QByteArray, QModelIndex,
                            QObject, Property, Qt, Signal, Slot)

from ..core import NODE_TYPES, PORT_COLORS, Graph, GraphError, History


def _roles(*names: str) -> Dict[int, QByteArray]:
    return {Qt.ItemDataRole.UserRole + i: QByteArray(name.encode())
            for i, name in enumerate(names)}


class NodeListModel(QAbstractListModel):
    ROLES = _roles("nodeId", "name", "typeId", "typeLabel", "nodeColor",
                   "posX", "posY", "inputs", "outputs")

    def __init__(self, parent=None):
        super().__init__(parent)
        self._graph = Graph()
        self._ids: List[str] = []
        self._role_ids = {bytes(v).decode(): k for k, v in self.ROLES.items()}

    def set_graph(self, graph: Graph):
        self.beginResetModel()
        self._graph = graph
        self._ids = list(graph.nodes)
        self.endResetModel()

    def roleNames(self):
        return self.ROLES

    def rowCount(self, parent=QModelIndex()):
        return 0 if parent.isValid() else len(self._ids)

    def data(self, index, role=Qt.ItemDataRole.DisplayRole):
        if not index.isValid() or role not in self.ROLES:
            return None
        node = self._graph.nodes[self._ids[index.row()]]
        name = bytes(self.ROLES[role]).decode()
        if name == "nodeId":
            return node.node_id
        if name == "name":
            return node.name
        if name == "typeId":
            return node.type_id
        if name == "typeLabel":
            return node.node_type.label
        if name == "nodeColor":
            return node.node_type.color
        if name == "posX":
            return node.x
        if name == "posY":
            return node.y
        if name == "inputs":
            return [{"name": p.name, "type": p.type, "color": PORT_COLORS[p.type],
                     "connected": self._graph.input_connection(node.node_id, p.name) is not None}
                    for p in node.node_type.inputs]
        if name == "outputs":
            return [{"name": p.name, "type": p.type, "color": PORT_COLORS[p.type]}
                    for p in node.node_type.outputs]
        return None

    def append(self, node_id: str):
        self.beginInsertRows(QModelIndex(), len(self._ids), len(self._ids))
        self._ids.append(node_id)
        self.endInsertRows()

    def remove(self, node_id: str):
        row = self._ids.index(node_id)
        self.beginRemoveRows(QModelIndex(), row, row)
        self._ids.pop(row)
        self.endRemoveRows()

    def refresh(self, node_id: str, *roles: str):
        if node_id not in self._ids:
            return
        index = self.index(self._ids.index(node_id))
        self.dataChanged.emit(index, index, [self._role_ids[r] for r in roles])


class ConnectionListModel(QAbstractListModel):
    ROLES = _roles("fromNode", "fromIndex", "toNode", "toIndex", "wireColor")

    def __init__(self, parent=None):
        super().__init__(parent)
        self._graph = Graph()

    def set_graph(self, graph: Graph):
        self.beginResetModel()
        self._graph = graph
        self.endResetModel()

    def reset(self):
        self.beginResetModel()
        self.endResetModel()

    def roleNames(self):
        return self.ROLES

    def rowCount(self, parent=QModelIndex()):
        return 0 if parent.isValid() else len(self._graph.connections)

    def data(self, index, role=Qt.ItemDataRole.DisplayRole):
        if not index.isValid() or role not in self.ROLES:
            return None
        c = self._graph.connections[index.row()]
        name = bytes(self.ROLES[role]).decode()
        from_type = self._graph.nodes[c.from_node].node_type
        to_type = self._graph.nodes[c.to_node].node_type
        if name == "fromNode":
            return c.from_node
        if name == "fromIndex":
            return [p.name for p in from_type.outputs].index(c.from_port)
        if name == "toNode":
            return c.to_node
        if name == "toIndex":
            return [p.name for p in to_type.inputs].index(c.to_port)
        if name == "wireColor":
            port = next(p for p in to_type.inputs if p.name == c.to_port)
            return PORT_COLORS[port.type]
        return None


class GraphController(QObject):
    """Edits the node graph and notifies the views"""

    selectedNodeIdChanged = Signal()
    viewNodeIdChanged = Signal()
    historyChanged = Signal()
    # Emitted when a new graph is loaded (not on undo / redo)
    graphReplaced = Signal()
    # Emitted when the scene produced by the graph may have changed
    changed = Signal()
    paramChanged = Signal(str, str)
    nodeRenamed = Signal(str)
    nodeRemoved = Signal(str)
    error = Signal(str)

    def __init__(self, parent=None):
        super().__init__(parent)
        self.graph = Graph()
        self._nodes = NodeListModel(self)
        self._connections = ConnectionListModel(self)
        self._selected = ""
        self._view = ""
        self._categories = self._build_categories()
        self._history = History()
        self._interaction = 0
        self._interacting = False

    @staticmethod
    def _build_categories() -> List[Dict[str, Any]]:
        categories: Dict[str, List[Dict[str, str]]] = {}
        for node_type in NODE_TYPES.values():
            categories.setdefault(node_type.category, []).append({
                "typeId": node_type.type_id, "label": node_type.label,
                "category": node_type.category, "color": node_type.color,
                "description": node_type.description,
            })
        return [{"name": name, "types": types} for name, types in categories.items()]

    def set_graph(self, graph: Graph):
        """Load a new graph and forget the undo history"""
        self._load(graph)
        self._history.clear()
        self.historyChanged.emit()
        self.graphReplaced.emit()

    def _load(self, graph: Graph, selected: str = ""):
        self.graph = graph
        self._nodes.set_graph(graph)
        self._connections.set_graph(graph)
        self.select("")
        self.select(selected)
        self._view = graph.view_node
        self.viewNodeIdChanged.emit()
        self.changed.emit()

    # === Undo / redo ===

    def _record(self, label: str, before: Dict[str, Any], merge_key=None, merge_delay=None):
        """Push the graph state taken before a successful edit"""
        self._history.push(label, before, merge_key, merge_delay)
        self.historyChanged.emit()

    def _restore(self, state: Optional[Dict[str, Any]]):
        if state is not None:
            self._load(Graph.from_dict(state), self._selected)
            self.historyChanged.emit()

    @Slot()
    def undo(self):
        self._restore(self._history.undo(self.graph.to_dict()))

    @Slot()
    def redo(self):
        self._restore(self._history.redo(self.graph.to_dict()))

    @Property(bool, notify=historyChanged)
    def canUndo(self):
        return self._history.can_undo

    @Property(bool, notify=historyChanged)
    def canRedo(self):
        return self._history.can_redo

    @Property(str, notify=historyChanged)
    def undoText(self):
        return self._history.undo_label

    @Property(str, notify=historyChanged)
    def redoText(self):
        return self._history.redo_label

    # === Properties ===

    @Property(QObject, constant=True)
    def nodes(self):
        return self._nodes

    @Property(QObject, constant=True)
    def connections(self):
        return self._connections

    @Property("QVariantList", constant=True)
    def nodeCategories(self):
        return self._categories

    @Property(str, notify=selectedNodeIdChanged)
    def selectedNodeId(self):
        return self._selected

    @Property(str, notify=viewNodeIdChanged)
    def viewNodeId(self):
        return self._view

    # === Slots ===

    @Slot(str)
    def select(self, node_id: str):
        if node_id != self._selected and (not node_id or node_id in self.graph.nodes):
            self._selected = node_id
            self.selectedNodeIdChanged.emit()

    @Slot()
    def beginInteraction(self):
        """Edits until endInteraction() are undone as one step (node drags)"""
        self._interaction += 1
        self._interacting = True

    @Slot()
    def endInteraction(self):
        self._interacting = False

    def _merge(self, key):
        """Merge key and delay for an edit that can be merged with the next ones"""
        if self._interacting:
            return ("interaction", self._interaction), float("inf")
        return key, None

    def _set_view(self, node_id: str):
        self._view = self.graph.view_node = node_id
        self.viewNodeIdChanged.emit()
        self.changed.emit()

    @Slot(str)
    def setViewNode(self, node_id: str):
        if node_id != self._view and (not node_id or node_id in self.graph.nodes):
            self._record("View Node", self.graph.to_dict())
            self._set_view(node_id)

    @Slot(str, float, float, result=str)
    def createNode(self, type_id: str, x: float, y: float) -> str:
        before = self.graph.to_dict()
        try:
            node = self.graph.add_node(type_id, x, y)
        except GraphError as e:
            self.error.emit(str(e))
            return ""
        self._record(f"Create {node.name}", before)
        self._nodes.append(node.node_id)
        self.select(node.node_id)
        if not self._view:
            self._set_view(node.node_id)
        return node.node_id

    @Slot(str)
    def deleteNode(self, node_id: str):
        node = self.graph.nodes.get(node_id)
        if node is None:
            return
        self._record(f"Delete {node.name}", self.graph.to_dict())
        if self._selected == node_id:
            self.select("")
        downstream = {c.to_node for c in self.graph.connections if c.from_node == node_id}
        self.graph.remove_node(node_id)
        self._nodes.remove(node_id)
        self._connections.reset()
        for other in downstream:
            self._nodes.refresh(other, "inputs")
        self.nodeRemoved.emit(node_id)
        if self._view == node_id:
            self._set_view("")
        self.changed.emit()

    @Slot(str, float, float)
    def moveNode(self, node_id: str, x: float, y: float):
        node = self.graph.nodes.get(node_id)
        if node is not None and (node.x, node.y) != (x, y):
            self._record(f"Move {node.name}", self.graph.to_dict(),
                         *self._merge(("move", node_id)))
            node.x, node.y = x, y
            self._nodes.refresh(node_id, "posX", "posY")

    @Slot(str, str, result=str)
    def renameNode(self, node_id: str, name: str) -> str:
        node = self.graph.nodes.get(node_id)
        if node is None:
            return ""
        before = self.graph.to_dict()
        old_name = node.name
        new_name = self.graph.rename_node(node_id, name)
        if new_name == old_name:
            return new_name
        self._record(f"Rename {old_name}", before)
        self._nodes.refresh(node_id, "name")
        self.nodeRenamed.emit(node_id)
        self.changed.emit()
        return new_name

    @Slot(str, str, "QVariant", result=bool)
    def setParam(self, node_id: str, name: str, value: Any) -> bool:
        node = self.graph.nodes.get(node_id)
        before = self.graph.to_dict()
        old_value = node.params.get(name) if node else None
        try:
            new_value = self.graph.set_param(node_id, name, value)
        except (GraphError, KeyError, ValueError, TypeError) as e:
            self.error.emit(f"Invalid value for {name}: {e}")
            self.paramChanged.emit(node_id, name)  # restore the editor value
            return False
        if new_value != old_value:
            self._record(f"Edit {node.name}.{name}", before,
                         *self._merge(("param", node_id, name)))
        self.paramChanged.emit(node_id, name)
        self.changed.emit()
        return True

    @Slot(str, str, str, str, result=bool)
    def canConnect(self, from_node: str, from_port: str, to_node: str, to_port: str) -> bool:
        return self.graph.can_connect(from_node, from_port, to_node, to_port)[0]

    @Slot(str, str, str, str, result=bool)
    def connectPorts(self, from_node: str, from_port: str, to_node: str, to_port: str) -> bool:
        before = self.graph.to_dict()
        try:
            self.graph.connect(from_node, from_port, to_node, to_port)
        except GraphError as e:
            self.error.emit(str(e))
            return False
        self._record("Connect", before, *self._merge(None))
        self._connections.reset()
        self._nodes.refresh(to_node, "inputs")
        self.changed.emit()
        return True

    @Slot(str, str, result="QVariantMap")
    def disconnectInput(self, node_id: str, port: str) -> Dict[str, str]:
        """Remove the connection of an input, return its source port"""
        before = self.graph.to_dict()
        connection = self.graph.disconnect(node_id, port)
        if connection is None:
            return {}
        self._record("Disconnect", before, *self._merge(None))
        self._connections.reset()
        self._nodes.refresh(node_id, "inputs")
        self.changed.emit()
        return {"fromNode": connection.from_node, "fromPort": connection.from_port}

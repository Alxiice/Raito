"""Scene graph tree and attributes of the selected location"""

from typing import Dict, List, Optional

from PySide6.QtCore import (QAbstractItemModel, QAbstractListModel, QModelIndex,
                            QObject, Property, Qt, Signal, Slot)
from PySide6.QtGui import QColor

from ..core import ROOT, Attribute, Scene
from .graph_controller import _roles


class _TreeItem:
    def __init__(self, path: str, loc_type: str, parent: Optional["_TreeItem"], row: int):
        self.path = path
        self.name = path.rsplit("/", 1)[-1]
        self.type = loc_type
        self.parent = parent
        self.row = row
        self.children: List["_TreeItem"] = []


def format_value(attr: Attribute) -> str:
    if attr.type == "float":
        return f"{attr.value:g}"
    if attr.type in ("point", "color"):
        return ", ".join(f"{v:g}" for v in attr.value)
    return str(attr.value)


class AttributesModel(QAbstractListModel):
    ROLES = _roles("name", "group", "type", "value", "swatch")

    def __init__(self, parent=None):
        super().__init__(parent)
        self._rows = []

    def set_attributes(self, attributes: Dict[str, Attribute]):
        self.beginResetModel()
        self._rows = []
        for name in sorted(attributes):
            attr = attributes[name]
            group, _, short = name.rpartition(".")
            swatch = QColor.fromRgbF(*[min(max(v, 0.0), 1.0) for v in attr.value]) \
                if attr.type == "color" else QColor()
            self._rows.append({"name": short, "group": group or "-", "type": attr.type,
                               "value": format_value(attr), "swatch": swatch})
        self.endResetModel()

    def roleNames(self):
        return self.ROLES

    def rowCount(self, parent=QModelIndex()):
        return 0 if parent.isValid() else len(self._rows)

    def data(self, index, role=Qt.ItemDataRole.DisplayRole):
        if not index.isValid() or role not in self.ROLES:
            return None
        return self._rows[index.row()][bytes(self.ROLES[role]).decode()]


class SceneGraphModel(QAbstractItemModel):
    ROLES = _roles("name", "locationType", "path", "childCount")

    selectedPathChanged = Signal()
    sceneChanged = Signal()

    def __init__(self, parent=None):
        super().__init__(parent)
        self._scene = Scene()
        self._root: Optional[_TreeItem] = None
        self._items: Dict[str, _TreeItem] = {}
        self._selected = ""
        self._attributes = AttributesModel(self)
        self.set_scene(Scene())

    def set_scene(self, scene: Scene):
        self.beginResetModel()
        self._scene = scene
        self._items = {}
        self._root = self._build(ROOT, None, 0)
        self.endResetModel()
        self.sceneChanged.emit()
        # Keep the selection when the location still exists
        selected = self._selected if self._selected in self._items else ""
        self._selected = None
        self.select(selected)

    def _build(self, path: str, parent: Optional[_TreeItem], row: int) -> _TreeItem:
        item = _TreeItem(path, self._scene.get(path).type, parent, row)
        self._items[path] = item
        item.children = [self._build(child.path, item, r)
                         for r, child in enumerate(self._scene.children(path))]
        return item

    # === Model ===

    def roleNames(self):
        return self.ROLES

    def index(self, row, column, parent=QModelIndex()):
        if column != 0:
            return QModelIndex()
        if not parent.isValid():
            return self.createIndex(row, 0, self._root) if row == 0 else QModelIndex()
        item: _TreeItem = parent.internalPointer()
        if 0 <= row < len(item.children):
            return self.createIndex(row, 0, item.children[row])
        return QModelIndex()

    def parent(self, index):
        if not index.isValid():
            return QModelIndex()
        item: _TreeItem = index.internalPointer()
        if item.parent is None:
            return QModelIndex()
        return self.createIndex(item.parent.row, 0, item.parent)

    def rowCount(self, parent=QModelIndex()):
        if not parent.isValid():
            return 1
        return len(parent.internalPointer().children)

    def columnCount(self, parent=QModelIndex()):
        return 1

    def data(self, index, role=Qt.ItemDataRole.DisplayRole):
        if not index.isValid():
            return None
        item: _TreeItem = index.internalPointer()
        name = bytes(self.ROLES.get(role, b"")).decode()
        if name == "name" or role == Qt.ItemDataRole.DisplayRole:
            return item.name
        if name == "locationType":
            return item.type
        if name == "path":
            return item.path
        if name == "childCount":
            return len(item.children)
        return None

    # === Selection ===

    @Property(QObject, constant=True)
    def attributes(self):
        return self._attributes

    @Property(str, notify=selectedPathChanged)
    def selectedPath(self):
        return self._selected

    @Property(str, notify=selectedPathChanged)
    def selectedType(self):
        loc = self._scene.get(self._selected)
        return loc.type if loc else ""

    @Property(int, notify=sceneChanged)
    def locationCount(self):
        return len(self._items)

    @Slot(str)
    def select(self, path: str):
        if path == self._selected:
            return
        self._selected = path
        loc = self._scene.get(path)
        self._attributes.set_attributes(loc.attributes if loc else {})
        self.selectedPathChanged.emit()

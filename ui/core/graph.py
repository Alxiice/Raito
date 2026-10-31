"""Node graph: nodes, connections, cooking and serialization"""

import copy
import json
import re
import uuid
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, Tuple

from .nodes import NODE_TYPES, CookContext, NodeType
from .scene import Scene


FILE_VERSION = 1
# Scene files are JSON documents with their own extension
FILE_EXTENSION = ".rt"


class GraphError(Exception):
    pass


@dataclass
class Node:
    node_id: str
    type_id: str
    name: str
    x: float = 0.0
    y: float = 0.0
    params: Dict[str, Any] = field(default_factory=dict)

    @property
    def node_type(self) -> NodeType:
        return NODE_TYPES[self.type_id]


@dataclass(frozen=True)
class Connection:
    """Link from an output port to an input port"""
    from_node: str
    from_port: str
    to_node: str
    to_port: str


class Graph:
    def __init__(self):
        self.nodes: Dict[str, Node] = {}
        self.connections: List[Connection] = []
        # Node whose output is displayed in the scene graph and the viewer
        self.view_node: str = ""

    # === Nodes ===

    def unique_name(self, base: str, ignore: Optional[str] = None) -> str:
        base = re.sub(r"[^A-Za-z0-9_]", "_", base).strip("_") or "node"
        used = {n.name for n in self.nodes.values() if n.node_id != ignore}
        if base not in used:
            return base
        stem = base.rstrip("0123456789") or base
        index = 1
        while f"{stem}{index}" in used:
            index += 1
        return f"{stem}{index}"

    def add_node(self, type_id: str, x: float = 0.0, y: float = 0.0,
                 name: Optional[str] = None, node_id: Optional[str] = None,
                 params: Optional[Dict[str, Any]] = None) -> Node:
        if type_id not in NODE_TYPES:
            raise GraphError(f"Unknown node type {type_id}")
        node_type = NODE_TYPES[type_id]
        node = Node(
            node_id=node_id or uuid.uuid4().hex,
            type_id=type_id,
            name=self.unique_name(name or f"{node_type.label.lower()}1"),
            x=x, y=y,
            params=node_type.default_params(),
        )
        for key, value in (params or {}).items():
            spec = node_type.param(key)
            if spec is not None:
                node.params[key] = spec.validate(value)
        self.nodes[node.node_id] = node
        return node

    def remove_node(self, node_id: str):
        self.nodes.pop(node_id, None)
        if self.view_node == node_id:
            self.view_node = ""
        self.connections = [c for c in self.connections
                            if node_id not in (c.from_node, c.to_node)]

    def rename_node(self, node_id: str, name: str) -> str:
        node = self.nodes[node_id]
        node.name = self.unique_name(name, ignore=node_id)
        return node.name

    def set_param(self, node_id: str, name: str, value: Any) -> Any:
        node = self.nodes[node_id]
        spec = node.node_type.param(name)
        if spec is None:
            raise GraphError(f"{node.name} has no parameter {name}")
        node.params[name] = spec.validate(value)
        return node.params[name]

    # === Connections ===

    def input_connection(self, node_id: str, port: str) -> Optional[Connection]:
        return next((c for c in self.connections
                     if c.to_node == node_id and c.to_port == port), None)

    def upstream(self, node_id: str) -> List[str]:
        """All nodes feeding node_id, directly or not"""
        seen, stack = set(), [node_id]
        while stack:
            current = stack.pop()
            for c in self.connections:
                if c.to_node == current and c.from_node not in seen:
                    seen.add(c.from_node)
                    stack.append(c.from_node)
        return list(seen)

    def can_connect(self, from_node: str, from_port: str,
                    to_node: str, to_port: str) -> Tuple[bool, str]:
        if from_node not in self.nodes or to_node not in self.nodes:
            return False, "Unknown node"
        if from_node == to_node:
            return False, "Cannot connect a node to itself"
        out_port = next((p for p in self.nodes[from_node].node_type.outputs
                         if p.name == from_port), None)
        in_port = next((p for p in self.nodes[to_node].node_type.inputs
                        if p.name == to_port), None)
        if out_port is None or in_port is None:
            return False, "Unknown port"
        if out_port.type != in_port.type:
            return False, f"Cannot connect {out_port.type} to {in_port.type}"
        if to_node in self.upstream(from_node):
            return False, "Connection would create a cycle"
        return True, ""

    def connect(self, from_node: str, from_port: str,
                to_node: str, to_port: str) -> Connection:
        """Connect two ports, replacing the existing input connection"""
        ok, reason = self.can_connect(from_node, from_port, to_node, to_port)
        if not ok:
            raise GraphError(reason)
        self.disconnect(to_node, to_port)
        connection = Connection(from_node, from_port, to_node, to_port)
        self.connections.append(connection)
        return connection

    def disconnect(self, to_node: str, to_port: str) -> Optional[Connection]:
        connection = self.input_connection(to_node, to_port)
        if connection is not None:
            self.connections.remove(connection)
        return connection

    # === Cooking ===

    def cook(self, node_id: str) -> Scene:
        """Build the scene seen at the output of a node"""
        cache: Dict[str, Scene] = {}

        def cook_node(nid: str) -> Scene:
            if nid not in cache:
                node = self.nodes[nid]
                inputs = {}
                for port in node.node_type.inputs:
                    connection = self.input_connection(nid, port.name)
                    inputs[port.name] = cook_node(connection.from_node) if connection else None
                cache[nid] = node.node_type.cook(
                    CookContext(node.name, dict(node.params), inputs))
            return cache[nid]

        return cook_node(node_id)

    # === Serialization ===

    def to_dict(self) -> Dict[str, Any]:
        return {
            "version": FILE_VERSION,
            "viewNode": self.view_node,
            "nodes": [
                {"id": n.node_id, "type": n.type_id, "name": n.name,
                 "x": n.x, "y": n.y, "params": copy.deepcopy(n.params)}
                for n in self.nodes.values()
            ],
            "connections": [
                {"from": [c.from_node, c.from_port], "to": [c.to_node, c.to_port]}
                for c in self.connections
            ],
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Graph":
        graph = cls()
        for n in data.get("nodes", []):
            graph.add_node(n["type"], n.get("x", 0.0), n.get("y", 0.0),
                           name=n.get("name"), node_id=n.get("id"),
                           params=n.get("params"))
        for c in data.get("connections", []):
            graph.connect(*c["from"], *c["to"])
        if data.get("viewNode") in graph.nodes:
            graph.view_node = data["viewNode"]
        return graph

    def save(self, path: str):
        with open(path, "w", encoding="utf-8") as f:
            json.dump(self.to_dict(), f, indent=2)

    @classmethod
    def load(cls, path: str) -> "Graph":
        with open(path, "r", encoding="utf-8") as f:
            return cls.from_dict(json.load(f))


def default_graph() -> Graph:
    """Raito's reference scene (tests/scenes/scene_000.xml) as a node graph"""
    g = Graph()
    materials = {
        "ground_mat": g.add_node("lambert_material", 0, 0, "ground_mat", params={"color": [0.5, 0.5, 0.5]}),
        "lambert0": g.add_node("lambert_material", 0, 110, "lambert0", params={"color": [0.4, 0.2, 0.1]}),
        "glass0": g.add_node("glass_material", 0, 220, "glass0"),
        "metal0": g.add_node("metal_material", 0, 330, "metal0", params={"color": [0.7, 0.6, 0.5], "fuzz": 0.0}),
    }
    spheres = [
        ("ground", "ground_mat", [0.0, -1000.0, 0.0], 1000.0),
        ("sphere_diffuse", "lambert0", [-4.0, 1.0, 0.0], 1.0),
        ("sphere_glass", "glass0", [0.0, 1.0, 0.0], 1.0),
        ("sphere_metal", "metal0", [4.0, 1.0, 0.0], 1.0),
    ]
    merge = g.add_node("merge", 480, 160, "merge")
    for index, (name, material, center, radius) in enumerate(spheres):
        sphere = g.add_node("sphere", 240, index * 110, name,
                            params={"center": center, "radius": radius})
        g.connect(materials[material].node_id, "out", sphere.node_id, "material")
        g.connect(sphere.node_id, "out", merge.node_id, f"in{index + 1}")
    camera = g.add_node("camera", 480, -60, "camera")
    top_merge = g.add_node("merge", 690, 40, "merge_all")
    g.connect(camera.node_id, "out", top_merge.node_id, "in1")
    g.connect(merge.node_id, "out", top_merge.node_id, "in2")
    settings = g.add_node("render_settings", 900, 40, "render_settings", params={"spp": 16})
    g.connect(top_merge.node_id, "out", settings.node_id, "in")
    g.view_node = settings.node_id
    return g

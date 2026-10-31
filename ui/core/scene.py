"""Scene graph produced by cooking a node graph

A scene is a flat dictionary of locations indexed by their path
(e.g. ``/root/world/geo/sphere1``). Each location has a type and a set of
typed attributes (``geometry.radius``, ``material.color``...).
"""

from dataclasses import dataclass, field
from typing import Any, Dict, Iterator, List, Optional, Tuple


ROOT = "/root"


@dataclass
class Attribute:
    """A typed attribute value (types: float, int, string, point, color)"""
    type: str
    value: Any


@dataclass
class Location:
    """A scene graph location"""
    path: str
    type: str = "group"
    attributes: Dict[str, Attribute] = field(default_factory=dict)

    @property
    def name(self) -> str:
        return self.path.rsplit("/", 1)[-1]

    @property
    def parent_path(self) -> Optional[str]:
        if self.path == ROOT:
            return None
        return self.path.rsplit("/", 1)[0]

    def set(self, name: str, attr_type: str, value: Any):
        self.attributes[name] = Attribute(attr_type, value)

    def get(self, name: str, default: Any = None) -> Any:
        attr = self.attributes.get(name)
        return attr.value if attr is not None else default


class Scene:
    """Flat collection of locations, always containing /root"""

    def __init__(self):
        self.locations: Dict[str, Location] = {ROOT: Location(ROOT)}
        # Location created by the node that produced this scene, used to
        # link a material to the shape it is connected to
        self.primary: Optional[str] = None

    def location(self, path: str, loc_type: Optional[str] = None) -> Location:
        """Get or create a location (and its missing parents)"""
        loc = self.locations.get(path)
        if loc is None:
            parent = path.rsplit("/", 1)[0]
            if parent and parent not in self.locations:
                self.location(parent)
            loc = Location(path)
            self.locations[path] = loc
        if loc_type is not None:
            loc.type = loc_type
        return loc

    def get(self, path: str) -> Optional[Location]:
        return self.locations.get(path)

    def merge(self, other: "Scene"):
        """Merge another scene into this one, other's attributes win"""
        for path, loc in other.locations.items():
            target = self.location(path)
            if loc.type != "group":
                target.type = loc.type
            target.attributes.update(loc.attributes)

    def children(self, path: str) -> List[Location]:
        prefix = path + "/"
        return sorted(
            (loc for p, loc in self.locations.items()
             if p.startswith(prefix) and "/" not in p[len(prefix):]),
            key=lambda loc: loc.path)

    def of_type(self, loc_type: str) -> Iterator[Location]:
        for path in sorted(self.locations):
            if self.locations[path].type == loc_type:
                yield self.locations[path]

    def items(self) -> Iterator[Tuple[str, Location]]:
        return iter(sorted(self.locations.items()))

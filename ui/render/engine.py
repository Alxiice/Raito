"""Render engine interface

A render engine turns a cooked :class:`Scene` into an image. Engines run
asynchronously and report through Qt signals so the viewer stays
responsive. New engines (e.g. a rasterizer for previsualization) register
themselves in :data:`ENGINES` to appear in the viewer's engine selector.
"""

from typing import Dict, Type

from PySide6.QtCore import QObject, Signal
from PySide6.QtGui import QImage

from ..core import Scene


class RenderEngine(QObject):
    """Base class of render engines"""

    #: Name displayed in the viewer
    name = "Engine"

    started = Signal()
    finished = Signal(QImage)
    failed = Signal(str)
    log = Signal(str)

    def is_running(self) -> bool:
        raise NotImplementedError

    def render(self, scene: Scene):
        """Start rendering the scene, emit finished or failed when done"""
        raise NotImplementedError

    def cancel(self):
        raise NotImplementedError


ENGINES: Dict[str, Type[RenderEngine]] = {}


def register_engine(cls: Type[RenderEngine]) -> Type[RenderEngine]:
    ENGINES[cls.name] = cls
    return cls

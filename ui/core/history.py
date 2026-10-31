"""Undo / redo history made of graph snapshots"""

import time
from typing import Any, Hashable, List, Optional, Tuple


class History:
    """Stacks of (label, state) pairs

    Callers push the state *before* an edit. Edits pushed with the same
    merge key less than ``merge_delay`` seconds apart form a single step, so
    dragging a node or scrubbing a value is undone at once.
    """

    def __init__(self, limit: int = 200, merge_delay: float = 0.6):
        self.limit = limit
        self.merge_delay = merge_delay
        self._undo: List[Tuple[str, Any]] = []
        self._redo: List[Tuple[str, Any]] = []
        self._merge_key: Optional[Hashable] = None
        self._merge_time = 0.0

    def push(self, label: str, state: Any, merge_key: Optional[Hashable] = None,
             merge_delay: Optional[float] = None):
        now = time.monotonic()
        delay = self.merge_delay if merge_delay is None else merge_delay
        merging = (merge_key is not None and merge_key == self._merge_key
                   and now - self._merge_time < delay and self._undo)
        self._merge_key, self._merge_time = merge_key, now
        if merging:
            return
        self._undo.append((label, state))
        del self._undo[:-self.limit]
        self._redo.clear()

    def undo(self, current: Any) -> Optional[Any]:
        """Return the state to restore, or None if there is nothing to undo"""
        return self._step(self._undo, self._redo, current)

    def redo(self, current: Any) -> Optional[Any]:
        return self._step(self._redo, self._undo, current)

    def _step(self, source, target, current):
        if not source:
            return None
        label, state = source.pop()
        target.append((label, current))
        self._merge_key = None
        return state

    def clear(self):
        self._undo.clear()
        self._redo.clear()
        self._merge_key = None

    @property
    def can_undo(self) -> bool:
        return bool(self._undo)

    @property
    def can_redo(self) -> bool:
        return bool(self._redo)

    @property
    def undo_label(self) -> str:
        return self._undo[-1][0] if self._undo else ""

    @property
    def redo_label(self) -> str:
        return self._redo[-1][0] if self._redo else ""

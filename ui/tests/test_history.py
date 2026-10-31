"""Tests of the undo / redo history, run with: python -m unittest discover ui/tests"""

import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))

from ui.core import History  # noqa: E402


class HistoryTest(unittest.TestCase):
    def test_undo_redo(self):
        h = History()
        h.push("a", 0)
        h.push("b", 1)
        self.assertEqual(h.undo_label, "b")
        self.assertEqual(h.undo(2), 1)
        self.assertEqual(h.undo(1), 0)
        self.assertIsNone(h.undo(0))
        self.assertEqual(h.redo(0), 1)
        self.assertEqual(h.redo(1), 2)
        self.assertFalse(h.can_redo)

    def test_push_clears_redo(self):
        h = History()
        h.push("a", 0)
        h.undo(1)
        h.push("b", 0)
        self.assertFalse(h.can_redo)

    def test_merge_same_key(self):
        h = History(merge_delay=10)
        h.push("move", 0, "n1")
        h.push("move", 1, "n1")
        h.push("move", 2, "n2")
        self.assertEqual(h.undo(3), 2)
        self.assertEqual(h.undo(2), 0)
        self.assertFalse(h.can_undo)

    def test_no_merge_after_delay_or_undo(self):
        h = History(merge_delay=0)
        h.push("edit", 0, "k")
        h.push("edit", 1, "k")
        self.assertEqual(h.undo(2), 1)
        h = History(merge_delay=10)
        h.push("edit", 0, "k")
        h.undo(1)
        h.redo(0)
        h.push("edit", 1, "k")
        self.assertEqual(h.undo(2), 1)

    def test_limit(self):
        h = History(limit=2)
        for i in range(5):
            h.push("edit", i)
        self.assertEqual(h.undo(5), 4)
        self.assertEqual(h.undo(4), 3)
        self.assertFalse(h.can_undo)


class GraphControllerUndoTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        try:
            from PySide6.QtCore import QCoreApplication
        except ImportError:
            raise unittest.SkipTest("PySide6 is not installed")
        cls.app = QCoreApplication.instance() or QCoreApplication([])

    def setUp(self):
        from ui.backend.graph_controller import GraphController
        from ui.core import default_graph
        self.c = GraphController()
        self.c.set_graph(default_graph())
        self.sphere = next(n for n in self.c.graph.nodes.values() if n.name == "sphere_metal").node_id

    def test_set_graph_clears_history(self):
        self.c.setParam(self.sphere, "radius", 2.0)
        self.assertTrue(self.c.canUndo)
        from ui.core import Graph
        self.c.set_graph(Graph())
        self.assertFalse(self.c.canUndo)

    def test_param_and_delete(self):
        self.c.setParam(self.sphere, "radius", 2.0)
        self.c.deleteNode(self.sphere)
        self.assertNotIn(self.sphere, self.c.graph.nodes)
        self.c.undo()
        self.assertEqual(self.c.graph.nodes[self.sphere].params["radius"], 2.0)
        self.c.undo()
        self.assertEqual(self.c.graph.nodes[self.sphere].params["radius"], 1.0)
        self.c.redo()
        self.c.redo()
        self.assertNotIn(self.sphere, self.c.graph.nodes)

    def test_invalid_edit_is_not_recorded(self):
        self.c.setParam(self.sphere, "radius", "not a number")
        self.c.connectPorts(self.sphere, "out", self.sphere, "material")
        self.assertFalse(self.c.canUndo)

    def test_drag_is_one_step(self):
        self.c.beginInteraction()
        for x in range(10):
            self.c.moveNode(self.sphere, float(x), 0.0)
        self.c.endInteraction()
        self.c.moveNode(self.sphere, 50.0, 0.0)
        self.c.undo()
        self.assertEqual(self.c.graph.nodes[self.sphere].x, 9.0)
        self.c.undo()
        self.assertEqual(self.c.graph.nodes[self.sphere].x, 240.0)

    def test_undo_keeps_selection(self):
        self.c.select(self.sphere)
        self.c.setParam(self.sphere, "radius", 2.0)
        self.c.undo()
        self.assertEqual(self.c.selectedNodeId, self.sphere)


if __name__ == "__main__":
    unittest.main()

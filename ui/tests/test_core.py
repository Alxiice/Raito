"""Tests of the Qt independent core, run with: python -m unittest discover ui/tests"""

import os
import sys
import tempfile
import unittest
import xml.etree.ElementTree as ET

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))

from ui.core import Graph, GraphError, default_graph  # noqa: E402
from ui.render.raito import ExportError, scene_to_xml  # noqa: E402


class GraphTest(unittest.TestCase):
    def test_unique_names(self):
        g = Graph()
        self.assertEqual(g.add_node("sphere").name, "sphere1")
        self.assertEqual(g.add_node("sphere").name, "sphere2")
        self.assertEqual(g.add_node("sphere", name="my sphere!").name, "my_sphere")

    def test_connection_rules(self):
        g = Graph()
        sphere = g.add_node("sphere")
        material = g.add_node("lambert_material")
        merge = g.add_node("merge")
        with self.assertRaises(GraphError):  # material output into a scene input
            g.connect(material.node_id, "out", merge.node_id, "in1")
        g.connect(sphere.node_id, "out", merge.node_id, "in1")
        with self.assertRaises(GraphError):  # cycle
            other = g.add_node("merge")
            g.connect(merge.node_id, "out", other.node_id, "in1")
            g.connect(other.node_id, "out", merge.node_id, "in2")

    def test_input_connection_is_replaced(self):
        g = Graph()
        sphere = g.add_node("sphere")
        first, second = g.add_node("glass_material"), g.add_node("metal_material")
        g.connect(first.node_id, "out", sphere.node_id, "material")
        g.connect(second.node_id, "out", sphere.node_id, "material")
        self.assertEqual(len(g.connections), 1)
        self.assertEqual(g.connections[0].from_node, second.node_id)

    def test_param_validation(self):
        g = Graph()
        metal = g.add_node("metal_material")
        self.assertEqual(g.set_param(metal.node_id, "fuzz", 4.0), 1.0)  # clamped
        with self.assertRaises(ValueError):
            g.set_param(metal.node_id, "color", [1.0, 0.0])

    def test_cook_default_graph(self):
        g = default_graph()
        scene = g.cook(g.view_node)
        sphere = scene.get("/root/world/geo/sphere_glass")
        self.assertEqual(sphere.type, "sphere")
        self.assertEqual(sphere.get("material.assign"), "/root/materials/glass0")
        self.assertEqual(scene.get("/root").get("renderSettings.spp"), 16)
        self.assertEqual(len(list(scene.of_type("sphere"))), 4)

    def test_save_load_roundtrip(self):
        g = default_graph()
        with tempfile.TemporaryDirectory() as tmp:
            path = os.path.join(tmp, "scene.rt")
            g.save(path)
            loaded = Graph.load(path)
        self.assertEqual(loaded.to_dict(), g.to_dict())
        self.assertEqual(loaded.view_node, g.view_node)


class RaitoExportTest(unittest.TestCase):
    def test_export_default_graph(self):
        g = default_graph()
        root = ET.fromstring(scene_to_xml(g.cook(g.view_node)))
        self.assertEqual(len(root.findall("shader")), 4)
        self.assertEqual(len(root.findall("shape")), 4)
        self.assertIsNotNone(root.find("camera"))
        spp = root.find("render_settings/parameter[@name='spp']")
        self.assertEqual(spp.get("value"), "16")

    def test_unassigned_shape_uses_default_shader(self):
        g = Graph()
        sphere = g.add_node("sphere")
        camera = g.add_node("camera")
        merge = g.add_node("merge")
        g.connect(sphere.node_id, "out", merge.node_id, "in1")
        g.connect(camera.node_id, "out", merge.node_id, "in2")
        root = ET.fromstring(scene_to_xml(g.cook(merge.node_id)))
        self.assertEqual(root.find("shape/parameter[@name='shader']").get("id"), "")

    def test_export_requires_camera(self):
        g = Graph()
        sphere = g.add_node("sphere")
        with self.assertRaises(ExportError):
            scene_to_xml(g.cook(sphere.node_id))


if __name__ == "__main__":
    unittest.main()

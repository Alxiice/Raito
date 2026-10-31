"""Raito UI entry point: loads the QML interface"""

import argparse
import os
import sys

from PySide6.QtCore import QUrl
from PySide6.QtGui import QGuiApplication
from PySide6.QtQml import QQmlApplicationEngine, qmlRegisterSingletonInstance
from PySide6.QtQuickControls2 import QQuickStyle

from . import __version__
from .backend import AppController


QML_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "qml")


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(prog="raito-ui", description="Raito scene editor")
    parser.add_argument("scene", nargs="?", help="Scene file (.rt) to open")
    parser.add_argument("--reset-layout", action="store_true",
                        help="Ignore the saved panel layout")
    args, qt_args = parser.parse_known_args(argv if argv is not None else sys.argv[1:])

    QQuickStyle.setStyle("Fusion")
    app = QGuiApplication([sys.argv[0]] + qt_args)
    app.setOrganizationName("Raito")
    app.setApplicationName("Raito")
    app.setApplicationVersion(__version__)

    controller = AppController()
    if args.scene:
        controller.openScene(QUrl.fromLocalFile(os.path.abspath(args.scene)))
    qmlRegisterSingletonInstance(AppController, "Raito", 1, 0, "App", controller)

    engine = QQmlApplicationEngine()
    engine.addImageProvider("render", controller.render.provider)
    engine.setInitialProperties({"resetLayout": args.reset_layout})
    engine.load(QUrl.fromLocalFile(os.path.join(QML_DIR, "Main.qml")))
    if not engine.rootObjects():
        return 1
    return app.exec()

#!/usr/bin/env python3
"""
Setup script for Raito UI
"""

from setuptools import setup

setup(
    name="raito_ui",
    version="0.1.0",
    description="QML scene editor for the Raito render engine",
    package_dir={"raito_ui": "."},
    packages=["raito_ui", "raito_ui.core", "raito_ui.backend", "raito_ui.render"],
    package_data={"raito_ui": ["qml/*.qml", "qml/*/*.qml", "qml/*/qmldir", "icons/*"]},
    python_requires=">=3.9",
    install_requires=[
        "PySide6>=6.12.0",
    ],
    entry_points={
        "console_scripts": [
            "raito-ui=raito_ui.app:main",
        ],
    },
)

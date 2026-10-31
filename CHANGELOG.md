# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Additionnaly we ask developers to track down modifications to their related PR/Issue.

## [Unreleased]

### Added

- Python + QML scene editor (`ui/`) replacing the experimental PySide6 widgets UI:
  node graph, node editor, scene graph, attribute viewer and render viewer with an
  engine selector, in dockable panels that can float as separate windows
- Headless render mode: `raito --scene <scene.xml> --output <image.ppm>`
- `RtRenderResult::export_as_ppm` writes binary PPM images
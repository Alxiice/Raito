# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Additionnaly we ask developers to track down modifications to their related PR/Issue.

## [Unreleased]

## 0.0.2 - pre-release n.2 - November 2026

- Headless render mode: `raito --scene <scene.xml> --output <image.ppm>`
- `RtRenderResult::export_as_ppm` writes binary PPM images
- UI: Python + QML replacing `egui`:
  - node graph
  - node editor
  - scene graph
  - attribute viewer
  - render viewer with an engine selector
  - dockable panels that can float as separate windows
  - Scene editor: undo / redo (`Ctrl+Z` / `Ctrl+Y`) of node graph and node editor changes
  - Scene editor: scenes are saved as `.rt` files
- Raito logo (wordmark with dark / light text, app icon) as SVG, PNG and ICO in
  `docs/source/images/logo`
- Documentation updates

## 0.0.1 - pre-release n.1 - September 2024

First milestone:
- Simple demo UI in `egui`
- Working raytracing rendering with spheres geometry
- Several shaders
- XML scenes
"""Export the Raito logo SVGs to PNG variants and a Windows .ico

The SVGs in docs/source/images/logo are the source of truth, this script only
rasterizes them (desktop sizes only). Uses PySide6 from the UI requirements:

    python scripts/export_logo.py
"""

import os
import shutil
import struct
import sys

from PySide6.QtCore import QBuffer, QByteArray, QIODevice, QRectF, Qt
from PySide6.QtGui import QGuiApplication, QImage, QPainter
from PySide6.QtSvg import QSvgRenderer


ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LOGO_DIR = os.path.join(ROOT, "docs", "source", "images", "logo")
PNG_DIR = os.path.join(LOGO_DIR, "png")
UI_ICON_DIR = os.path.join(ROOT, "ui", "icons")

# Windows / desktop icon sizes
ICON_SIZES = [16, 24, 32, 48, 64, 128, 256, 512]
ICO_SIZES = [16, 24, 32, 48, 64, 128, 256]
# Wordmark heights, from docs header to high-res slides
LOGO_HEIGHTS = [64, 128, 256, 512]


def render(svg_path, width, height) -> QImage:
    renderer = QSvgRenderer(svg_path)
    image = QImage(width, height, QImage.Format_ARGB32_Premultiplied)
    image.fill(Qt.transparent)
    painter = QPainter(image)
    painter.setRenderHints(QPainter.Antialiasing | QPainter.SmoothPixmapTransform)
    renderer.render(painter, QRectF(0, 0, width, height))
    painter.end()
    return image


def png_bytes(image) -> bytes:
    data = QByteArray()
    buffer = QBuffer(data)
    buffer.open(QIODevice.WriteOnly)
    image.save(buffer, "PNG")
    return bytes(data)


def write_ico(path, images):
    """ICO file with PNG compressed entries (supported since Windows Vista)"""
    header = struct.pack("<HHH", 0, 1, len(images))
    offset = 6 + 16 * len(images)
    entries, blobs = b"", b""
    for size, image in images:
        blob = png_bytes(image)
        dim = 0 if size >= 256 else size
        entries += struct.pack("<BBBBHHII", dim, dim, 0, 0, 1, 32, len(blob), offset)
        blobs += blob
        offset += len(blob)
    with open(path, "wb") as f:
        f.write(header + entries + blobs)


def main() -> int:
    app = QGuiApplication(sys.argv)  # noqa: F841 (needed by QPainter)
    os.makedirs(PNG_DIR, exist_ok=True)
    os.makedirs(UI_ICON_DIR, exist_ok=True)

    icon_svg = os.path.join(LOGO_DIR, "raito-icon.svg")
    icons = {size: render(icon_svg, size, size) for size in ICON_SIZES}
    for size, image in icons.items():
        image.save(os.path.join(PNG_DIR, f"raito-icon-{size}.png"))

    for theme in ("light", "dark"):
        svg = os.path.join(LOGO_DIR, f"raito-logo-{theme}.svg")
        ratio = QSvgRenderer(svg).viewBoxF().width() / QSvgRenderer(svg).viewBoxF().height()
        for height in LOGO_HEIGHTS:
            image = render(svg, round(height * ratio), height)
            image.save(os.path.join(PNG_DIR, f"raito-logo-{theme}-{height}.png"))

    ico = os.path.join(LOGO_DIR, "raito.ico")
    write_ico(ico, [(size, icons[size]) for size in ICO_SIZES])

    # Window icon used by the scene editor
    shutil.copy(icon_svg, os.path.join(UI_ICON_DIR, "raito.svg"))
    shutil.copy(ico, os.path.join(UI_ICON_DIR, "raito.ico"))
    print(f"Exported logo to {LOGO_DIR} and {UI_ICON_DIR}")
    return 0


if __name__ == "__main__":
    sys.exit(main())

import QtQuick
import QtQuick.Shapes

// Bezier curve from an output port (x1, y1) to an input port (x2, y2)
Shape {
    id: wire

    property real x1
    property real y1
    property real x2
    property real y2
    property color color: "white"
    property real lineWidth: 2

    readonly property real tangent: Math.max(40, Math.abs(x2 - x1) / 2)

    preferredRendererType: Shape.CurveRenderer

    ShapePath {
        strokeColor: wire.color
        strokeWidth: wire.lineWidth
        fillColor: "transparent"
        startX: wire.x1
        startY: wire.y1

        PathCubic {
            x: wire.x2
            y: wire.y2
            control1X: wire.x1 + wire.tangent
            control1Y: wire.y1
            control2X: wire.x2 - wire.tangent
            control2Y: wire.y2
        }
    }
}

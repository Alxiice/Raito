import QtQuick
import QtQuick.Controls
import Raito
import "../controls"

// A node of the graph. Ports are laid out in rows: input i on the left and
// output i on the right of row i, so wires can compute port positions.
Rectangle {
    id: node

    required property string nodeId
    required property string name
    required property string typeLabel
    required property color nodeColor
    required property real posX
    required property real posY
    required property var inputs
    required property var outputs
    required property Item graphView

    readonly property bool selected: App.graph.selectedNodeId === nodeId
    readonly property bool viewed: App.graph.viewNodeId === nodeId

    x: posX
    y: posY
    width: graphView.nodeWidth
    height: graphView.headerHeight + Math.max(inputs.length, outputs.length, 1) * graphView.rowHeight + 6
    radius: 5
    color: Theme.nodeBody
    border.color: selected ? Theme.accent : Theme.border
    border.width: selected ? 2 : 1

    Component.onCompleted: graphView.registerNode(nodeId, node)
    Component.onDestruction: graphView.unregisterNode(nodeId)

    // Body: select, drag, view, context menu
    MouseArea {
        property point pressPos
        property point startPos

        anchors.fill: parent
        acceptedButtons: Qt.LeftButton | Qt.RightButton
        cursorShape: pressed ? Qt.ClosedHandCursor : Qt.ArrowCursor
        onPressed: (mouse) => {
            node.graphView.forceActiveFocus()
            App.graph.select(node.nodeId)
            if (mouse.button === Qt.RightButton) {
                node.graphView.openNodeMenu(node.nodeId)
                return
            }
            pressPos = mapToItem(node.graphView.canvas, mouse.x, mouse.y)
            startPos = Qt.point(node.posX, node.posY)
            App.graph.beginInteraction()
        }
        onReleased: App.graph.endInteraction()
        onCanceled: App.graph.endInteraction()
        onPositionChanged: (mouse) => {
            if (!(pressedButtons & Qt.LeftButton))
                return
            const p = mapToItem(node.graphView.canvas, mouse.x, mouse.y)
            App.graph.moveNode(node.nodeId, startPos.x + p.x - pressPos.x, startPos.y + p.y - pressPos.y)
        }
        onDoubleClicked: App.graph.setViewNode(node.nodeId)
    }

    // Header
    Rectangle {
        id: header
        anchors { left: parent.left; right: parent.right; top: parent.top; margins: node.border.width }
        height: node.graphView.headerHeight - node.border.width
        radius: node.radius - 1
        color: node.nodeColor

        // Square off the bottom corners
        Rectangle {
            anchors { left: parent.left; right: parent.right; bottom: parent.bottom }
            height: parent.radius
            color: parent.color
        }

        // View flag
        Rectangle {
            id: viewFlag
            anchors { left: parent.left; leftMargin: 5; verticalCenter: parent.verticalCenter }
            width: 12
            height: 12
            radius: 2
            color: node.viewed ? Theme.accent : Qt.alpha("#000000", 0.35)
            border.color: Qt.alpha("#000000", 0.5)

            MouseArea {
                anchors.fill: parent
                anchors.margins: -3
                cursorShape: Qt.PointingHandCursor
                onClicked: App.graph.setViewNode(node.viewed ? "" : node.nodeId)
                ToolTip.visible: containsMouse
                ToolTip.text: "View this node in the scene graph and the viewer (V)"
                ToolTip.delay: 600
                hoverEnabled: true
            }
        }

        Text {
            anchors { left: viewFlag.right; leftMargin: 6; right: typeText.left; rightMargin: 4; verticalCenter: parent.verticalCenter }
            text: node.name
            color: "white"
            font.pixelSize: Theme.fontSize
            font.bold: true
            elide: Text.ElideRight
        }

        Text {
            id: typeText
            anchors { right: parent.right; rightMargin: 6; verticalCenter: parent.verticalCenter }
            // The node name gets the room it needs first
            width: Math.max(0, Math.min(implicitWidth, parent.width - 34 - nameMetrics.advanceWidth))
            elide: Text.ElideRight
            text: node.typeLabel

            TextMetrics {
                id: nameMetrics
                text: node.name
                font.pixelSize: Theme.fontSize
                font.bold: true
            }
            color: Qt.alpha("white", 0.6)
            font.pixelSize: Theme.smallFontSize
        }
    }

    component Port: Item {
        id: port

        required property var modelData
        required property int index
        property bool isInput: true
        property Item nodeItem
        readonly property Item graph: nodeItem.graphView

        x: isInput ? 0 : port.nodeItem.width
        y: port.graph.headerHeight + index * port.graph.rowHeight + port.graph.rowHeight / 2

        Text {
            anchors.verticalCenter: parent.verticalCenter
            x: port.isInput ? 10 : -implicitWidth - 10
            text: port.modelData.name
            color: Theme.textDim
            font.pixelSize: Theme.smallFontSize
        }

        Rectangle {
            readonly property bool filled: !port.isInput || port.modelData.connected
            x: -width / 2
            y: -height / 2
            width: portMouse.containsMouse ? 13 : 11
            height: width
            radius: width / 2
            color: filled ? port.modelData.color : Theme.nodeBody
            border.color: port.modelData.color
            border.width: 2

            MouseArea {
                id: portMouse
                anchors.fill: parent
                anchors.margins: -5
                hoverEnabled: true
                preventStealing: true
                cursorShape: Qt.CrossCursor
                onPressed: (mouse) => {
                    port.graph.forceActiveFocus()
                    port.graph.beginWire(port.nodeItem.nodeId, port.modelData.name, port.isInput,
                                             port.modelData.type, mapToItem(port.graph.canvas, mouse.x, mouse.y))
                }
                onPositionChanged: (mouse) => {
                    if (pressed)
                        port.graph.updateWire(mapToItem(port.graph.canvas, mouse.x, mouse.y))
                }
                onReleased: (mouse) => port.graph.endWire(mapToItem(port.graph.canvas, mouse.x, mouse.y))
                onCanceled: port.graph.endWire(Qt.point(-1e9, -1e9))
            }
        }
    }

    Repeater {
        model: node.inputs
        delegate: Port { isInput: true; nodeItem: node }
    }

    Repeater {
        model: node.outputs
        delegate: Port { isInput: false; nodeItem: node }
    }
}

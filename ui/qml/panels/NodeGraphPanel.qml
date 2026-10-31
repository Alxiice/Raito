import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Raito
import "../controls"

// Node graph editor
//   - Right click / Tab: create a node     - Drag from a port: connect
//   - Drag from a connected input: move or remove the wire
//   - Middle drag or Alt + drag: pan       - Wheel: zoom
//   - F: frame all   V / double click: view node   Delete: delete node
Item {
    id: graphView

    readonly property real nodeWidth: 170
    readonly property real headerHeight: 24
    readonly property real rowHeight: 20
    property alias canvas: canvas

    property real zoom: 1.0
    property real panX: 40
    property real panY: 40

    // nodeId -> NodeItem, wires look up their end nodes here
    property var nodeItems: ({})
    property int nodeItemsRevision: 0

    // Wire being dragged: { node, port, fromOutput, type, color, anchor }
    property var pendingWire: null
    property point pendingEnd
    property point createPos

    clip: true
    focus: true

    function registerNode(id, item) {
        nodeItems[id] = item
        nodeItemsRevision++
    }

    function unregisterNode(id) {
        if (nodeItems[id]) {
            delete nodeItems[id]
            nodeItemsRevision++
        }
    }

    function portPosition(item, index, isInput) {
        return Qt.point(item.x + (isInput ? 0 : item.width),
                        item.y + headerHeight + index * rowHeight + rowHeight / 2)
    }

    function toCanvas(x, y) {
        return Qt.point((x - panX) / zoom, (y - panY) / zoom)
    }

    function zoomAt(x, y, factor) {
        const p = toCanvas(x, y)
        zoom = Math.min(3.0, Math.max(0.15, zoom * factor))
        panX = x - p.x * zoom
        panY = y - p.y * zoom
    }

    function frameAll() {
        const ids = Object.keys(nodeItems)
        if (ids.length === 0 || width <= 0 || height <= 0)
            return
        let x0 = Infinity, y0 = Infinity, x1 = -Infinity, y1 = -Infinity
        for (const id of ids) {
            const n = nodeItems[id]
            x0 = Math.min(x0, n.x); y0 = Math.min(y0, n.y)
            x1 = Math.max(x1, n.x + n.width); y1 = Math.max(y1, n.y + n.height)
        }
        const margin = 50
        zoom = Math.min(1.2, Math.max(0.15, Math.min(width / (x1 - x0 + 2 * margin),
                                                    height / (y1 - y0 + 2 * margin))))
        panX = width / 2 - (x0 + x1) / 2 * zoom
        panY = height / 2 - (y0 + y1) / 2 * zoom
    }

    // === Wires ===

    function beginWire(nodeId, port, isInput, type, pos) {
        // Picking up a wire and dropping it elsewhere is undone as one step
        App.graph.beginInteraction()
        const item = nodeItems[nodeId]
        const ports = isInput ? item.inputs : item.outputs
        const index = ports.findIndex(p => p.name === port)
        if (isInput && ports[index].connected) {
            // Pick up the existing wire from its source
            const source = App.graph.disconnectInput(nodeId, port)
            const src = nodeItems[source.fromNode]
            const srcIndex = src.outputs.findIndex(p => p.name === source.fromPort)
            pendingWire = { node: source.fromNode, port: source.fromPort, fromOutput: true, type: type,
                            color: ports[index].color, anchor: portPosition(src, srcIndex, false) }
        } else {
            pendingWire = { node: nodeId, port: port, fromOutput: !isInput, type: type,
                            color: ports[index].color, anchor: portPosition(item, index, isInput) }
        }
        pendingEnd = pos
    }

    function updateWire(pos) {
        if (pendingWire)
            pendingEnd = pos
    }

    function endWire(pos) {
        const wire = pendingWire
        pendingWire = null
        const target = wire ? findPort(pos, wire) : null
        if (target && wire.fromOutput)
            App.graph.connectPorts(wire.node, wire.port, target.node, target.port)
        else if (target)
            App.graph.connectPorts(target.node, target.port, wire.node, wire.port)
        App.graph.endInteraction()
    }

    // Port under pos accepting the wire, or the best port of the node under pos
    function findPort(pos, wire) {
        const wantInput = wire.fromOutput
        for (const id in nodeItems) {
            const item = nodeItems[id]
            const ports = wantInput ? item.inputs : item.outputs
            for (let i = 0; i < ports.length; i++) {
                const p = portPosition(item, i, wantInput)
                if (Math.hypot(pos.x - p.x, pos.y - p.y) <= 12)
                    return { node: id, port: ports[i].name }
            }
        }
        for (const id in nodeItems) {
            const item = nodeItems[id]
            if (id === wire.node || pos.x < item.x || pos.x > item.x + item.width
                    || pos.y < item.y || pos.y > item.y + item.height)
                continue
            const ports = (wantInput ? item.inputs : item.outputs).filter(p => p.type === wire.type)
            const port = ports.find(p => !p.connected) || ports[0]
            return port ? { node: id, port: port.name } : null
        }
        return null
    }

    // === Menus ===

    function openNodeMenu(nodeId) {
        nodeMenu.nodeId = nodeId
        nodeMenu.popup()
    }

    function openCreateMenu(viewPos) {
        createPos = toCanvas(viewPos.x, viewPos.y)
        createMenu.popup()
    }

    function createNode(typeId) {
        App.graph.createNode(typeId, createPos.x - nodeWidth / 2, createPos.y - headerHeight / 2)
    }

    Connections {
        target: App.graph
        function onGraphReplaced() { Qt.callLater(graphView.frameAll) }
    }

    onWidthChanged: if (width > 0 && height > 0 && !framed) { framed = true; Qt.callLater(frameAll) }
    property bool framed: false

    Keys.onPressed: (event) => {
        const selected = App.graph.selectedNodeId
        if ((event.key === Qt.Key_Delete || event.key === Qt.Key_Backspace) && selected) {
            App.graph.deleteNode(selected)
        } else if (event.key === Qt.Key_F) {
            frameAll()
        } else if (event.key === Qt.Key_V && selected) {
            App.graph.setViewNode(selected)
        } else if (event.key === Qt.Key_Tab) {
            createPos = toCanvas(backgroundMouse.mouseX, backgroundMouse.mouseY)
            searchPopup.x = Math.min(backgroundMouse.mouseX, width - searchPopup.width)
            searchPopup.y = Math.min(backgroundMouse.mouseY, height - searchPopup.height)
            searchPopup.open()
        } else {
            return
        }
        event.accepted = true
    }

    // === Background ===

    Rectangle {
        anchors.fill: parent
        color: Theme.graphBackground
    }

    Canvas {
        id: grid
        anchors.fill: parent
        onPaint: {
            const ctx = getContext("2d")
            ctx.reset()
            const step = 20 * graphView.zoom
            if (step < 6)
                return
            for (const major of [false, true]) {
                const s = major ? step * 5 : step
                ctx.strokeStyle = major ? Theme.graphGridMajor : Theme.graphGrid
                ctx.lineWidth = 1
                ctx.beginPath()
                for (let x = graphView.panX % s; x < width; x += s) {
                    ctx.moveTo(Math.round(x) + 0.5, 0); ctx.lineTo(Math.round(x) + 0.5, height)
                }
                for (let y = graphView.panY % s; y < height; y += s) {
                    ctx.moveTo(0, Math.round(y) + 0.5); ctx.lineTo(width, Math.round(y) + 0.5)
                }
                ctx.stroke()
            }
        }
        Connections {
            target: graphView
            function onZoomChanged() { grid.requestPaint() }
            function onPanXChanged() { grid.requestPaint() }
            function onPanYChanged() { grid.requestPaint() }
        }
    }

    MouseArea {
        id: backgroundMouse

        property point last
        property bool panning: false

        anchors.fill: parent
        hoverEnabled: true
        acceptedButtons: Qt.LeftButton | Qt.MiddleButton | Qt.RightButton
        cursorShape: panning ? Qt.ClosedHandCursor : Qt.ArrowCursor
        onPressed: (mouse) => {
            graphView.forceActiveFocus()
            last = Qt.point(mouse.x, mouse.y)
            if (mouse.button === Qt.MiddleButton
                    || (mouse.button === Qt.LeftButton && (mouse.modifiers & Qt.AltModifier))) {
                panning = true
            } else if (mouse.button === Qt.LeftButton) {
                App.graph.select("")
            } else if (mouse.button === Qt.RightButton) {
                graphView.openCreateMenu(Qt.point(mouse.x, mouse.y))
            }
        }
        onPositionChanged: (mouse) => {
            if (!panning)
                return
            graphView.panX += mouse.x - last.x
            graphView.panY += mouse.y - last.y
            last = Qt.point(mouse.x, mouse.y)
        }
        onReleased: panning = false
        onWheel: (wheel) => graphView.zoomAt(wheel.x, wheel.y, Math.pow(1.0015, wheel.angleDelta.y))
    }

    // === Graph content, in canvas coordinates ===

    Item {
        id: canvas
        x: graphView.panX
        y: graphView.panY
        scale: graphView.zoom
        transformOrigin: Item.TopLeft

        Repeater {
            model: App.graph.connections

            delegate: Wire {
                required property string fromNode
                required property int fromIndex
                required property string toNode
                required property int toIndex
                required property color wireColor

                readonly property Item fromItem: graphView.nodeItemsRevision, graphView.nodeItems[fromNode] || null
                readonly property Item toItem: graphView.nodeItemsRevision, graphView.nodeItems[toNode] || null

                visible: fromItem !== null && toItem !== null
                color: wireColor
                x1: fromItem ? fromItem.x + fromItem.width : 0
                y1: fromItem ? fromItem.y + graphView.headerHeight + (fromIndex + 0.5) * graphView.rowHeight : 0
                x2: toItem ? toItem.x : 0
                y2: toItem ? toItem.y + graphView.headerHeight + (toIndex + 0.5) * graphView.rowHeight : 0
            }
        }

        Wire {
            readonly property var w: graphView.pendingWire
            visible: w !== null
            color: w ? w.color : "white"
            lineWidth: 2
            opacity: 0.8
            x1: w ? (w.fromOutput ? w.anchor.x : graphView.pendingEnd.x) : 0
            y1: w ? (w.fromOutput ? w.anchor.y : graphView.pendingEnd.y) : 0
            x2: w ? (w.fromOutput ? graphView.pendingEnd.x : w.anchor.x) : 0
            y2: w ? (w.fromOutput ? graphView.pendingEnd.y : w.anchor.y) : 0
        }

        Repeater {
            model: App.graph.nodes
            // canvas.parent is graphView, shadowed here by NodeItem.graphView
            delegate: NodeItem { graphView: canvas.parent }
        }
    }

    // === Overlay ===

    Text {
        anchors { left: parent.left; bottom: parent.bottom; margins: 8 }
        text: "Right click or Tab: add node   •   F: frame   •   V: view node   •   Del: delete"
        color: Theme.textDim
        font.pixelSize: Theme.smallFontSize
        opacity: 0.7
    }

    Text {
        anchors.centerIn: parent
        visible: Object.keys(graphView.nodeItems).length === 0 && graphView.nodeItemsRevision >= 0
        text: "Empty graph\nRight click or press Tab to create a node"
        horizontalAlignment: Text.AlignHCenter
        color: Theme.textDim
        font.pixelSize: Theme.fontSize + 2
    }

    Menu {
        id: createMenu

        Instantiator {
            model: App.graph.nodeCategories

            delegate: Menu {
                id: categoryMenu

                required property var modelData

                title: modelData.name

                Instantiator {
                    model: categoryMenu.modelData.types
                    delegate: MenuItem {
                        required property var modelData
                        text: modelData.label
                        onTriggered: graphView.createNode(modelData.typeId)
                    }
                    onObjectAdded: (index, object) => categoryMenu.insertItem(index, object)
                    onObjectRemoved: (index, object) => categoryMenu.removeItem(object)
                }
            }
            onObjectAdded: (index, object) => createMenu.insertMenu(index, object)
            onObjectRemoved: (index, object) => createMenu.removeMenu(object)
        }
    }

    Menu {
        id: nodeMenu

        property string nodeId

        MenuItem {
            text: "View node"
            onTriggered: App.graph.setViewNode(nodeMenu.nodeId)
        }
        MenuSeparator {}
        MenuItem {
            text: "Delete"
            onTriggered: App.graph.deleteNode(nodeMenu.nodeId)
        }
    }

    // Tab node search
    Popup {
        id: searchPopup

        readonly property var allTypes: {
            const types = []
            for (const category of App.graph.nodeCategories)
                for (const t of category.types)
                    types.push(t)
            return types
        }
        readonly property var matches: allTypes.filter(
            t => (t.label + " " + t.category).toLowerCase().includes(searchField.text.toLowerCase()))

        width: 220
        height: 260
        padding: 4
        focus: true
        onOpened: {
            searchField.text = ""
            searchField.forceActiveFocus()
            resultList.currentIndex = 0
        }
        onClosed: graphView.forceActiveFocus()

        function accept() {
            if (resultList.currentIndex >= 0 && resultList.currentIndex < matches.length)
                graphView.createNode(matches[resultList.currentIndex].typeId)
            close()
        }

        ColumnLayout {
            anchors.fill: parent
            spacing: 4

            TextField {
                id: searchField
                Layout.fillWidth: true
                placeholderText: "Search nodes..."
                onTextChanged: resultList.currentIndex = 0
                Keys.onDownPressed: resultList.incrementCurrentIndex()
                Keys.onUpPressed: resultList.decrementCurrentIndex()
                onAccepted: searchPopup.accept()
            }

            ListView {
                id: resultList
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                model: searchPopup.matches
                highlightMoveDuration: 0

                delegate: ItemDelegate {
                    required property var modelData
                    required property int index
                    width: ListView.view.width
                    height: 24
                    highlighted: ListView.isCurrentItem
                    onClicked: { resultList.currentIndex = index; searchPopup.accept() }
                    contentItem: RowLayout {
                        Rectangle { width: 8; height: 8; radius: 2; color: modelData.color }
                        Text { text: modelData.label; color: Theme.text; Layout.fillWidth: true; font.pixelSize: Theme.fontSize }
                        Text { text: modelData.category; color: Theme.textDim; font.pixelSize: Theme.smallFontSize }
                    }
                }
            }
        }
    }
}

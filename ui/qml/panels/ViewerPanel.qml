import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Raito
import "../controls"

// Render viewer: renders the viewed node with the selected engine
Rectangle {
    id: root

    readonly property var renderer: App.render
    property real zoom: 1.0
    property real panX: 0
    property real panY: 0

    function fit() {
        const s = renderer.imageSize
        if (s.width <= 0 || view.width <= 0)
            return
        zoom = Math.min(view.width / s.width, view.height / s.height) * 0.95
        center()
    }

    function center() {
        const s = renderer.imageSize
        panX = (view.width - s.width * zoom) / 2
        panY = (view.height - s.height * zoom) / 2
    }

    function setZoom(z, x, y) {
        const ix = (x - panX) / zoom, iy = (y - panY) / zoom
        zoom = Math.min(16, Math.max(0.05, z))
        panX = x - ix * zoom
        panY = y - iy * zoom
    }

    color: Theme.panel

    Connections {
        target: root.renderer
        property bool first: true
        function onImageChanged() {
            if (first) {
                first = false
                root.fit()
            }
        }
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        // Toolbar
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 34
            color: Theme.panelAlt

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 6
                anchors.rightMargin: 6
                spacing: 6

                Label { text: "Engine"; color: Theme.textDim }

                ComboBox {
                    id: engineBox
                    Layout.preferredWidth: 120
                    model: root.renderer.engines
                    currentIndex: root.renderer.engines.indexOf(root.renderer.currentEngine)
                    onActivated: root.renderer.currentEngine = currentText
                    ToolTip.visible: hovered
                    ToolTip.text: "Render engine used by the viewer"
                    ToolTip.delay: 600
                }

                Button {
                    text: root.renderer.running ? "Stop" : "Render"
                    highlighted: !root.renderer.running
                    onClicked: root.renderer.running ? root.renderer.cancel() : root.renderer.render()
                    ToolTip.visible: hovered
                    ToolTip.text: root.renderer.running ? "Cancel the render" : "Render the viewed node (F5)"
                    ToolTip.delay: 600
                }

                CheckBox {
                    text: "Auto"
                    checked: root.renderer.autoRender
                    onToggled: root.renderer.autoRender = checked
                    ToolTip.visible: hovered
                    ToolTip.text: "Render again whenever the scene changes"
                    ToolTip.delay: 600
                }

                Item { Layout.fillWidth: true }

                Label {
                    text: Math.round(root.zoom * 100) + "%"
                    color: Theme.textDim
                }
                ToolIconButton { text: "⤢"; tip: "Fit (F)"; onClicked: root.fit() }
                ToolIconButton { text: "1:1"; font.pixelSize: 11; tip: "Actual size (1)"; onClicked: { root.zoom = 1; root.center() } }
            }
        }

        // Image view
        Item {
            id: view

            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            focus: true

            Keys.onPressed: (event) => {
                if (event.key === Qt.Key_F) { root.fit(); event.accepted = true }
                else if (event.key === Qt.Key_1) { root.zoom = 1; root.center(); event.accepted = true }
            }

            Rectangle {
                anchors.fill: parent
                color: "#141416"
            }

            Rectangle {
                x: image.x - 1
                y: image.y - 1
                width: image.width + 2
                height: image.height + 2
                visible: image.status === Image.Ready
                color: "transparent"
                border.color: Theme.border
            }

            Image {
                id: image
                x: root.panX
                y: root.panY
                width: sourceSize.width * root.zoom
                height: sourceSize.height * root.zoom
                source: root.renderer.imageRevision > 0 ? "image://render/" + root.renderer.imageRevision : ""
                cache: false
                smooth: root.zoom < 1
                mipmap: root.zoom < 1
            }

            Text {
                anchors.centerIn: parent
                visible: image.status !== Image.Ready && !root.renderer.running
                text: "No render yet\nPress Render (F5) to render the viewed node"
                horizontalAlignment: Text.AlignHCenter
                color: Theme.textDim
                font.pixelSize: Theme.fontSize + 2
            }

            BusyIndicator {
                anchors { right: parent.right; top: parent.top; margins: 8 }
                width: 32
                height: 32
                running: root.renderer.running
            }

            MouseArea {
                property point last

                anchors.fill: parent
                acceptedButtons: Qt.LeftButton | Qt.MiddleButton
                cursorShape: pressed ? Qt.ClosedHandCursor : Qt.OpenHandCursor
                onPressed: (mouse) => { view.forceActiveFocus(); last = Qt.point(mouse.x, mouse.y) }
                onPositionChanged: (mouse) => {
                    root.panX += mouse.x - last.x
                    root.panY += mouse.y - last.y
                    last = Qt.point(mouse.x, mouse.y)
                }
                onDoubleClicked: root.fit()
                onWheel: (wheel) => root.setZoom(root.zoom * Math.pow(1.0015, wheel.angleDelta.y), wheel.x, wheel.y)
            }
        }

        // Status and log
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 24
            color: Theme.panelAlt

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 8
                anchors.rightMargin: 4

                Label {
                    Layout.fillWidth: true
                    text: root.renderer.status
                    color: root.renderer.status.startsWith("Raito failed") || root.renderer.status.includes("not found")
                           ? Theme.error : Theme.textDim
                    font.pixelSize: Theme.smallFontSize + 1
                    elide: Text.ElideRight
                }
                ToolIconButton {
                    text: logView.visible ? "▾" : "▴"
                    tip: logView.visible ? "Hide render log" : "Show render log"
                    onClicked: logView.visible = !logView.visible
                }
            }
        }

        ScrollView {
            id: logView
            Layout.fillWidth: true
            Layout.preferredHeight: 120
            visible: false

            TextArea {
                readOnly: true
                text: root.renderer.logText
                font.family: Theme.monoFont
                font.pixelSize: Theme.smallFontSize + 1
                wrapMode: TextEdit.NoWrap
                selectByMouse: true
                background: Rectangle { color: Theme.field }
                onTextChanged: cursorPosition = length
            }
        }
    }
}

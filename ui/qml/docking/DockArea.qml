import QtQuick
import QtQuick.Controls
import "../controls"

// Tabbed container showing the panels docked in one area of the DockManager
Rectangle {
    id: area

    required property string areaName
    required property Item manager

    readonly property var panels: {
        manager.revision
        return manager.panelsIn(areaName)
    }
    readonly property var currentPanel: {
        let best = null
        for (const p of panels)
            if (best === null || p.activation > best.activation)
                best = p
        return best
    }
    readonly property bool highlighted: manager.dragPanel !== null && manager.hoverZone === areaName

    // Use this rather than visible, which is false while the parent is hidden
    readonly property bool hasPanels: panels.length > 0

    objectName: "area_" + areaName
    visible: hasPanels
    color: Theme.panel
    border.color: Theme.border

    onPanelsChanged: layoutPanels()
    onCurrentPanelChanged: layoutPanels()

    function layoutPanels() {
        for (const p of panels) {
            p.parent = container
            p.anchors.fill = container
            p.visible = p === currentPanel
        }
    }

    Rectangle {
        id: header
        anchors { left: parent.left; right: parent.right; top: parent.top }
        height: 26
        color: Theme.header

        Row {
            id: tabs
            anchors { left: parent.left; top: parent.top; bottom: parent.bottom; right: buttons.left }
            clip: true

            Repeater {
                model: area.panels

                delegate: Rectangle {
                    id: tab

                    required property var modelData
                    readonly property bool current: modelData === area.currentPanel

                    width: label.implicitWidth + 24
                    height: header.height
                    color: current ? Theme.panel : (tabMouse.containsMouse ? Theme.headerActive : "transparent")

                    Rectangle {
                        visible: tab.current
                        anchors { left: parent.left; right: parent.right; top: parent.top }
                        height: 2
                        color: Theme.accent
                    }

                    Text {
                        id: label
                        anchors.centerIn: parent
                        text: tab.modelData.title
                        color: tab.current ? Theme.text : Theme.textDim
                        font.pixelSize: Theme.fontSize
                    }

                    MouseArea {
                        id: tabMouse

                        property point pressPos
                        property bool dragging: false

                        anchors.fill: parent
                        hoverEnabled: true
                        acceptedButtons: Qt.LeftButton | Qt.MiddleButton
                        onPressed: (mouse) => {
                            pressPos = Qt.point(mouse.x, mouse.y)
                            area.manager.activate(tab.modelData)
                        }
                        onPositionChanged: (mouse) => {
                            if (!pressed)
                                return
                            if (!dragging && Math.hypot(mouse.x - pressPos.x, mouse.y - pressPos.y) > 10) {
                                dragging = true
                                area.manager.beginDrag(tab.modelData)
                            }
                            if (dragging)
                                area.manager.updateDrag(mapToItem(area.manager, mouse.x, mouse.y))
                        }
                        onReleased: (mouse) => {
                            if (!dragging)
                                return
                            dragging = false
                            area.manager.endDrag(mapToItem(area.manager, mouse.x, mouse.y),
                                                 mapToGlobal(mouse.x, mouse.y))
                        }
                        onClicked: (mouse) => {
                            if (mouse.button === Qt.MiddleButton)
                                area.manager.closePanel(tab.modelData)
                        }
                        onDoubleClicked: area.manager.floatPanel(tab.modelData)
                    }
                }
            }
        }

        Row {
            id: buttons
            anchors { right: parent.right; verticalCenter: parent.verticalCenter; rightMargin: 2 }

            ToolIconButton {
                text: "⧉"
                tip: "Float panel in its own window"
                onClicked: area.manager.floatPanel(area.currentPanel)
            }
            ToolIconButton {
                text: "✕"
                tip: "Close panel (reopen it from the View menu)"
                onClicked: area.manager.closePanel(area.currentPanel)
            }
        }
    }

    Item {
        id: container
        anchors { left: parent.left; right: parent.right; top: header.bottom; bottom: parent.bottom; margins: 1 }
    }

    Rectangle {
        anchors.fill: parent
        visible: area.highlighted
        color: Qt.alpha(Theme.accent, 0.18)
        border.color: Theme.accent
        border.width: 2
    }
}

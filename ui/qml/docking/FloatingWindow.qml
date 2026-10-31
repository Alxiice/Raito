import QtQuick
import QtQuick.Controls
import "../controls"

// Independent top level window hosting a floating DockPanel
Window {
    id: win

    required property var panel
    required property Item manager

    title: panel.title + " - Raito"
    color: Theme.panel
    flags: Qt.Window
    // Not transient: the window lives independently from the main window
    transientParent: null
    minimumWidth: 240
    minimumHeight: 160
    visible: true

    // Closing a floating window docks its panel back
    onClosing: manager.dockPanel(panel)

    Component.onCompleted: {
        Theme.applyPalette(palette)
        panel.parent = container
        panel.anchors.fill = container
        panel.visible = true
    }

    Rectangle {
        id: header
        anchors { left: parent.left; right: parent.right; top: parent.top }
        height: 26
        color: Theme.header

        Text {
            anchors { left: parent.left; leftMargin: 10; verticalCenter: parent.verticalCenter }
            text: win.panel.title
            color: Theme.text
            font.pixelSize: Theme.fontSize
        }

        Row {
            anchors { right: parent.right; verticalCenter: parent.verticalCenter; rightMargin: 2 }

            ToolIconButton {
                text: "↵"
                tip: "Dock back into the main window"
                onClicked: win.manager.dockPanel(win.panel)
            }
            ToolIconButton {
                text: "✕"
                tip: "Close panel (reopen it from the View menu)"
                onClicked: win.manager.closePanel(win.panel)
            }
        }
    }

    Item {
        id: container
        anchors { left: parent.left; right: parent.right; top: header.bottom; bottom: parent.bottom }
    }
}

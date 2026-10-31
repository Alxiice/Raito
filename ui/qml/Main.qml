import QtQuick
import QtQuick.Controls
import QtQuick.Dialogs
import QtQuick.Layouts
import QtCore
import Raito
import "controls"
import "docking"
import "panels"

ApplicationWindow {
    id: window

    property bool resetLayout: false

    width: 1500
    height: 920
    visible: true
    title: App.currentFileName + " - Raito"
    color: Theme.window

    Component.onCompleted: Theme.applyPalette(palette)

    onClosing: {
        dock.saveLayout()
        dock.closeAllWindows()
        Qt.quit()
    }

    Settings {
        category: "mainWindow"
        property alias x: window.x
        property alias y: window.y
        property alias width: window.width
        property alias height: window.height
    }

    // === Actions ===

    Action { id: newAction; text: "&New Scene"; shortcut: StandardKey.New; onTriggered: App.newScene() }
    Action { id: openAction; text: "&Open..."; shortcut: StandardKey.Open; onTriggered: openDialog.open() }
    Action {
        id: saveAction
        text: "&Save"
        shortcut: StandardKey.Save
        onTriggered: App.currentFile ? App.saveScene("") : saveDialog.open()
    }
    Action { id: saveAsAction; text: "Save &As..."; shortcut: "Ctrl+Shift+S"; onTriggered: saveDialog.open() }
    Action {
        id: undoAction
        text: App.graph.undoText ? "&Undo " + App.graph.undoText : "&Undo"
        shortcut: StandardKey.Undo
        enabled: App.graph.canUndo
        onTriggered: App.graph.undo()
    }
    Action {
        id: redoAction
        text: App.graph.redoText ? "&Redo " + App.graph.redoText : "&Redo"
        shortcut: StandardKey.Redo
        enabled: App.graph.canRedo
        onTriggered: App.graph.redo()
    }
    Action { id: renderAction; text: "&Render"; shortcut: "F5"; onTriggered: App.render.render() }
    Action {
        id: cancelAction
        text: "&Cancel Render"
        shortcut: "Esc"
        enabled: App.render.running
        onTriggered: App.render.cancel()
    }

    menuBar: MenuBar {
        Menu {
            title: "&File"
            MenuItem { action: newAction }
            MenuItem { text: "New &Empty Scene"; onTriggered: App.clearScene() }
            MenuItem { action: openAction }
            MenuSeparator {}
            MenuItem { action: saveAction }
            MenuItem { action: saveAsAction }
            MenuSeparator {}
            MenuItem { text: "&Quit"; onTriggered: window.close() }
        }
        Menu {
            title: "&Edit"
            MenuItem { action: undoAction }
            MenuItem { action: redoAction }
        }
        Menu {
            id: viewMenu
            title: "&View"

            Instantiator {
                model: dock.panels
                delegate: MenuItem {
                    required property var modelData
                    text: modelData.title + (modelData.floating && modelData.open ? "  (floating)" : "")
                    checkable: true
                    checked: modelData.open
                    onTriggered: modelData.open ? dock.closePanel(modelData) : dock.showPanel(modelData)
                }
                onObjectAdded: (index, object) => viewMenu.insertItem(index, object)
                onObjectRemoved: (index, object) => viewMenu.removeItem(object)
            }
            MenuSeparator {}
            MenuItem { text: "&Reset Layout"; onTriggered: dock.resetLayout() }
        }
        Menu {
            title: "&Render"
            MenuItem { action: renderAction }
            MenuItem { action: cancelAction }
            MenuItem {
                text: "&Auto Render"
                checkable: true
                checked: App.render.autoRender
                onTriggered: App.render.autoRender = checked
            }
            MenuSeparator {}
            Menu {
                id: engineMenu
                title: "&Engine"
                Instantiator {
                    model: App.render.engines
                    delegate: MenuItem {
                        required property string modelData
                        text: modelData
                        checkable: true
                        checked: App.render.currentEngine === modelData
                        onTriggered: App.render.currentEngine = modelData
                    }
                    onObjectAdded: (index, object) => engineMenu.insertItem(index, object)
                    onObjectRemoved: (index, object) => engineMenu.removeItem(object)
                }
            }
        }
    }

    // === Panels ===

    DockManager {
        id: dock
        objectName: "dockManager"
        anchors.fill: parent
        restoreSavedLayout: !window.resetLayout

        DockPanel {
            panelId: "viewer"
            title: "Viewer"
            defaultArea: "center"
            ViewerPanel { anchors.fill: parent }
        }
        DockPanel {
            panelId: "nodeGraph"
            title: "Node Graph"
            defaultArea: "bottom"
            NodeGraphPanel { anchors.fill: parent }
        }
        DockPanel {
            panelId: "nodeEditor"
            title: "Node Editor"
            defaultArea: "rightTop"
            NodeEditorPanel { anchors.fill: parent }
        }
        DockPanel {
            panelId: "sceneGraph"
            title: "Scene Graph"
            defaultArea: "leftTop"
            SceneGraphPanel { anchors.fill: parent }
        }
        DockPanel {
            panelId: "attributes"
            title: "Attributes"
            defaultArea: "leftBottom"
            AttributesPanel { anchors.fill: parent }
        }
    }

    footer: Rectangle {
        height: 24
        color: Theme.header

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 8
            anchors.rightMargin: 8

            Label {
                id: messageLabel
                property bool isError: false
                Layout.fillWidth: true
                color: isError ? Theme.error : Theme.textDim
                font.pixelSize: Theme.smallFontSize + 1
                elide: Text.ElideRight
            }
            Label {
                text: App.render.status
                color: Theme.textDim
                font.pixelSize: Theme.smallFontSize + 1
            }
        }

        Timer {
            id: messageTimer
            interval: 6000
            onTriggered: messageLabel.text = ""
        }

        Connections {
            target: App
            function onMessage(text, isError) {
                messageLabel.text = text
                messageLabel.isError = isError
                messageTimer.restart()
            }
        }
    }

    // === Dialogs ===

    FileDialog {
        id: openDialog
        title: "Open scene"
        nameFilters: ["Raito scenes (*.json)", "All files (*)"]
        onAccepted: App.openScene(selectedFile)
    }

    FileDialog {
        id: saveDialog
        title: "Save scene"
        fileMode: FileDialog.SaveFile
        defaultSuffix: "json"
        nameFilters: ["Raito scenes (*.json)"]
        onAccepted: App.saveScene(selectedFile)
    }
}

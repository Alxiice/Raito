import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Raito
import "../controls"

// Scene graph locations produced by the viewed node
Rectangle {
    id: root

    readonly property var typeColors: ({
        group: "#8a8c92", sphere: "#5cb879", material: "#b98ae0", camera: "#5fb0d6"
    })

    color: Theme.panel

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 28
            color: Theme.panelAlt

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 8
                anchors.rightMargin: 4

                Label {
                    Layout.fillWidth: true
                    text: App.graph.viewNodeId === "" ? "No node viewed"
                                                     : App.sceneGraph.locationCount + " locations"
                    color: Theme.textDim
                    font.pixelSize: Theme.smallFontSize
                }
                ToolIconButton {
                    text: "⊞"
                    tip: "Expand all"
                    onClicked: tree.expandRecursively()
                }
                ToolIconButton {
                    text: "⊟"
                    tip: "Collapse all"
                    onClicked: { tree.collapseRecursively(); tree.expand(0) }
                }
            }
        }

        TreeView {
            id: tree

            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            model: App.sceneGraph
            boundsBehavior: Flickable.StopAtBounds
            ScrollBar.vertical: ScrollBar {}

            Connections {
                target: App.sceneGraph
                function onModelReset() { Qt.callLater(tree.expandRecursively) }
            }
            Component.onCompleted: Qt.callLater(expandRecursively)

            delegate: Rectangle {
                id: row

                required property TreeView treeView
                required property bool isTreeNode
                required property bool expanded
                required property bool hasChildren
                required property int depth
                required property int row
                required property string name
                required property string locationType
                required property string path

                readonly property bool selected: App.sceneGraph.selectedPath === path

                implicitWidth: Math.max(tree.width, label.x + label.implicitWidth + 80)
                implicitHeight: 22
                color: selected ? Theme.selection : (rowMouse.containsMouse ? Theme.panelAlt : "transparent")

                MouseArea {
                    id: rowMouse
                    anchors.fill: parent
                    hoverEnabled: true
                    onClicked: App.sceneGraph.select(row.path)
                    onDoubleClicked: row.treeView.toggleExpanded(row.row)
                }

                Text {
                    id: indicator
                    x: 6 + row.depth * 16
                    anchors.verticalCenter: parent.verticalCenter
                    width: 12
                    text: row.hasChildren ? (row.expanded ? "▾" : "▸") : ""
                    color: Theme.textDim
                    font.pixelSize: Theme.fontSize

                    MouseArea {
                        anchors.fill: parent
                        anchors.margins: -4
                        onClicked: row.treeView.toggleExpanded(row.row)
                    }
                }

                Rectangle {
                    id: typeDot
                    anchors { left: indicator.right; leftMargin: 4; verticalCenter: parent.verticalCenter }
                    width: 8
                    height: 8
                    radius: row.locationType === "group" ? 1 : 4
                    color: root.typeColors[row.locationType] || Theme.textDim
                }

                Text {
                    id: label
                    anchors { left: typeDot.right; leftMargin: 6; verticalCenter: parent.verticalCenter }
                    text: row.name
                    color: Theme.text
                    font.pixelSize: Theme.fontSize
                }

                Text {
                    anchors { right: parent.right; rightMargin: 8; verticalCenter: parent.verticalCenter }
                    text: row.locationType
                    color: Theme.textDim
                    font.pixelSize: Theme.smallFontSize
                }
            }
        }
    }
}

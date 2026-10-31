import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Raito
import "../controls"

// Attributes of the location selected in the scene graph
Rectangle {
    id: root

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
                anchors.rightMargin: 8

                Label {
                    Layout.fillWidth: true
                    text: App.sceneGraph.selectedPath || "No location selected"
                    color: App.sceneGraph.selectedPath ? Theme.text : Theme.textDim
                    font.family: Theme.monoFont
                    font.pixelSize: Theme.smallFontSize + 1
                    elide: Text.ElideLeft
                }
                Label {
                    text: App.sceneGraph.selectedType
                    color: Theme.accent
                    font.pixelSize: Theme.smallFontSize
                }
            }
        }

        ListView {
            id: list

            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            model: App.sceneGraph.attributes
            boundsBehavior: Flickable.StopAtBounds
            ScrollBar.vertical: ScrollBar {}

            section.property: "group"
            section.delegate: Rectangle {
                required property string section
                width: ListView.view.width
                height: 22
                color: Theme.header
                Text {
                    anchors { left: parent.left; leftMargin: 8; verticalCenter: parent.verticalCenter }
                    text: section
                    color: Theme.text
                    font.bold: true
                    font.pixelSize: Theme.smallFontSize + 1
                }
            }

            delegate: Rectangle {
                id: row

                required property int index
                required property string name
                required property string type
                required property string value
                required property color swatch

                width: ListView.view.width
                height: 22
                color: index % 2 ? Theme.panelAlt : Theme.panel

                RowLayout {
                    anchors.fill: parent
                    anchors.leftMargin: 18
                    anchors.rightMargin: 8
                    spacing: 8

                    Text {
                        Layout.preferredWidth: 100
                        text: row.name
                        color: Theme.text
                        font.pixelSize: Theme.fontSize
                        elide: Text.ElideRight
                    }
                    Rectangle {
                        visible: row.type === "color"
                        width: 12
                        height: 12
                        radius: 2
                        color: row.swatch
                        border.color: Theme.border
                    }
                    Text {
                        Layout.fillWidth: true
                        text: row.value
                        color: Theme.text
                        font.family: Theme.monoFont
                        font.pixelSize: Theme.fontSize
                        elide: Text.ElideRight
                    }
                    Text {
                        text: row.type
                        color: Theme.textDim
                        font.pixelSize: Theme.smallFontSize
                    }
                }
            }

            Text {
                anchors.centerIn: parent
                visible: list.count === 0
                text: App.sceneGraph.selectedPath ? "No attributes" : "Select a location in the scene graph"
                color: Theme.textDim
                font.pixelSize: Theme.fontSize
            }
        }
    }
}

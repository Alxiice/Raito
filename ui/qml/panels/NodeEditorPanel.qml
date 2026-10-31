import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Raito
import "../controls"

// Parameters of the selected node
Rectangle {
    id: root

    readonly property var editor: App.nodeEditor

    color: Theme.panel

    Text {
        anchors.centerIn: parent
        visible: !root.editor.hasNode
        text: "Select a node to edit its parameters"
        color: Theme.textDim
        font.pixelSize: Theme.fontSize
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 8
        spacing: 8
        visible: root.editor.hasNode

        // Node header: name and type
        RowLayout {
            Layout.fillWidth: true
            spacing: 6

            Rectangle {
                Layout.preferredWidth: 6
                Layout.preferredHeight: nameField.height
                radius: 2
                color: root.editor.nodeColor
            }

            TextField {
                id: nameField
                Layout.fillWidth: true
                text: root.editor.nodeName
                font.bold: true
                font.pixelSize: Theme.fontSize + 1
                selectByMouse: true
                validator: RegularExpressionValidator { regularExpression: /[A-Za-z0-9_]*/ }
                onEditingFinished: {
                    if (text !== root.editor.nodeName && text !== "")
                        root.editor.rename(text)
                    text = Qt.binding(() => root.editor.nodeName)
                }
                ToolTip.visible: hovered
                ToolTip.text: "Node name, also used as its scene graph location name"
                ToolTip.delay: 600
            }

            Label {
                text: root.editor.nodeType
                color: Theme.textDim
            }

            Button {
                text: App.graph.viewNodeId === root.editor.nodeId ? "Viewed" : "View"
                checkable: false
                highlighted: App.graph.viewNodeId === root.editor.nodeId
                onClicked: App.graph.setViewNode(root.editor.nodeId)
            }
        }

        Label {
            Layout.fillWidth: true
            text: root.editor.nodeDescription
            color: Theme.textDim
            font.pixelSize: Theme.smallFontSize
            wrapMode: Text.WordWrap
        }

        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: Theme.border
        }

        Label {
            visible: paramList.count === 0
            text: "This node has no parameters"
            color: Theme.textDim
        }

        ListView {
            id: paramList
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            spacing: 4
            model: root.editor
            boundsBehavior: Flickable.StopAtBounds
            ScrollBar.vertical: ScrollBar {}

            delegate: RowLayout {
                id: row

                required property int index
                required property string label
                required property string type
                required property var value
                required property real min
                required property real max
                required property bool hasRange
                required property var options
                required property string tooltip

                function commit(v) { root.editor.setValue(index, v) }

                width: ListView.view.width - 12
                spacing: 8

                Label {
                    Layout.preferredWidth: 110
                    text: row.label
                    horizontalAlignment: Text.AlignRight
                    elide: Text.ElideRight
                    ToolTip.visible: row.tooltip !== "" && labelMouse.containsMouse
                    ToolTip.text: row.tooltip
                    ToolTip.delay: 500
                    MouseArea { id: labelMouse; anchors.fill: parent; hoverEnabled: true }
                }

                Loader {
                    Layout.fillWidth: true
                    sourceComponent: {
                        switch (row.type) {
                        case "float":
                        case "int": return numberEditor
                        case "point": return vecEditor
                        case "color": return colorEditor
                        case "bool": return boolEditor
                        case "enum": return enumEditor
                        default: return stringEditor
                        }
                    }

                    Component {
                        id: numberEditor
                        RowLayout {
                            spacing: 6
                            FloatField {
                                Layout.preferredWidth: row.hasRange ? 70 : -1
                                Layout.fillWidth: !row.hasRange
                                integer: row.type === "int"
                                value: row.value
                                onCommitted: (v) => row.commit(v)
                            }
                            Slider {
                                visible: row.hasRange
                                Layout.fillWidth: true
                                from: row.min
                                to: row.max
                                value: row.value
                                stepSize: row.type === "int" ? 1 : 0
                                onMoved: row.commit(value)
                            }
                        }
                    }
                    Component {
                        id: vecEditor
                        Vec3Field { value: row.value; onCommitted: (v) => row.commit(v) }
                    }
                    Component {
                        id: colorEditor
                        ColorField { value: row.value; onCommitted: (v) => row.commit(v) }
                    }
                    Component {
                        id: boolEditor
                        CheckBox { checked: row.value; onToggled: row.commit(checked) }
                    }
                    Component {
                        id: enumEditor
                        ComboBox {
                            model: row.options
                            currentIndex: row.options.indexOf(row.value)
                            onActivated: row.commit(currentText)
                        }
                    }
                    Component {
                        id: stringEditor
                        TextField {
                            text: row.value
                            selectByMouse: true
                            onEditingFinished: {
                                if (text !== row.value)
                                    row.commit(text)
                                text = Qt.binding(() => row.value)
                            }
                        }
                    }
                }
            }
        }
    }
}

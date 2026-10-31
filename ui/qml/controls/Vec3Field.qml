import QtQuick
import QtQuick.Layouts

// Three float fields editing a [x, y, z] value
RowLayout {
    id: root

    property var value: [0, 0, 0]
    property var axisColors: ["#d9534f", "#5cb85c", "#5b8fd9"]

    signal committed(var value)

    spacing: 3

    Repeater {
        model: 3
        delegate: FloatField {
            required property int index
            Layout.fillWidth: true
            Layout.minimumWidth: 40
            value: root.value[index]
            leftPadding: 10
            onCommitted: (v) => {
                const values = root.value.slice()
                values[index] = v
                root.committed(values)
            }
            Rectangle {
                width: 3
                anchors { left: parent.left; top: parent.top; bottom: parent.bottom; margins: 3 }
                color: root.axisColors[index]
                radius: 1
            }
        }
    }
}

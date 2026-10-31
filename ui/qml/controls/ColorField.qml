import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts

// RGB color: a swatch opening a color dialog, and the three float channels
RowLayout {
    id: root

    property var value: [0, 0, 0]

    signal committed(var value)

    spacing: 4

    Rectangle {
        Layout.preferredWidth: 28
        Layout.preferredHeight: 22
        radius: 3
        border.color: Theme.border
        color: Qt.rgba(root.value[0], root.value[1], root.value[2], 1)

        MouseArea {
            anchors.fill: parent
            cursorShape: Qt.PointingHandCursor
            onClicked: {
                dialog.selectedColor = parent.color
                dialog.open()
            }
        }
    }

    Vec3Field {
        Layout.fillWidth: true
        value: root.value
        axisColors: ["#d9534f", "#5cb85c", "#5b8fd9"]
        onCommitted: (v) => root.committed(v)
    }

    ColorDialog {
        id: dialog
        title: "Pick a color"
        onAccepted: root.committed([selectedColor.r, selectedColor.g, selectedColor.b])
    }
}

import QtQuick
import QtQuick.Controls

// Small flat button showing a text glyph, used in panel headers
ToolButton {
    id: button

    property string tip: ""

    implicitWidth: 22
    implicitHeight: 22
    padding: 0
    font.pixelSize: 13
    flat: true

    ToolTip.visible: hovered && tip !== ""
    ToolTip.text: tip
    ToolTip.delay: 500
}

import QtQuick
import QtQuick.Controls

// Numeric field: type a value, or drag with the middle mouse button to scrub it
TextField {
    id: field

    property real value: 0
    property bool integer: false
    property int decimals: 4
    property real scrubStep: integer ? 1 : 0.01

    signal committed(real value)

    function format(v) {
        return integer ? Math.round(v).toString() : Number(v.toFixed(decimals)).toString()
    }

    implicitWidth: 70
    text: format(value)
    selectByMouse: true
    horizontalAlignment: TextInput.AlignRight
    font.pixelSize: Theme.fontSize
    validator: RegularExpressionValidator {
        regularExpression: field.integer ? /^-?\d*$/ : /^-?\d*\.?\d*([eE][-+]?\d*)?$/
    }

    onEditingFinished: {
        const v = Number(text)
        if (text.trim() !== "" && !isNaN(v) && v !== value)
            committed(v)
        text = Qt.binding(() => format(value))
    }

    MouseArea {
        property real startX
        property real startValue

        anchors.fill: parent
        acceptedButtons: Qt.MiddleButton
        cursorShape: Qt.SizeHorCursor
        onPressed: (mouse) => { startX = mouse.x; startValue = field.value }
        onPositionChanged: (mouse) => {
            const step = field.scrubStep * ((mouse.modifiers & Qt.ShiftModifier) ? 10 : 1)
            const v = startValue + Math.round((mouse.x - startX) / 4) * step
            if (v !== field.value)
                field.committed(field.integer ? Math.round(v) : v)
        }
    }
}

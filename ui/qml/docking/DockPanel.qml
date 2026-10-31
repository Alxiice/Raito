import QtQuick

// Content managed by a DockManager: docked as a tab in a DockArea or
// isolated in its own FloatingWindow. Children should fill the panel.
Item {
    // Unique identifier used to save the layout
    property string panelId
    property string title
    // Area used by the default layout
    property string defaultArea: "center"

    // === State managed by the DockManager ===
    property string area: defaultArea
    property bool open: true
    property bool floating: false
    // Last activated panel of an area is its current tab
    property int activation: 0
    property var window: null
    property rect floatGeometry: Qt.rect(100, 100, 640, 480)

    visible: false
}

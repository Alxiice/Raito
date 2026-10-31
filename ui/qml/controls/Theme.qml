pragma Singleton
import QtQuick

QtObject {
    readonly property color window: "#1b1c1f"
    readonly property color panel: "#232427"
    readonly property color panelAlt: "#292a2e"
    readonly property color header: "#2f3035"
    readonly property color headerActive: "#3a3b41"
    readonly property color border: "#111214"
    readonly property color field: "#18191b"
    readonly property color text: "#d4d5d8"
    readonly property color textDim: "#8a8c92"
    readonly property color accent: "#e8913a"
    readonly property color selection: "#35598c"
    readonly property color error: "#e35d5d"
    readonly property color graphBackground: "#1e1f22"
    readonly property color graphGrid: "#26272b"
    readonly property color graphGridMajor: "#2e2f34"
    readonly property color nodeBody: "#34353a"

    readonly property int fontSize: 12
    readonly property int smallFontSize: 10
    readonly property string monoFont: "Consolas"
    readonly property int rowHeight: 24

    // Quick Controls do not inherit the application palette: every top
    // level window applies this one, popups and controls inherit it
    function applyPalette(p) {
        p.window = panel
        p.windowText = text
        p.base = field
        p.alternateBase = panelAlt
        p.text = text
        p.placeholderText = "#6c6e74"
        p.button = "#33343a"
        p.buttonText = text
        p.brightText = "#ffffff"
        p.highlight = accent
        p.highlightedText = "#111214"
        p.toolTipBase = header
        p.toolTipText = text
        p.light = "#45464d"
        p.midlight = "#3a3b41"
        p.mid = "#2a2b30"
        p.dark = "#141517"
        p.shadow = "#000000"
        p.disabled.text = "#6c6e74"
        p.disabled.windowText = "#6c6e74"
        p.disabled.buttonText = "#6c6e74"
    }
}

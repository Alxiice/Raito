import QtQuick
import QtQuick.Controls
import QtCore
import "../controls"

// Docking layout: six tabbed areas in resizable splits. Panel tabs can be
// dragged to another area, or out of any drop zone to float them in their
// own window. The layout is saved between sessions.
Item {
    id: manager

    default property list<DockPanel> panels
    property bool restoreSavedLayout: true

    // Bumped whenever panels move, so areas recompute their content
    property int revision: 0
    property int activationCounter: 0

    // === Tab drag state ===
    property DockPanel dragPanel: null
    property point dragPos
    property string hoverZone: ""
    property var dropZones: []

    readonly property var areas: [leftTop, leftBottom, center, bottom, rightTop, rightBottom]

    function panelsIn(areaName) {
        const result = []
        for (const p of panels)
            if (p.open && !p.floating && p.area === areaName)
                result.push(p)
        return result
    }

    function activate(panel) {
        panel.activation = ++activationCounter
    }

    function movePanel(panel, areaName) {
        if (panel.floating)
            _destroyWindow(panel)
        panel.area = areaName
        panel.open = true
        activate(panel)
        revision++
    }

    function floatPanel(panel, globalPos) {
        if (!panel || panel.floating)
            return
        const g = panel.floatGeometry
        const w = panel.width > 50 ? panel.width : g.width
        const h = panel.height > 50 ? panel.height + 26 : g.height
        panel.floatGeometry = globalPos ? Qt.rect(globalPos.x - 40, globalPos.y - 12, w, h)
                                        : Qt.rect(g.x, g.y, w, h)
        panel.floating = true
        panel.open = true
        revision++
        _createWindow(panel)
    }

    function dockPanel(panel) {
        if (!panel.floating)
            return
        _destroyWindow(panel)
        panel.open = true
        activate(panel)
        revision++
    }

    function closePanel(panel) {
        if (!panel)
            return
        if (panel.floating) {
            _destroyWindow(panel)
            // Remember it was floating: it reopens in a window
            panel.floating = true
        }
        panel.open = false
        panel.visible = false
        revision++
    }

    function showPanel(panel) {
        if (panel.open) {
            if (panel.floating) {
                panel.window.raise()
                panel.window.requestActivate()
            } else {
                activate(panel)
            }
            return
        }
        panel.open = true
        activate(panel)
        revision++
        if (panel.floating)
            _createWindow(panel)
    }

    function resetLayout() {
        for (const p of panels) {
            if (p.floating)
                _destroyWindow(p)
            p.area = p.defaultArea
            p.open = true
            p.activation = 0
        }
        revision++
    }

    function _createWindow(panel) {
        const g = panel.floatGeometry
        panel.window = floatingWindow.createObject(manager, {
            panel: panel, manager: manager, x: g.x, y: g.y, width: g.width, height: g.height
        })
    }

    function _destroyWindow(panel) {
        const win = panel.window
        if (win) {
            panel.floatGeometry = Qt.rect(win.x, win.y, win.width, win.height)
            panel.window = null
            panel.visible = false
            panel.parent = manager
            win.destroy()
        }
        panel.floating = false
    }

    // === Tab drag and drop ===

    function beginDrag(panel) {
        const zones = []
        const e = 40
        const w = width, h = height
        // Edge zones first so that empty areas can be reached
        zones.push({ area: "leftTop", rect: Qt.rect(0, 0, e, h / 2) })
        zones.push({ area: "leftBottom", rect: Qt.rect(0, h / 2, e, h / 2) })
        zones.push({ area: "rightTop", rect: Qt.rect(w - e, 0, e, h / 2) })
        zones.push({ area: "rightBottom", rect: Qt.rect(w - e, h / 2, e, h / 2) })
        zones.push({ area: "bottom", rect: Qt.rect(e, h - e, w - 2 * e, e) })
        zones.push({ area: "center", rect: Qt.rect(w / 2 - 45, h / 2 - 45, 90, 90) })
        // Then visible areas, dropped panels become a new tab
        for (const a of areas) {
            if (a.visible) {
                const p = a.mapToItem(manager, 0, 0)
                zones.push({ area: a.areaName, rect: Qt.rect(p.x, p.y, a.width, a.height), isArea: true })
            }
        }
        dropZones = zones
        dragPanel = panel
    }

    function updateDrag(pos) {
        dragPos = pos
        hoverZone = ""
        for (const z of dropZones) {
            const r = z.rect
            if (pos.x >= r.x && pos.x < r.x + r.width && pos.y >= r.y && pos.y < r.y + r.height) {
                hoverZone = z.area
                return
            }
        }
    }

    function endDrag(pos, globalPos) {
        updateDrag(pos)
        const panel = dragPanel
        const zone = hoverZone
        dragPanel = null
        hoverZone = ""
        // Defer: the dragged tab is destroyed when its panel moves
        Qt.callLater(() => {
            if (zone === "")
                floatPanel(panel, globalPos)
            else if (zone !== panel.area || panel.floating)
                movePanel(panel, zone)
        })
    }

    // === Persistence ===

    function saveLayout() {
        const state = {}
        for (const p of panels) {
            const g = p.window ? Qt.rect(p.window.x, p.window.y, p.window.width, p.window.height)
                               : p.floatGeometry
            state[p.panelId] = {
                area: p.area, open: p.open, floating: p.floating, activation: p.activation,
                geometry: [g.x, g.y, g.width, g.height]
            }
        }
        settings.panels = JSON.stringify(state)
        settings.columns = hSplit.saveState()
        settings.left = leftColumn.saveState()
        settings.middle = centerColumn.saveState()
        settings.right = rightColumn.saveState()
    }

    function restoreLayout() {
        let state
        try {
            state = JSON.parse(settings.panels)
        } catch (e) {
            return
        }
        const validAreas = areas.map(a => a.areaName)
        for (const p of panels) {
            const s = state[p.panelId]
            if (!s)
                continue
            p.area = validAreas.includes(s.area) ? s.area : p.defaultArea
            p.open = s.open
            p.floating = s.floating
            p.activation = s.activation
            activationCounter = Math.max(activationCounter, s.activation)
            p.floatGeometry = Qt.rect(s.geometry[0], s.geometry[1], s.geometry[2], s.geometry[3])
        }
        revision++
        for (const p of panels)
            if (p.open && p.floating)
                _createWindow(p)
        if (settings.columns) hSplit.restoreState(settings.columns)
        if (settings.left) leftColumn.restoreState(settings.left)
        if (settings.middle) centerColumn.restoreState(settings.middle)
        if (settings.right) rightColumn.restoreState(settings.right)
    }

    function closeAllWindows() {
        for (const p of panels)
            if (p.window)
                p.window.destroy()
    }

    Component.onCompleted: {
        if (restoreSavedLayout && settings.panels)
            restoreLayout()
        else
            revision++
    }

    Settings {
        id: settings
        category: "dockLayout"
        property string panels: ""
        property var columns
        property var left
        property var middle
        property var right
    }

    Component {
        id: floatingWindow
        FloatingWindow {}
    }

    // === Layout ===

    component Handle: Rectangle {
        implicitWidth: 4
        implicitHeight: 4
        color: SplitHandle.pressed ? Theme.accent : (SplitHandle.hovered ? Theme.headerActive : Theme.border)
    }

    SplitView {
        id: hSplit
        anchors.fill: parent
        orientation: Qt.Horizontal
        handle: Handle {}

        SplitView {
            id: leftColumn
            visible: leftTop.hasPanels || leftBottom.hasPanels
            orientation: Qt.Vertical
            handle: Handle {}
            SplitView.preferredWidth: 300
            SplitView.minimumWidth: 160
            SplitView.fillWidth: !(center.hasPanels || bottom.hasPanels) && !(rightTop.hasPanels || rightBottom.hasPanels)

            DockArea {
                id: leftTop
                areaName: "leftTop"
                manager: manager
                SplitView.fillHeight: true
                SplitView.minimumHeight: 80
            }
            DockArea {
                id: leftBottom
                areaName: "leftBottom"
                manager: manager
                SplitView.preferredHeight: 300
                SplitView.minimumHeight: 80
                SplitView.fillHeight: !leftTop.hasPanels
            }
        }

        SplitView {
            id: centerColumn
            visible: center.hasPanels || bottom.hasPanels
            orientation: Qt.Vertical
            handle: Handle {}
            SplitView.fillWidth: true
            SplitView.minimumWidth: 200

            DockArea {
                id: center
                areaName: "center"
                manager: manager
                SplitView.fillHeight: true
                SplitView.minimumHeight: 100
            }
            DockArea {
                id: bottom
                areaName: "bottom"
                manager: manager
                SplitView.preferredHeight: 340
                SplitView.minimumHeight: 100
                SplitView.fillHeight: !center.hasPanels
            }
        }

        SplitView {
            id: rightColumn
            visible: rightTop.hasPanels || rightBottom.hasPanels
            orientation: Qt.Vertical
            handle: Handle {}
            SplitView.preferredWidth: 330
            SplitView.minimumWidth: 180
            SplitView.fillWidth: !(center.hasPanels || bottom.hasPanels)

            DockArea {
                id: rightTop
                areaName: "rightTop"
                manager: manager
                SplitView.fillHeight: true
                SplitView.minimumHeight: 80
            }
            DockArea {
                id: rightBottom
                areaName: "rightBottom"
                manager: manager
                SplitView.preferredHeight: 300
                SplitView.minimumHeight: 80
                SplitView.fillHeight: !rightTop.hasPanels
            }
        }
    }

    // === Drag overlay ===

    Item {
        anchors.fill: parent
        visible: manager.dragPanel !== null
        z: 100

        Repeater {
            model: manager.dropZones

            delegate: Rectangle {
                required property var modelData
                readonly property bool hovered: manager.hoverZone === modelData.area

                visible: !modelData.isArea
                x: modelData.rect.x
                y: modelData.rect.y
                width: modelData.rect.width
                height: modelData.rect.height
                radius: 4
                color: Qt.alpha(Theme.accent, hovered ? 0.5 : 0.15)
                border.color: Theme.accent
            }
        }

        Rectangle {
            x: manager.dragPos.x + 12
            y: manager.dragPos.y + 8
            width: ghostLabel.implicitWidth + 20
            height: 24
            radius: 3
            color: Theme.headerActive
            border.color: Theme.accent

            Text {
                id: ghostLabel
                anchors.centerIn: parent
                color: Theme.text
                font.pixelSize: Theme.fontSize
                text: manager.dragPanel
                      ? manager.dragPanel.title + (manager.hoverZone === "" ? "  (release to float)" : "")
                      : ""
            }
        }
    }
}

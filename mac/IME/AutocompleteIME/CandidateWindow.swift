import Cocoa

class CandidateWindow: NSPanel {
    private var tableView: NSTableView!
    private var candidates: [String] = []
    private(set) var selectedIndex: Int = 0

    init() {
        let contentRect = NSRect(x: 0, y: 0, width: 400, height: 200)

        super.init(
            contentRect: contentRect,
            styleMask: [.borderless, .nonactivatingPanel],
            backing: .buffered,
            defer: false
        )

        self.isFloatingPanel = true
        self.level = .popUpMenu
        self.isOpaque = false
        self.hasShadow = true
        self.backgroundColor = NSColor.windowBackgroundColor.withAlphaComponent(0.95)

        setupTableView()
    }

    private func setupTableView() {
        let scrollView = NSScrollView(frame: self.contentView!.bounds)
        scrollView.autoresizingMask = [.width, .height]
        scrollView.hasVerticalScroller = true
        scrollView.borderType = .noBorder

        tableView = NSTableView(frame: scrollView.bounds)
        tableView.headerView = nil
        tableView.backgroundColor = .clear
        tableView.gridStyleMask = []
        tableView.intercellSpacing = NSSize(width: 0, height: 2)
        tableView.rowHeight = 24
        tableView.selectionHighlightStyle = .regular

        let column = NSTableColumn(identifier: NSUserInterfaceItemIdentifier("candidate"))
        column.width = scrollView.bounds.width
        tableView.addTableColumn(column)

        tableView.dataSource = self
        tableView.delegate = self

        scrollView.documentView = tableView
        self.contentView?.addSubview(scrollView)
    }

    func setCandidates(_ candidates: [String]) {
        self.candidates = candidates
        self.selectedIndex = 0
        tableView.reloadData()

        if !candidates.isEmpty {
            tableView.selectRowIndexes(IndexSet(integer: 0), byExtendingSelection: false)
        }

        // Adjust window height based on number of candidates
        let rowHeight: CGFloat = 24
        let maxVisibleRows = 8
        let visibleRows = min(candidates.count, maxVisibleRows)
        let newHeight = CGFloat(visibleRows) * rowHeight + 10

        var frame = self.frame
        frame.size.height = newHeight
        self.setFrame(frame, display: true)
    }

    func selectNext() {
        if selectedIndex < candidates.count - 1 {
            selectedIndex += 1
            tableView.selectRowIndexes(IndexSet(integer: selectedIndex), byExtendingSelection: false)
            tableView.scrollRowToVisible(selectedIndex)
        }
    }

    func selectPrevious() {
        if selectedIndex > 0 {
            selectedIndex -= 1
            tableView.selectRowIndexes(IndexSet(integer: selectedIndex), byExtendingSelection: false)
            tableView.scrollRowToVisible(selectedIndex)
        }
    }

    func show() {
        self.orderFront(nil)
    }

    func hide() {
        self.orderOut(nil)
    }
}

extension CandidateWindow: NSTableViewDataSource {
    func numberOfRows(in tableView: NSTableView) -> Int {
        return candidates.count
    }
}

extension CandidateWindow: NSTableViewDelegate {
    func tableView(_ tableView: NSTableView, viewFor tableColumn: NSTableColumn?, row: Int) -> NSView? {
        let identifier = NSUserInterfaceItemIdentifier("CandidateCell")

        var cellView = tableView.makeView(withIdentifier: identifier, owner: self) as? NSTableCellView

        if cellView == nil {
            cellView = NSTableCellView()
            cellView?.identifier = identifier

            let textField = NSTextField(frame: NSRect(x: 8, y: 4, width: 384, height: 16))
            textField.isBordered = false
            textField.backgroundColor = .clear
            textField.isEditable = false
            textField.font = NSFont.systemFont(ofSize: 13)

            cellView?.addSubview(textField)
            cellView?.textField = textField
        }

        cellView?.textField?.stringValue = candidates[row]

        return cellView
    }

    func tableViewSelectionDidChange(_ notification: Notification) {
        selectedIndex = tableView.selectedRow
    }
}

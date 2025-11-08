import Cocoa
import InputMethodKit

final class CandidateWindow {
    private var window: NSWindow?

    func show(candidates: [String], for sender: Any!) {
        guard let client = sender as? IMKTextInput else { return }
        let rect = client.boundingRect(forGlyphIndex: 0)
        let list = candidates.joined(separator: "\n")
        let textView = NSTextView(frame: NSRect(x: 0, y: 0, width: 280, height: 80))
        textView.string = list
        textView.isEditable = false

        if window == nil {
            window = NSWindow(contentRect: NSRect(x: rect.origin.x, y: rect.origin.y, width: 280, height: 80),
                              styleMask: [.titled, .hudWindow],
                              backing: .buffered,
                              defer: false)
        }
        window?.contentView = textView
        window?.makeKeyAndOrderFront(nil)
    }

    func hide() {
        window?.orderOut(nil)
    }

    var topCandidate: String? {
        return (window?.contentView as? NSTextView)?.string.components(separatedBy: "\n").first
    }
}

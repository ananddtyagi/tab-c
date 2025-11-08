import InputMethodKit
import Cocoa

final class AutocompleteInputController: IMKInputController {
    private let bridge = EngineBridge()
    private let candidateWindow = CandidateWindow()

    override func handle(_ event: NSEvent!, client sender: Any!) -> Bool {
        guard event.type == .keyDown else {
            return false
        }

        if event.keyCode == kVK_Tab {
            commitTopCandidate(sender)
            return true
        }

        if event.keyCode == kVK_Escape {
            candidateWindow.hide()
            return true
        }

        let prefix = extractPrefix(from: sender)
        bridge.send(prefix: prefix)
        candidateWindow.show(candidates: bridge.lastInstant, for: sender)
        return false
    }

    private func extractPrefix(from sender: Any!) -> String {
        guard let client = sender as? IMKTextInput else {
            return ""
        }
        let range = NSRange(location: 0, length: 128)
        if let attributed = client.attributedSubstring(from: range) {
            return attributed.string
        }
        return ""
    }

    private func commitTopCandidate(_ sender: Any!) {
        guard let client = sender as? IMKTextInput else {
            return
        }
        if let suggestion = bridge.lastInstant.first {
            client.insertText(suggestion, replacementRange: NSRange(location: NSNotFound, length: 0))
            candidateWindow.hide()
        }
    }
}

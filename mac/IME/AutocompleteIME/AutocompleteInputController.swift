import InputMethodKit
import Foundation

@objc(AutocompleteInputController)
class AutocompleteInputController: IMKInputController {
    private var currentPrefix: String = ""
    private var currentCandidates: [String] = []
    private var candidateWindow: CandidateWindow?
    private let engineClient = EngineClient.shared
    private var currentCursorId: String = ""
    private var isShowingCandidates = false

    override init!(server: IMKServer!, delegate: Any!, client inputClient: Any!) {
        super.init(server: server, delegate: delegate, client: inputClient)
        setupEngineCallbacks()
    }

    private func setupEngineCallbacks() {
        engineClient.onInstantResponse = { [weak self] response in
            DispatchQueue.main.async {
                self?.handleInstantResponse(response)
            }
        }

        engineClient.onRefinedResponse = { [weak self] response in
            DispatchQueue.main.async {
                self?.handleRefinedResponse(response)
            }
        }
    }

    override func handle(_ event: NSEvent!, client sender: Any!) -> Bool {
        guard event.type == .keyDown else { return false }

        let keyCode = event.keyCode
        let modifierFlags = event.modifierFlags

        // Handle Tab key for accepting suggestion
        if keyCode == 48 { // Tab
            if isShowingCandidates && !currentCandidates.isEmpty {
                commitTopCandidate(sender)
                return true
            }
        }

        // Handle Escape for dismissing
        if keyCode == 53 { // Escape
            if isShowingCandidates {
                dismissCandidates()
                return true
            }
        }

        // Handle arrow keys for candidate navigation
        if isShowingCandidates {
            if keyCode == 125 { // Down arrow
                candidateWindow?.selectNext()
                return true
            } else if keyCode == 126 { // Up arrow
                candidateWindow?.selectPrevious()
                return true
            }
        }

        return false
    }

    override func inputText(_ string: String!, client sender: Any!) -> Bool {
        // Text input event - request completions
        updatePrefix(from: sender)
        requestCompletions()
        return false // Let the system handle the actual text input
    }

    private func updatePrefix(from sender: Any!) {
        guard let client = sender as? IMKTextInput else {
            currentPrefix = ""
            return
        }

        // Try to get surrounding text
        var range = NSRange(location: 0, length: 512)
        if let attributedString = client.attributedSubstring(from: range) {
            let text = attributedString.string
            currentPrefix = String(text.suffix(512))
        } else {
            // Fallback: use composition text if available
            currentPrefix = ""
        }
    }

    private func requestCompletions() {
        guard currentPrefix.count >= 3 else {
            dismissCandidates()
            return
        }

        // Get app identifier
        let appId = getActiveAppIdentifier()

        // Cancel previous request
        if !currentCursorId.isEmpty {
            engineClient.cancelRequest(cursorId: currentCursorId)
        }

        // Request new completions
        currentCursorId = UUID().uuidString
        engineClient.requestCompletion(
            prefix: currentPrefix,
            maxTokens: 64,
            appId: appId,
            cursorId: currentCursorId,
            fastOnly: false
        )
    }

    private func handleInstantResponse(_ response: EngineResponse) {
        guard response.cursorId == currentCursorId else { return }

        currentCandidates = response.candidates
        if !currentCandidates.isEmpty {
            showCandidates()
        }
    }

    private func handleRefinedResponse(_ response: EngineResponse) {
        guard response.cursorId == currentCursorId else { return }

        currentCandidates = response.candidates
        if !currentCandidates.isEmpty {
            showCandidates()
        }
    }

    private func showCandidates() {
        guard !currentCandidates.isEmpty else { return }

        if candidateWindow == nil {
            candidateWindow = CandidateWindow()
        }

        candidateWindow?.setCandidates(currentCandidates)

        // Position near cursor (simplified - in production use actual caret position)
        if let screen = NSScreen.main {
            let mouseLocation = NSEvent.mouseLocation
            candidateWindow?.setFrameOrigin(mouseLocation)
        }

        candidateWindow?.show()
        isShowingCandidates = true
    }

    private func dismissCandidates() {
        candidateWindow?.hide()
        isShowingCandidates = false
        currentCandidates = []
    }

    private func commitTopCandidate(_ sender: Any!) {
        guard !currentCandidates.isEmpty,
              let client = sender as? IMKTextInput else {
            return
        }

        let selectedIndex = candidateWindow?.selectedIndex ?? 0
        let candidate = currentCandidates[selectedIndex]

        // Insert the completion
        client.insertText(candidate, replacementRange: NSRange(location: NSNotFound, length: 0))

        dismissCandidates()
    }

    private func getActiveAppIdentifier() -> String {
        if let frontApp = NSWorkspace.shared.frontmostApplication {
            return frontApp.bundleIdentifier ?? "unknown"
        }
        return "unknown"
    }

    override func deactivateServer(_ sender: Any!) {
        super.deactivateServer(sender)
        dismissCandidates()
    }
}

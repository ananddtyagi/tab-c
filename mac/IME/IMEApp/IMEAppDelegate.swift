import Cocoa
import SwiftUI

@main
class IMEAppDelegate: NSObject, NSApplicationDelegate {
    var statusItem: NSStatusItem?
    var engineProcess: Process?
    private var preferencesWindow: NSWindow?

    func applicationDidFinishLaunching(_ notification: Notification) {
        // Create menu bar item
        setupMenuBar()

        // Start the engine
        startEngine()

        // Check if IME is installed
        checkIMEStatus()
    }

    func applicationWillTerminate(_ notification: Notification) {
        stopEngine()
    }

    private func setupMenuBar() {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)

        if let button = statusItem?.button {
            button.title = "⌨️"
            button.toolTip = "MacAutoComplete"
        }

        let menu = NSMenu()

        menu.addItem(NSMenuItem(
            title: "Preferences...",
            action: #selector(openPreferences),
            keyEquivalent: ","
        ))

        menu.addItem(NSMenuItem.separator())

        menu.addItem(NSMenuItem(
            title: "Open Input Sources...",
            action: #selector(openInputSources),
            keyEquivalent: ""
        ))

        menu.addItem(NSMenuItem.separator())

        menu.addItem(NSMenuItem(
            title: "Restart Engine",
            action: #selector(restartEngine),
            keyEquivalent: "r"
        ))

        menu.addItem(NSMenuItem(
            title: "View Metrics",
            action: #selector(showMetrics),
            keyEquivalent: ""
        ))

        menu.addItem(NSMenuItem.separator())

        menu.addItem(NSMenuItem(
            title: "Quit",
            action: #selector(quit),
            keyEquivalent: "q"
        ))

        statusItem?.menu = menu
    }

    private func startEngine() {
        let homeDir = FileManager.default.homeDirectoryForCurrentUser
        let enginePath = homeDir
            .appendingPathComponent("Library/Application Support/MacAutoComplete/engine")
            .path

        guard FileManager.default.fileExists(atPath: enginePath) else {
            print("Engine binary not found at: \(enginePath)")
            showAlert(
                title: "Engine Not Found",
                message: "The autocomplete engine was not found. Please install the engine binary."
            )
            return
        }

        engineProcess = Process()
        engineProcess?.executableURL = URL(fileURLWithPath: enginePath)
        engineProcess?.arguments = [
            "--log-level", "info"
        ]

        do {
            try engineProcess?.run()
            print("Engine started")
        } catch {
            print("Failed to start engine: \(error)")
            showAlert(
                title: "Engine Start Failed",
                message: "Failed to start the autocomplete engine: \(error.localizedDescription)"
            )
        }
    }

    private func stopEngine() {
        engineProcess?.terminate()
        engineProcess = nil
    }

    @objc private func restartEngine() {
        stopEngine()
        // Wait a bit
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) { [weak self] in
            self?.startEngine()
        }
    }

    @objc private func openPreferences() {
        if preferencesWindow == nil {
            let contentView = PreferencesView()
            preferencesWindow = NSWindow(
                contentRect: NSRect(x: 0, y: 0, width: 600, height: 500),
                styleMask: [.titled, .closable, .miniaturizable],
                backing: .buffered,
                defer: false
            )
            preferencesWindow?.title = "MacAutoComplete Preferences"
            preferencesWindow?.contentView = NSHostingView(rootView: contentView)
            preferencesWindow?.center()
        }

        preferencesWindow?.makeKeyAndOrderFront(nil)
        NSApp.activate(ignoringOtherApps: true)
    }

    @objc private func openInputSources() {
        // Open System Preferences to Keyboard -> Input Sources
        let url = URL(string: "x-apple.systempreferences:com.apple.preference.keyboard?Input")!
        NSWorkspace.shared.open(url)
    }

    @objc private func showMetrics() {
        // TODO: Implement metrics display
        showAlert(
            title: "Metrics",
            message: "Metrics viewer not yet implemented. Check console logs for performance data."
        )
    }

    @objc private func quit() {
        NSApplication.shared.terminate(nil)
    }

    private func checkIMEStatus() {
        // Check if IME is registered
        // This is a simplified check - production would use TIS APIs
        print("Checking IME installation status...")
    }

    private func showAlert(title: String, message: String) {
        let alert = NSAlert()
        alert.messageText = title
        alert.informativeText = message
        alert.alertStyle = .warning
        alert.addButton(withTitle: "OK")
        alert.runModal()
    }
}

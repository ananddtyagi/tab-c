import Foundation

final class EngineBridge: ObservableObject {
    @Published var lastInstant: [String] = []
    @Published var lastRefined: [String] = []

    private let socketPath: String

    init(socketPath: String = NSHomeDirectory().appending("/Library/Application Support/MacAutoComplete/engine.sock")) {
        self.socketPath = socketPath
    }

    func send(prefix: String) {
        // Placeholder for IPC connection. On macOS this would open a UNIX domain socket.
        DispatchQueue.main.async {
            self.lastInstant = ["\(prefix) instant"]
            self.lastRefined = ["\(prefix) refined"]
        }
    }
}

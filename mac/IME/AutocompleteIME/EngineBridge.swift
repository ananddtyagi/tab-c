import Foundation

struct EngineRequest: Codable {
    let type: String
    let prefix: String?
    let max_tokens: Int?
    let app: String?
    let cursor_id: String?
    let fast_only: Bool?
}

struct EngineResponse: Codable {
    let type: String
    let cursorId: String
    let candidates: [String]
    let scores: [Float]?
    let expiresInMs: UInt64?
    let modelInfo: ModelInfo?

    enum CodingKeys: String, CodingKey {
        case type
        case cursorId = "cursor_id"
        case candidates
        case scores
        case expiresInMs = "expires_in_ms"
        case modelInfo = "model_info"
    }
}

struct ModelInfo: Codable {
    let name: String
    let latencyMs: UInt64
    let tokensPerSec: Float?

    enum CodingKeys: String, CodingKey {
        case name
        case latencyMs = "latency_ms"
        case tokensPerSec = "tokens_per_sec"
    }
}

class EngineClient {
    static let shared = EngineClient()

    private var socket: CFSocket?
    private var socketAddress: Data?
    private var isConnected = false
    private let socketQueue = DispatchQueue(label: "com.macautocomplete.engine.socket")

    var onInstantResponse: ((EngineResponse) -> Void)?
    var onRefinedResponse: ((EngineResponse) -> Void)?
    var onError: ((String) -> Void)?

    private init() {
        connect()
    }

    func connect() {
        socketQueue.async { [weak self] in
            self?.performConnect()
        }
    }

    private func performConnect() {
        let homeDir = FileManager.default.homeDirectoryForCurrentUser
        let socketPath = homeDir
            .appendingPathComponent("Library/Application Support/MacAutoComplete/engine.sock")
            .path

        // Create Unix domain socket
        var addr = sockaddr_un()
        addr.sun_family = sa_family_t(AF_UNIX)

        guard socketPath.utf8CString.count <= MemoryLayout.size(ofValue: addr.sun_path) else {
            print("Socket path too long")
            return
        }

        _ = withUnsafeMutablePointer(to: &addr.sun_path.0) { ptr in
            socketPath.withCString { cString in
                strcpy(ptr, cString)
            }
        }

        var sockaddrUnSize = socklen_t(MemoryLayout<sockaddr_un>.size)
        socketAddress = Data(bytes: &addr, count: Int(sockaddrUnSize))

        // Create socket
        let sock = Darwin.socket(AF_UNIX, SOCK_STREAM, 0)
        guard sock >= 0 else {
            print("Failed to create socket")
            return
        }

        // Connect
        let result = socketAddress!.withUnsafeBytes { (ptr: UnsafeRawBufferPointer) -> Int32 in
            let sockaddrPtr = ptr.baseAddress!.assumingMemoryBound(to: sockaddr.self)
            return Darwin.connect(sock, sockaddrPtr, sockaddrUnSize)
        }

        if result == 0 {
            isConnected = true
            print("Connected to engine")
            startReadingSocket(sock)
        } else {
            print("Failed to connect to engine socket: \(String(cString: strerror(errno)))")
            close(sock)
        }
    }

    private func startReadingSocket(_ sock: Int32) {
        // Simple read loop (production would use RunLoop or GCD)
        DispatchQueue.global(qos: .userInteractive).async { [weak self] in
            var buffer = [UInt8](repeating: 0, count: 65536)

            while true {
                let bytesRead = Darwin.read(sock, &buffer, buffer.count)
                if bytesRead > 0 {
                    let data = Data(buffer[..<bytesRead])
                    self?.handleReceivedData(data)
                } else {
                    print("Socket closed or error")
                    break
                }
            }
        }
    }

    private func handleReceivedData(_ data: Data) {
        // Parse length-prefixed message
        guard data.count >= 4 else { return }

        let length = data.withUnsafeBytes { $0.load(as: UInt32.self).littleEndian }
        let jsonStart = 4
        let jsonEnd = jsonStart + Int(length)

        guard jsonEnd <= data.count else { return }

        let jsonData = data.subdata(in: jsonStart..<jsonEnd)

        do {
            let response = try JSONDecoder().decode(EngineResponse.self, from: jsonData)

            switch response.type {
            case "instant":
                onInstantResponse?(response)
            case "refined":
                onRefinedResponse?(response)
            default:
                print("Unknown response type: \(response.type)")
            }
        } catch {
            print("Failed to decode response: \(error)")
        }
    }

    func requestCompletion(prefix: String, maxTokens: Int, appId: String, cursorId: String, fastOnly: Bool) {
        let request = EngineRequest(
            type: "complete",
            prefix: prefix,
            max_tokens: maxTokens,
            app: appId,
            cursor_id: cursorId,
            fast_only: fastOnly
        )

        sendRequest(request)
    }

    func cancelRequest(cursorId: String) {
        let request = EngineRequest(
            type: "cancel",
            prefix: nil,
            max_tokens: nil,
            app: nil,
            cursor_id: cursorId,
            fast_only: nil
        )

        sendRequest(request)
    }

    private func sendRequest(_ request: EngineRequest) {
        guard isConnected else {
            print("Not connected to engine")
            return
        }

        do {
            let jsonData = try JSONEncoder().encode(request)
            let length = UInt32(jsonData.count).littleEndian

            var message = Data()
            message.append(withUnsafeBytes(of: length) { Data($0) })
            message.append(jsonData)

            // Send message (simplified - production should handle partial writes)
            // This is a placeholder - actual socket writing would go here
            print("Would send request: \(request.type)")
        } catch {
            print("Failed to encode request: \(error)")
        }
    }
}

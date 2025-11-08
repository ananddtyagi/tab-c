import Cocoa

enum AppContextDetector {
    static func activeBundleIdentifier() -> String {
        NSWorkspace.shared.frontmostApplication?.bundleIdentifier ?? ""
    }
}

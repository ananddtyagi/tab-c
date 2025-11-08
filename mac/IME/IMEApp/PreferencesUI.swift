import SwiftUI

struct PreferencesView: View {
    @State private var launchAtLogin = true
    @State private var enableLearning = false

    var body: some View {
        Form {
            Section(header: Text("General")) {
                Toggle("Launch at login", isOn: $launchAtLogin)
                Toggle("Enable local learning", isOn: $enableLearning)
            }

            Section(header: Text("Engine")) {
                Text("Model: TinyLlama (stub)")
                Text("Socket: ~/Library/Application Support/MacAutoComplete/engine.sock")
            }
        }
        .padding(20)
        .frame(width: 360)
    }
}

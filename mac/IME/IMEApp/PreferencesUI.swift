import SwiftUI

struct PreferencesView: View {
    @State private var enableLearning = false
    @State private var enableTelemetry = false
    @State private var minPrefixLength = 3
    @State private var maxTokens = 64
    @State private var temperature = 0.7
    @State private var selectedTab = 0

    var body: some View {
        TabView(selection: $selectedTab) {
            GeneralPreferencesView(
                enableLearning: $enableLearning,
                enableTelemetry: $enableTelemetry,
                minPrefixLength: $minPrefixLength
            )
            .tabItem {
                Label("General", systemImage: "gear")
            }
            .tag(0)

            ModelPreferencesView(
                maxTokens: $maxTokens,
                temperature: $temperature
            )
            .tabItem {
                Label("Model", systemImage: "cpu")
            }
            .tag(1)

            PrivacyPreferencesView(
                enableLearning: $enableLearning,
                enableTelemetry: $enableTelemetry
            )
            .tabItem {
                Label("Privacy", systemImage: "lock.shield")
            }
            .tag(2)

            AppRulesPreferencesView()
                .tabItem {
                    Label("Apps", systemImage: "app.badge")
                }
                .tag(3)
        }
        .frame(width: 600, height: 500)
        .padding()
    }
}

struct GeneralPreferencesView: View {
    @Binding var enableLearning: Bool
    @Binding var enableTelemetry: Bool
    @Binding var minPrefixLength: Int

    var body: some View {
        Form {
            Section(header: Text("Behavior")) {
                Stepper("Minimum prefix length: \(minPrefixLength)", value: $minPrefixLength, in: 1...10)
                    .help("Number of characters before suggestions appear")
            }

            Section(header: Text("Features")) {
                Toggle("Enable learning from acceptances", isOn: $enableLearning)
                    .help("Improve suggestions based on your accepted completions")

                Toggle("Enable anonymous telemetry", isOn: $enableTelemetry)
                    .help("Help improve the product by sending anonymous usage data")
            }
        }
        .padding()
    }
}

struct ModelPreferencesView: View {
    @Binding var maxTokens: Int
    @Binding var temperature: Double

    var body: some View {
        Form {
            Section(header: Text("Generation Settings")) {
                Stepper("Max tokens: \(maxTokens)", value: $maxTokens, in: 16...256, step: 16)
                    .help("Maximum length of suggestions")

                HStack {
                    Text("Temperature:")
                    Slider(value: $temperature, in: 0.0...1.0)
                    Text(String(format: "%.2f", temperature))
                }
                .help("Higher values make suggestions more creative but less predictable")
            }

            Section(header: Text("Model Info")) {
                HStack {
                    Text("Model:")
                    Spacer()
                    Text("TinyLlama 1.1B Q4")
                        .foregroundColor(.secondary)
                }

                HStack {
                    Text("Status:")
                    Spacer()
                    Text("Running")
                        .foregroundColor(.green)
                }
            }
        }
        .padding()
    }
}

struct PrivacyPreferencesView: View {
    @Binding var enableLearning: Bool
    @Binding var enableTelemetry: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: 20) {
            Text("Privacy & Data")
                .font(.headline)

            VStack(alignment: .leading, spacing: 10) {
                Text("MacAutoComplete runs 100% locally on your device.")
                    .font(.body)

                Text("• No data is sent to external servers")
                Text("• All processing happens on your Mac")
                Text("• Your text never leaves your device")
            }
            .font(.system(size: 13))

            Divider()

            Toggle("Learn from my accepted suggestions", isOn: $enableLearning)

            if enableLearning {
                Text("Accepted completions will be stored locally to improve future suggestions. This data never leaves your device.")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Divider()

            Toggle("Send anonymous usage statistics", isOn: $enableTelemetry)

            if enableTelemetry {
                Text("Anonymous performance metrics (latency, acceptance rate) help us improve the product. No text content is ever sent.")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            Button("Clear All Local Data") {
                clearLocalData()
            }
            .foregroundColor(.red)
        }
        .padding()
    }

    private func clearLocalData() {
        // TODO: Implement data clearing
        print("Clearing local data...")
    }
}

struct AppRulesPreferencesView: View {
    @State private var excludedApps: [String] = [
        "1Password",
        "Keychain Access"
    ]

    var body: some View {
        VStack(alignment: .leading) {
            Text("Per-App Rules")
                .font(.headline)

            Text("Autocomplete is automatically disabled in password managers and secure fields.")
                .font(.caption)
                .foregroundColor(.secondary)
                .padding(.bottom)

            List {
                Section(header: Text("Excluded Apps")) {
                    ForEach(excludedApps, id: \.self) { app in
                        HStack {
                            Image(systemName: "app")
                            Text(app)
                            Spacer()
                            Button("Remove") {
                                excludedApps.removeAll { $0 == app }
                            }
                            .buttonStyle(.borderless)
                        }
                    }
                }
            }

            Button("Add App...") {
                // TODO: Implement app picker
            }
        }
        .padding()
    }
}

#Preview {
    PreferencesView()
}

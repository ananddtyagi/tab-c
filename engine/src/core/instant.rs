use crate::ipc::protocol::CompleteRequest;

#[derive(Clone)]
pub struct InstantEngine {
    max_candidates: usize,
}

impl InstantEngine {
    pub fn new(max_candidates: usize) -> Self {
        Self { max_candidates }
    }

    pub fn suggest(&self, request: &CompleteRequest) -> Vec<String> {
        let prefix = request.prefix.trim_end();
        if prefix.is_empty() {
            return Vec::new();
        }

        let last_word = prefix
            .rsplit_once(' ')
            .map(|(_, tail)| tail)
            .unwrap_or(prefix);

        let mut candidates = vec![format!("{}{}", prefix, suggest_suffix(last_word, ""))];
        candidates.push(format!("{}{}", prefix, suggest_suffix(last_word, " more")));
        candidates.push(format!("{}{}", prefix, suggest_suffix(last_word, " text")));
        candidates.truncate(self.max_candidates);
        candidates
    }
}

fn suggest_suffix(word: &str, suffix: &str) -> String {
    if word.is_empty() {
        return format!("{}{}", suffix, if suffix.is_empty() { "…" } else { "" });
    }
    match word {
        "hel" | "hell" | "hello" => format!("lo{}", suffix),
        "than" | "thank" => format!("k{}", suffix),
        _ => format!("{}{}", "", suffix),
    }
}

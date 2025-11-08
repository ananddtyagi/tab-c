/// Context extraction and management
use std::collections::VecDeque;

const MAX_CONTEXT_TOKENS: usize = 512;

pub struct Context {
    history: VecDeque<String>,
    max_tokens: usize,
}

impl Context {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            max_tokens: MAX_CONTEXT_TOKENS,
        }
    }

    pub fn with_capacity(max_tokens: usize) -> Self {
        Self {
            history: VecDeque::new(),
            max_tokens,
        }
    }

    /// Extract the relevant prefix from the full text
    /// This is a simplified version; a production version would use proper tokenization
    pub fn extract_prefix(text: &str, max_chars: usize) -> String {
        let trimmed = text.trim_end();
        if trimmed.len() <= max_chars {
            trimmed.to_string()
        } else {
            // Take the last max_chars characters
            let start = trimmed.len() - max_chars;
            trimmed[start..].to_string()
        }
    }

    /// Normalize whitespace
    pub fn normalize(text: &str) -> String {
        text.split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Add to history (for learning)
    pub fn add_to_history(&mut self, text: String) {
        self.history.push_back(text);

        // Trim if too long (approximate by counting items)
        while self.history.len() > 100 {
            self.history.pop_front();
        }
    }

    /// Get recent context window
    pub fn get_context_window(&self, num_items: usize) -> Vec<String> {
        self.history
            .iter()
            .rev()
            .take(num_items)
            .rev()
            .cloned()
            .collect()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_prefix() {
        let text = "The quick brown fox jumps over the lazy dog";
        let prefix = Context::extract_prefix(text, 20);
        assert_eq!(prefix, "over the lazy dog");
    }

    #[test]
    fn test_normalize() {
        let text = "The   quick\n  brown\tfox";
        let normalized = Context::normalize(text);
        assert_eq!(normalized, "The quick brown fox");
    }

    #[test]
    fn test_context_history() {
        let mut ctx = Context::new();
        ctx.add_to_history("first".to_string());
        ctx.add_to_history("second".to_string());
        ctx.add_to_history("third".to_string());

        let window = ctx.get_context_window(2);
        assert_eq!(window, vec!["second".to_string(), "third".to_string()]);
    }
}

/// Utility functions

use std::time::SystemTime;

/// Get current timestamp in milliseconds
pub fn timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

/// Sanitize text for display
pub fn sanitize_text(text: &str, max_len: usize) -> String {
    let sanitized: String = text
        .chars()
        .filter(|c| !c.is_control() || c.is_whitespace())
        .take(max_len)
        .collect();
    sanitized
}

/// Truncate string to max length with ellipsis
pub fn truncate_with_ellipsis(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize() {
        let text = "hello\x00world\ntest";
        let sanitized = sanitize_text(text, 100);
        assert!(!sanitized.contains('\x00'));
    }

    #[test]
    fn test_truncate() {
        let text = "hello world this is a long string";
        let truncated = truncate_with_ellipsis(text, 10);
        assert_eq!(truncated, "hello w...");
    }
}

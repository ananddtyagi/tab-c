pub fn normalize_prefix(prefix: &str) -> String {
    prefix
        .trim_end_matches(|c: char| c.is_whitespace())
        .to_string()
}

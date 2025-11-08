/// Ranking and merging of instant + refined suggestions

pub struct Ranker;

impl Ranker {
    /// Merge instant and refined suggestions, removing duplicates and ranking
    pub fn merge_and_rank(
        instant: Vec<String>,
        refined: Vec<String>,
        prefix: &str,
    ) -> Vec<String> {
        let mut candidates = Vec::new();
        let mut seen = std::collections::HashSet::new();

        // Prioritize refined suggestions if they match the prefix
        for candidate in refined {
            if candidate.to_lowercase().starts_with(&prefix.to_lowercase()) {
                if seen.insert(candidate.to_lowercase()) {
                    candidates.push(candidate);
                }
            }
        }

        // Add instant suggestions
        for candidate in instant {
            if seen.insert(candidate.to_lowercase()) {
                candidates.push(candidate);
            }
        }

        // Sort by length (shorter first) and relevance
        candidates.sort_by_key(|c| c.len());

        candidates
    }

    /// Score a candidate based on various factors
    pub fn score_candidate(candidate: &str, prefix: &str, context: &str) -> f32 {
        let mut score = 0.0;

        // Exact prefix match
        if candidate.to_lowercase().starts_with(&prefix.to_lowercase()) {
            score += 1.0;
        }

        // Length penalty (prefer shorter, more concise completions)
        let length_penalty = 1.0 / (1.0 + (candidate.len() as f32 / 100.0));
        score += length_penalty * 0.5;

        // Context similarity (simple word overlap)
        let context_words: std::collections::HashSet<&str> =
            context.split_whitespace().collect();
        let candidate_words: std::collections::HashSet<&str> =
            candidate.split_whitespace().collect();
        let overlap = context_words.intersection(&candidate_words).count();
        score += (overlap as f32) * 0.1;

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_and_rank() {
        let instant = vec!["hello world".to_string(), "hello there".to_string()];
        let refined = vec!["hello world!".to_string(), "hello friend".to_string()];

        let merged = Ranker::merge_and_rank(instant, refined, "hello");
        assert!(!merged.is_empty());
    }

    #[test]
    fn test_score_candidate() {
        let score = Ranker::score_candidate("hello world", "hello", "hello there friend");
        assert!(score > 0.0);
    }
}

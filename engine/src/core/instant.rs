use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use tracing::{info, warn};

use crate::config::Config;

/// Trie node for prefix-based autocomplete
#[derive(Default, Clone)]
struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    candidates: Vec<(String, f32)>, // (completion, score)
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self::default()
    }
}

/// Instant suggestion engine using a trie data structure
pub struct InstantEngine {
    root: TrieNode,
    fallback_suggestions: Vec<String>,
}

impl InstantEngine {
    pub fn new(config: &Config) -> Result<Self> {
        let mut engine = Self {
            root: TrieNode::new(),
            fallback_suggestions: Self::default_suggestions(),
        };

        // Try to load trie from disk
        if Path::new(&config.instant.trie_path).exists() {
            info!("Loading trie from: {}", config.instant.trie_path);
            match engine.load_from_file(&config.instant.trie_path) {
                Ok(_) => info!("Trie loaded successfully"),
                Err(e) => warn!("Failed to load trie: {}, using defaults", e),
            }
        } else {
            info!("No trie file found, using default suggestions");
            engine.build_default_trie();
        }

        Ok(engine)
    }

    /// Build a default trie with common completions
    fn build_default_trie(&mut self) {
        let common_phrases = vec![
            // Common English
            ("the ", 1.0),
            ("and ", 0.9),
            ("for ", 0.8),
            ("with ", 0.8),
            ("this ", 0.7),
            ("that ", 0.7),
            ("from ", 0.7),
            ("have ", 0.7),
            ("will ", 0.6),
            ("your ", 0.6),
            ("more ", 0.6),
            ("when ", 0.6),
            ("where ", 0.6),
            ("which ", 0.6),
            // Programming
            ("function ", 0.8),
            ("return ", 0.8),
            ("const ", 0.8),
            ("let ", 0.8),
            ("var ", 0.7),
            ("if ", 0.7),
            ("else ", 0.7),
            ("for ", 0.7),
            ("while ", 0.7),
            ("import ", 0.8),
            ("export ", 0.8),
            ("class ", 0.8),
            ("public ", 0.7),
            ("private ", 0.7),
            ("static ", 0.7),
            ("async ", 0.8),
            ("await ", 0.8),
            ("try ", 0.7),
            ("catch ", 0.7),
            ("throw ", 0.7),
            // Common sentence starters
            ("I think ", 0.7),
            ("I will ", 0.7),
            ("I am ", 0.7),
            ("I have ", 0.7),
            ("You can ", 0.7),
            ("We should ", 0.7),
            ("There is ", 0.6),
            ("There are ", 0.6),
            ("It is ", 0.6),
            ("This is ", 0.6),
        ];

        for (phrase, score) in common_phrases {
            self.insert(phrase, score);
        }
    }

    /// Insert a phrase into the trie
    pub fn insert(&mut self, phrase: &str, score: f32) {
        let mut node = &mut self.root;

        for ch in phrase.chars() {
            node = node
                .children
                .entry(ch)
                .or_insert_with(|| Box::new(TrieNode::new()));
        }

        node.is_end = true;
        node.candidates.push((phrase.to_string(), score));
        node.candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    }

    /// Get suggestions for a prefix
    pub fn suggest(&self, prefix: &str, max_results: usize) -> Vec<String> {
        if prefix.is_empty() {
            return Vec::new();
        }

        let prefix_lower = prefix.to_lowercase();
        let mut node = &self.root;

        // Navigate to the prefix node
        for ch in prefix_lower.chars() {
            if let Some(child) = node.children.get(&ch) {
                node = child;
            } else {
                // Prefix not found, return fuzzy matches or fallback
                return self.fuzzy_match(&prefix_lower, max_results);
            }
        }

        // Collect all completions from this node
        let mut results = Vec::new();
        self.collect_completions(node, prefix, &mut results, max_results);

        // If we don't have enough results, add fuzzy matches
        if results.len() < max_results {
            let fuzzy = self.fuzzy_match(&prefix_lower, max_results - results.len());
            results.extend(fuzzy);
        }

        results.truncate(max_results);
        results
    }

    /// Collect completions from a node
    fn collect_completions(
        &self,
        node: &TrieNode,
        prefix: &str,
        results: &mut Vec<String>,
        max_results: usize,
    ) {
        if results.len() >= max_results {
            return;
        }

        // Add candidates from this node
        for (candidate, _score) in &node.candidates {
            if results.len() >= max_results {
                break;
            }
            // Return the completion (suffix only)
            if let Some(suffix) = candidate.strip_prefix(prefix) {
                if !suffix.is_empty() {
                    results.push(suffix.to_string());
                }
            }
        }

        // Recursively collect from children (DFS)
        for (_ch, child) in &node.children {
            if results.len() >= max_results {
                break;
            }
            self.collect_completions(child, prefix, results, max_results);
        }
    }

    /// Fuzzy matching for when exact prefix doesn't exist
    fn fuzzy_match(&self, prefix: &str, max_results: usize) -> Vec<String> {
        // Simple fuzzy match: return suggestions that contain the prefix
        self.fallback_suggestions
            .iter()
            .filter(|s| s.to_lowercase().contains(prefix))
            .take(max_results)
            .cloned()
            .collect()
    }

    /// Default suggestions when no trie is available
    fn default_suggestions() -> Vec<String> {
        vec![
            "the ".to_string(),
            "and ".to_string(),
            "for ".to_string(),
            "with ".to_string(),
            "that ".to_string(),
        ]
    }

    /// Load trie from a file (simple text format: one phrase per line)
    pub fn load_from_file(&mut self, path: &str) -> Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let phrase = line?;
            if !phrase.is_empty() {
                self.insert(&phrase, 1.0);
            }
        }

        Ok(())
    }

    /// Save trie to a file
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        let phrases = self.collect_all_phrases();
        for phrase in phrases {
            writeln!(writer, "{}", phrase)?;
        }

        Ok(())
    }

    /// Collect all phrases from the trie
    fn collect_all_phrases(&self) -> Vec<String> {
        let mut phrases = Vec::new();
        self.collect_phrases_recursive(&self.root, String::new(), &mut phrases);
        phrases
    }

    fn collect_phrases_recursive(
        &self,
        node: &TrieNode,
        current: String,
        phrases: &mut Vec<String>,
    ) {
        if node.is_end {
            phrases.push(current.clone());
        }

        for (ch, child) in &node.children {
            let mut new_current = current.clone();
            new_current.push(*ch);
            self.collect_phrases_recursive(child, new_current, phrases);
        }
    }

    /// Learn from user acceptance
    pub fn learn(&mut self, accepted_completion: &str) {
        self.insert(accepted_completion, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_basic() {
        let mut engine = InstantEngine {
            root: TrieNode::new(),
            fallback_suggestions: vec![],
        };

        engine.insert("hello world", 1.0);
        engine.insert("hello there", 0.9);

        let results = engine.suggest("hello", 5);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_trie_prefix() {
        let mut engine = InstantEngine {
            root: TrieNode::new(),
            fallback_suggestions: vec![],
        };

        engine.insert("the quick brown fox", 1.0);
        engine.insert("the quick brown bear", 0.9);

        let results = engine.suggest("the quick", 5);
        assert!(!results.is_empty());
    }
}

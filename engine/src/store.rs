//! Persistent rule storage — load rule sets from JSON or YAML files.
//!
//! Enable the **`yaml`** feature to unlock YAML support.

use std::collections::HashMap;
use std::path::Path;

use crate::metadata::RuleDefinition;
use crate::RuleEngineError;

/// A named collection of [`RuleDefinition`] objects loaded from disk.
///
/// Rules can be loaded from a JSON array file or a YAML array file.
/// After loading, individual rules are retrieved by name.
///
/// # JSON file format
/// ```json
/// [
///   { "name": "rule-a", "logic": { "...": "..." } },
///   { "name": "rule-b", "version": "1.0.0", "logic": { "...": "..." } }
/// ]
/// ```
///
/// # YAML file format
/// ```yaml
/// - name: rule-a
///   logic:
///     ">": [{"var": "score"}, 700]
/// ```
#[derive(Debug, Default)]
pub struct RuleStore {
    rules: Vec<RuleDefinition>,
    index: HashMap<String, usize>,
}

impl RuleStore {
    /// Create an empty rule store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build a name → position index for O(1) lookups.
    fn build_index(rules: &[RuleDefinition]) -> HashMap<String, usize> {
        rules
            .iter()
            .enumerate()
            .map(|(i, r)| (r.name.clone(), i))
            .collect()
    }

    /// Load rules from a JSON file (array of `RuleDefinition` objects).
    pub fn from_json_file(path: impl AsRef<Path>) -> Result<Self, RuleEngineError> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            RuleEngineError::InvalidRule(format!("cannot read {}: {}", path.as_ref().display(), e))
        })?;
        Self::from_json_str(&content)
    }

    /// Load rules from a JSON string (array of `RuleDefinition` objects).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tempus_engine::RuleStore;
    ///
    /// let json = r#"[
    ///   {"name": "rule-a", "logic": {">": [{"var": "score"}, 700]}},
    ///   {"name": "rule-b", "logic": {"<": [{"var": "risk"}, 0.3]}}
    /// ]"#;
    ///
    /// let store = RuleStore::from_json_str(json).unwrap();
    /// assert_eq!(store.len(), 2);
    /// assert!(store.get("rule-a").is_some());
    /// ```
    pub fn from_json_str(json: &str) -> Result<Self, RuleEngineError> {
        let rules: Vec<RuleDefinition> = serde_json::from_str(json)
            .map_err(|e| RuleEngineError::InvalidRule(format!("invalid rule JSON: {e}")))?;
        let index = Self::build_index(&rules);
        Ok(Self { rules, index })
    }

    /// Load rules from a YAML file (array of `RuleDefinition` objects).
    ///
    /// Requires the `yaml` feature to be enabled.
    #[cfg(feature = "yaml")]
    pub fn from_yaml_file(path: impl AsRef<Path>) -> Result<Self, RuleEngineError> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            RuleEngineError::InvalidRule(format!("cannot read {}: {}", path.as_ref().display(), e))
        })?;
        Self::from_yaml_str(&content)
    }

    /// Load rules from a YAML string.
    ///
    /// Requires the `yaml` feature to be enabled.
    #[cfg(feature = "yaml")]
    pub fn from_yaml_str(yaml: &str) -> Result<Self, RuleEngineError> {
        let rules: Vec<RuleDefinition> = serde_yaml::from_str(yaml)
            .map_err(|e| RuleEngineError::InvalidRule(format!("invalid rule YAML: {e}")))?;
        let index = Self::build_index(&rules);
        Ok(Self { rules, index })
    }

    /// Retrieve a rule by name.
    ///
    /// Returns `None` if no rule with that name exists in the store.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tempus_engine::RuleStore;
    ///
    /// let json = r#"[{"name": "age-check", "logic": {">": [{"var": "age"}, 18]}}]"#;
    /// let store = RuleStore::from_json_str(json).unwrap();
    ///
    /// assert!(store.get("age-check").is_some());
    /// assert!(store.get("missing-rule").is_none());
    /// ```
    pub fn get(&self, name: &str) -> Option<&RuleDefinition> {
        self.index.get(name).map(|&i| &self.rules[i])
    }

    /// All rules in the store.
    pub fn all(&self) -> &[RuleDefinition] {
        &self.rules
    }

    /// Number of rules in the store.
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    /// Returns `true` if the store contains no rules.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Add a rule to the store at runtime.
    pub fn insert(&mut self, rule: RuleDefinition) {
        if let Some(&idx) = self.index.get(&rule.name) {
            self.rules[idx] = rule;
        } else {
            let idx = self.rules.len();
            self.index.insert(rule.name.clone(), idx);
            self.rules.push(rule);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample_json() -> &'static str {
        r#"[
            {"name":"rule-a","logic":{">":[{"var":"x"},1]}},
            {"name":"rule-b","version":"2.0.0","logic":{"==":[{"var":"y"},true]}}
        ]"#
    }

    #[test]
    fn load_from_json_str() {
        let store = RuleStore::from_json_str(sample_json()).unwrap();
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn get_by_name() {
        let store = RuleStore::from_json_str(sample_json()).unwrap();
        let rule = store.get("rule-b").unwrap();
        assert_eq!(rule.version.as_deref(), Some("2.0.0"));
    }

    #[test]
    fn get_missing_returns_none() {
        let store = RuleStore::from_json_str(sample_json()).unwrap();
        assert!(store.get("does-not-exist").is_none());
    }

    #[test]
    fn insert_adds_and_replaces() {
        let mut store = RuleStore::new();
        store.insert(RuleDefinition::new("r", json!({})));
        assert_eq!(store.len(), 1);
        store.insert(RuleDefinition::new("r", json!({"==":[1,1]})));
        assert_eq!(store.len(), 1); // replaced, not duplicated
    }

    #[test]
    fn invalid_json_returns_error() {
        assert!(RuleStore::from_json_str("{bad}").is_err());
    }
}

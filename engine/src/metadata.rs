//! Rule metadata and builder API.
//!
//! [`RuleDefinition`] is the core struct that pairs a JSON-Logic rule with
//! governance metadata: name, version, tags, description, and required
//! context keys.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::ValidationError;

/// A rule definition with metadata for decision systems.
///
/// This wraps a JSON-Logic rule blob with identifying information:
/// name, version, tags, and optional description. The metadata is
/// carried alongside the rule for governance, audit, and traceability.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RuleDefinition {
    /// Unique name identifying this rule.
    pub name: String,

    /// The JSON-Logic rule blob.
    pub logic: Value,

    /// Optional semantic version for this rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Optional human-readable description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Tags for categorization and filtering.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    /// Context keys that must be present for this rule to execute.
    /// Used by `validate_context` to enforce schema contracts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_context_keys: Vec<String>,
}

impl RuleDefinition {
    /// Create a new rule definition with a name and logic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tempus_engine::metadata::RuleDefinition;
    ///
    /// let rule = RuleDefinition::new(
    ///     "approve-check",
    ///     serde_json::json!({">": [{"var": "score"}, 700]}),
    /// );
    /// assert_eq!(rule.name, "approve-check");
    /// assert!(rule.version.is_none());
    /// ```
    pub fn new(name: impl Into<String>, logic: Value) -> Self {
        Self {
            name: name.into(),
            logic,
            version: None,
            description: None,
            tags: Vec::new(),
            required_context_keys: Vec::new(),
        }
    }

    /// Builder: set the version.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tempus_engine::metadata::RuleDefinition;
    ///
    /// let rule = RuleDefinition::new("r", serde_json::json!(true))
    ///     .with_version("2.0.0");
    /// assert_eq!(rule.version.as_deref(), Some("2.0.0"));
    /// ```
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Builder: set the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Builder: set tags.
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Builder: declare which context keys are required.
    pub fn with_required_keys(mut self, keys: Vec<String>) -> Self {
        self.required_context_keys = keys;
        self
    }

    /// Validate that a parsed JSON context object contains all required keys.
    ///
    /// Returns `Ok(())` when all keys are present, or a [`ValidationError`]
    /// listing every missing key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tempus_engine::metadata::RuleDefinition;
    ///
    /// let rule = RuleDefinition::new("loan", serde_json::json!(true))
    ///     .with_required_keys(vec!["score".into(), "income".into()]);
    ///
    /// let ctx_ok = serde_json::json!({"score": 720, "income": 50000});
    /// assert!(rule.validate_context(&ctx_ok).is_ok());
    ///
    /// let ctx_bad = serde_json::json!({"score": 720}); // missing "income"
    /// let err = rule.validate_context(&ctx_bad).unwrap_err();
    /// assert!(err.missing_keys.contains(&"income".to_string()));
    /// ```
    pub fn validate_context(&self, context: &Value) -> Result<(), ValidationError> {
        if self.required_context_keys.is_empty() {
            return Ok(());
        }
        let obj = match context.as_object() {
            Some(o) => o,
            None => {
                return Err(ValidationError::new(
                    &self.name,
                    self.required_context_keys.clone(),
                ))
            }
        };
        let missing: Vec<String> = self
            .required_context_keys
            .iter()
            .filter(|k| !obj.contains_key(k.as_str()))
            .cloned()
            .collect();
        if missing.is_empty() {
            Ok(())
        } else {
            Err(ValidationError::new(&self.name, missing))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn new_creates_minimal_definition() {
        let rule = RuleDefinition::new("test", json!({"==": [1, 1]}));
        assert_eq!(rule.name, "test");
        assert_eq!(rule.version, None);
        assert_eq!(rule.description, None);
        assert!(rule.tags.is_empty());
        assert!(rule.required_context_keys.is_empty());
    }

    #[test]
    fn builder_pattern_works() {
        let rule = RuleDefinition::new("fraud-block", json!({">":[{"var":"risk"},0.8]}))
            .with_version("2.1.0")
            .with_description("Block high-risk transactions")
            .with_tags(vec!["fraud".into(), "prod".into()]);

        assert_eq!(rule.name, "fraud-block");
        assert_eq!(rule.version.as_deref(), Some("2.1.0"));
        assert_eq!(
            rule.description.as_deref(),
            Some("Block high-risk transactions")
        );
        assert_eq!(rule.tags, vec!["fraud", "prod"]);
    }

    #[test]
    fn serialization_roundtrip() {
        let rule = RuleDefinition::new("test", json!({">":[{"var":"x"},1]})).with_version("1.0.0");

        let json_str = serde_json::to_string(&rule).unwrap();
        let deserialized: RuleDefinition = serde_json::from_str(&json_str).unwrap();
        assert_eq!(rule, deserialized);
    }

    #[test]
    fn minimal_serialization_omits_empty_fields() {
        let rule = RuleDefinition::new("test", json!({"==": [1, 1]}));
        let json_str = serde_json::to_string(&rule).unwrap();
        assert!(!json_str.contains("version"));
        assert!(!json_str.contains("description"));
        assert!(!json_str.contains("tags"));
    }

    #[test]
    fn validate_context_passes_when_all_keys_present() {
        let rule = RuleDefinition::new("r", json!({}))
            .with_required_keys(vec!["score".into(), "income".into()]);
        let ctx = json!({"score": 700, "income": 50000});
        assert!(rule.validate_context(&ctx).is_ok());
    }

    #[test]
    fn validate_context_fails_on_missing_key() {
        let rule = RuleDefinition::new("r", json!({}))
            .with_required_keys(vec!["score".into(), "income".into()]);
        let ctx = json!({"score": 700});
        let err = rule.validate_context(&ctx).unwrap_err();
        assert!(err.missing_keys.contains(&"income".to_string()));
    }

    #[test]
    fn validate_context_passes_with_no_required_keys() {
        let rule = RuleDefinition::new("r", json!({}));
        assert!(rule.validate_context(&json!({})).is_ok());
    }
}

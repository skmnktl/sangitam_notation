use crate::types::*;
use anyhow::Result;

pub fn validate(document: &VnaDocument) -> Result<Vec<ValidationIssue>> {
    let mut validator = VnaValidator::new();
    validator.validate(document)
}

struct VnaValidator {
    issues: Vec<ValidationIssue>,
}

impl VnaValidator {
    fn new() -> Self {
        Self {
            issues: Vec::new(),
        }
    }

    fn validate(&mut self, document: &VnaDocument) -> Result<Vec<ValidationIssue>> {
        // Validate metadata
        self.validate_metadata(&document.metadata);

        // Validate sections
        for section in &document.sections {
            self.validate_section(section);
        }

        Ok(self.issues.clone())
    }

    fn validate_metadata(&mut self, metadata: &Metadata) {
        // Check tempo range
        if let Some(tempo) = metadata.tempo {
            if tempo < 20 || tempo > 300 {
                self.add_warning(
                    1, // Approximate line number for metadata
                    format!("Unusual tempo: {} BPM (typical range: 20-300)", tempo),
                    Some("unusual_tempo".to_string())
                );
            }
        }

        // Check for empty required fields (already handled by parser, but double-check)
        if metadata.title.trim().is_empty() {
            self.add_error(1, "Title cannot be empty".to_string(), Some("empty_title".to_string()));
        }
        if metadata.raga.trim().is_empty() {
            self.add_error(1, "Raga cannot be empty".to_string(), Some("empty_raga".to_string()));
        }
        if metadata.tala.trim().is_empty() {
            self.add_error(1, "Tala cannot be empty".to_string(), Some("empty_tala".to_string()));
        }
    }

    fn validate_section(&mut self, section: &Section) {
        // Check section name
        if section.name.trim().is_empty() {
            self.add_error(
                section.line_number,
                "Section name cannot be empty".to_string(),
                Some("empty_section_name".to_string())
            );
        }

        // Check phrases
        for phrase in &section.phrases {
            self.validate_phrase(phrase);
        }
    }

    fn validate_phrase(&mut self, phrase: &Phrase) {
        // Check that all three lines have elements
        if phrase.swaras.is_empty() {
            self.add_error(
                phrase.line_number,
                "Swara line cannot be empty".to_string(),
                Some("empty_swara_line".to_string())
            );
        }
        
        if phrase.sahitya.is_empty() {
            self.add_error(
                phrase.line_number + 1,
                "Sahitya line cannot be empty".to_string(),
                Some("empty_sahitya_line".to_string())
            );
        }
        
        // Check line length consistency (warning, not error)
        let swara_count = phrase.swaras.len();
        let sahitya_count = phrase.sahitya.len();

        if swara_count != sahitya_count {
            self.add_warning(
                phrase.line_number + 1,
                format!(
                    "Line length mismatch: swara line has {} elements, sahitya line has {}",
                    swara_count, sahitya_count
                ),
                Some("line_length_mismatch".to_string())
            );
        }


        // Check for basic formatting issues in swaras
        for (i, swara) in phrase.swaras.iter().enumerate() {
            if swara.contains(char::is_lowercase) && swara.contains(char::is_uppercase) {
                self.add_warning(
                    phrase.line_number,
                    format!(
                        "Mixed case in swara '{}' at position {}",
                        swara, i + 1
                    ),
                    Some("mixed_case_swara".to_string())
                );
            }
        }
    }

    fn add_error(&mut self, line: usize, message: String, code: Option<String>) {
        self.issues.push(ValidationIssue {
            severity: Severity::Error,
            message,
            line,
            column: None,
            code,
            range: None,
        });
    }

    fn add_warning(&mut self, line: usize, message: String, code: Option<String>) {
        self.issues.push(ValidationIssue {
            severity: Severity::Warning,
            message,
            line,
            column: None,
            code,
            range: None,
        });
    }

    fn add_info(&mut self, line: usize, message: String, code: Option<String>) {
        self.issues.push(ValidationIssue {
            severity: Severity::Info,
            message,
            line,
            column: None,
            code,
            range: None,
        });
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn test_valid_document() {
        let content = r#"---
title: "Test Varnam"
raga: "mohanam"
tala: "adi"
tempo: 60
---

[pallavi]
G , G , | R , , , ||
nin - nu - | ko - - - ||
~ ~ ~ ~ | ~ ~ ~ ~ ||
"#;

        let doc = parse(content).unwrap();
        let issues = validate(&doc).unwrap();
        
        // Should have no errors, maybe some info messages
        let errors: Vec<_> = issues.iter().filter(|i| i.severity == Severity::Error).collect();
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_line_length_mismatch() {
        let content = r#"---
title: "Test"
raga: "mohanam"
tala: "adi"
---

[pallavi]
G , G , | R , , , ||
nin - nu ||
~ ~ ~ ~ | ~ ~ ~ ~ ||
"#;

        let doc = parse(content).unwrap();
        let issues = validate(&doc).unwrap();
        
        // Should have warnings about line length mismatch
        let warnings: Vec<_> = issues.iter().filter(|i| i.severity == Severity::Warning).collect();
        assert!(!warnings.is_empty());
    }

    #[test]
    fn test_invalid_merge_indicators() {
        let content = r#"---
title: "Test"
raga: "mohanam"
tala: "adi"
---

[pallavi]
G , G , ||
nin - nu - ||
x ~ y ~ ||
"#;

        let doc = parse(content).unwrap();
        let issues = validate(&doc).unwrap();
        
        // Should have errors about invalid merge indicators
        let errors: Vec<_> = issues.iter().filter(|i| i.severity == Severity::Error).collect();
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_extreme_tempo() {
        let content = r#"---
title: "Test"
raga: "mohanam"
tala: "adi"
tempo: 500
---

[pallavi]
G ||
nin ||
~ ||
"#;

        let doc = parse(content).unwrap();
        let issues = validate(&doc).unwrap();
        
        // Should have warning about unusual tempo
        let warnings: Vec<_> = issues.iter().filter(|i| i.severity == Severity::Warning).collect();
        assert!(!warnings.is_empty());
    }
}
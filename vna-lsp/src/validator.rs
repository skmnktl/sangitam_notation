use crate::types::*;
use crate::sahitya_parser::parse_sahitya_token_with_lang;
use anyhow::Result;

pub fn validate(document: &VnaDocument) -> Result<Vec<ValidationIssue>> {
    let mut validator = VnaValidator::new();
    validator.validate(document)
}

struct VnaValidator {
    issues: Vec<ValidationIssue>,
    language: Option<String>,
}

impl VnaValidator {
    fn new() -> Self {
        Self {
            issues: Vec::new(),
            language: None,
        }
    }

    fn validate(&mut self, document: &VnaDocument) -> Result<Vec<ValidationIssue>> {
        // Validate metadata and capture language
        self.validate_metadata(&document.metadata);
        self.language = document.metadata.language.clone();

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

        // Check gati value
        if let Some(gati) = metadata.gati {
            if !matches!(gati, 3 | 4 | 5 | 7 | 9) {
                self.add_warning(
                    1,
                    format!("Unusual gati value: {} (typical values: 3, 4, 5, 7, 9)", gati),
                    Some("unusual_gati".to_string())
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

        // Validate tala pattern
        self.validate_tala_pattern(&metadata.tala, 1);
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

        // Check section-level gati if present
        if let Some(gati) = section.gati {
            if !matches!(gati, 3 | 4 | 5 | 7 | 9) {
                self.add_warning(
                    section.line_number,
                    format!("Unusual gati value: {} (typical values: 3, 4, 5, 7, 9)", gati),
                    Some("unusual_gati".to_string())
                );
            }
        }

        // Check section-level tala if present
        if let Some(tala) = &section.tala {
            self.validate_tala_pattern(tala, section.line_number);
        }

        // Check phrases
        for phrase in &section.phrases {
            self.validate_phrase(phrase);
        }
    }

    fn validate_phrase(&mut self, phrase: &Phrase) {
        // Check line-level gati if present
        if let Some(gati) = phrase.gati {
            if !matches!(gati, 3 | 4 | 5 | 7 | 9) {
                self.add_warning(
                    phrase.line_number,
                    format!("Unusual gati value: {} (typical values: 3, 4, 5, 7, 9)", gati),
                    Some("unusual_gati".to_string())
                );
            }
        }

        // Check line-level tala if present
        if let Some(tala) = &phrase.tala {
            self.validate_tala_pattern(tala, phrase.line_number);
        }

        // Check that required lines have elements
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
        
        // Check token count consistency
        let swara_count = phrase.swaras.len();
        let sahitya_count = phrase.sahitya.len();

        if swara_count != sahitya_count {
            self.add_error(
                phrase.line_number + 1,
                format!(
                    "Token count mismatch: swara line has {} tokens, sahitya line has {}",
                    swara_count, sahitya_count
                ),
                Some("token_count_mismatch".to_string())
            );
            return; // Skip further checks if counts don't match
        }

        // Check token length matching (strict rule)
        for (i, (swara, sahitya)) in phrase.swaras.iter().zip(phrase.sahitya.iter()).enumerate() {
            // Parse token-level gati notation if present (e.g., SRG:3)
            let (swara_text, swara_gati) = if let Some(colon_pos) = swara.find(':') {
                let text = &swara[..colon_pos];
                let gati_str = &swara[colon_pos + 1..];
                if let Ok(gati) = gati_str.parse::<u8>() {
                    if !matches!(gati, 3 | 4 | 5 | 7 | 9) {
                        self.add_warning(
                            phrase.line_number,
                            format!("Unusual gati value in token '{}': {} (typical values: 3, 4, 5, 7, 9)", swara, gati),
                            Some("unusual_token_gati".to_string())
                        );
                    }
                    (text, Some(gati))
                } else {
                    self.add_error(
                        phrase.line_number,
                        format!("Invalid gati notation in token '{}': expected number after colon", swara),
                        Some("invalid_token_gati".to_string())
                    );
                    (text, None)
                }
            } else {
                (swara.as_str(), None)
            };

            // Parse swara and sahitya into units
            let swara_units = self.parse_swara_units(swara_text);
            let sahitya_units = parse_sahitya_token_with_lang(sahitya, self.language.as_deref());
            
            if swara_units.len() != sahitya_units.len() {
                self.add_error(
                    phrase.line_number + 1,
                    format!(
                        "Token unit mismatch at position {}: swara '{}' ({} units) vs sahitya '{}' ({} units)",
                        i + 1, swara_text, swara_units.len(), sahitya, sahitya_units.len()
                    ),
                    Some("token_unit_mismatch".to_string())
                );
            }
        }

        // Validate phrase analysis if present
        if let Some(analysis) = &phrase.phrase_analysis {
            // Basic validation: check for invalid characters
            for (i, ch) in analysis.chars().enumerate() {
                if !matches!(ch, '_' | '*' | '(' | ')' | ' ') {
                    self.add_warning(
                        phrase.line_number + 2,
                        format!(
                            "Invalid character '{}' in phrase analysis at position {}",
                            ch, i + 1
                        ),
                        Some("invalid_phrase_analysis".to_string())
                    );
                }
            }
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
    
    fn validate_tala_pattern(&mut self, pattern: &str, line: usize) {
        // Validate tala pattern format
        for (i, ch) in pattern.chars().enumerate() {
            match ch {
                '+' | '0' => {}, // Valid tala markers
                '2'..='9' => {}, // Valid finger counts
                _ => {
                    self.add_error(
                        line,
                        format!(
                            "Invalid character '{}' in tala pattern at position {}: valid characters are +, 0, and 2-9",
                            ch, i + 1
                        ),
                        Some("invalid_tala_pattern".to_string())
                    );
                }
            }
        }
        
        // Check for common tala patterns
        let known_patterns = vec![
            ("+234+0+0", "Adi"),
            ("0++234", "Rupaka"),
            ("+230+00", "Misra Chapu"),
            ("+23+0+0", "Triputa"),
            ("+0+0", "Khanda Chapu"),
            ("++++++++", "All claps"),
        ];
        
        let is_known = known_patterns.iter().any(|(p, _)| p == &pattern);
        if !is_known && !pattern.is_empty() {
            self.add_info(
                line,
                format!("Uncommon tala pattern '{}'. Common patterns include: {}", 
                    pattern,
                    known_patterns.iter()
                        .map(|(p, name)| format!("{} ({})", p, name))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                Some("uncommon_tala_pattern".to_string())
            );
        }
    }
    
    /// Parse swara token into individual units
    /// Each note counts as one unit, including octave markers
    fn parse_swara_units(&self, token: &str) -> Vec<String> {
        let mut units = Vec::new();
        let mut chars = token.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == ',' {
                // Comma is a sustain marker, counts as one unit
                units.push(",".to_string());
            } else if ch == '-' {
                // Dash is a rest marker, counts as one unit
                units.push("-".to_string());
            } else if ['S', 'R', 'G', 'M', 'P', 'D', 'N'].contains(&ch) {
                // Swara note
                let mut note = String::from(ch);
                
                // Check for variant (1, 2, 3)
                if let Some(&next_ch) = chars.peek() {
                    if ['1', '2', '3'].contains(&next_ch) {
                        note.push(chars.next().unwrap());
                    }
                }
                
                // Check for octave markers (., ')
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '.' || next_ch == '\'' {
                        note.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                
                units.push(note);
            }
        }
        
        units
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
tala: "+234+0+0"
tempo: 60
---

[pallavi]
G, G, | R,,, ||
ni- nn- | u-ko- ||
"#;

        let doc = parse(content).unwrap();
        let issues = validate(&doc).unwrap();
        
        // Print all issues for debugging
        for issue in &issues {
            println!("{:?}: {}", issue.severity, issue.message);
        }
        
        // Should have no errors, maybe some info messages
        let errors: Vec<_> = issues.iter().filter(|i| i.severity == Severity::Error).collect();
        assert_eq!(errors.len(), 0, "Got {} errors: {:?}", errors.len(), errors);
    }

    #[test]
    fn test_line_length_mismatch() {
        let content = r#"---
title: "Test"
raga: "mohanam"
tala: "+234+0+0"
---

[pallavi]
G,G, R,,, | SSRR ||
nin uko- | ri-- ||
"#;

        let doc = parse(content).unwrap();
        let issues = validate(&doc).unwrap();
        
        // Should have errors about unit count mismatch
        let errors: Vec<_> = issues.iter().filter(|i| i.severity == Severity::Error).collect();
        assert!(!errors.is_empty(), "Expected errors for unit mismatch, but got none. Issues: {:?}", issues);
    }

    #[test]
    fn test_invalid_phrase_analysis() {
        let content = r#"---
title: "Test"
raga: "mohanam"
tala: "+234+0+0"
---

[pallavi]
G , G , ||
nin - nu - ||
phrases = x ~ y ~
"#;

        let doc = parse(content).unwrap();
        let issues = validate(&doc).unwrap();
        
        // Should have warnings about invalid phrase analysis characters
        let warnings: Vec<_> = issues.iter().filter(|i| i.severity == Severity::Warning).collect();
        assert!(!warnings.is_empty());
    }

    #[test]
    fn test_extreme_tempo() {
        let content = r#"---
title: "Test"
raga: "mohanam"
tala: "+234+0+0"
tempo: 500
---

[pallavi]
G ||
nin ||
"#;

        let doc = parse(content).unwrap();
        let issues = validate(&doc).unwrap();
        
        // Should have warning about unusual tempo
        let warnings: Vec<_> = issues.iter().filter(|i| i.severity == Severity::Warning).collect();
        assert!(!warnings.is_empty());
    }
}
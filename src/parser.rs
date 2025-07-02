use crate::types::*;
use anyhow::{anyhow, Result};
use regex::Regex;
use serde_yaml;

pub fn parse(content: &str) -> Result<VnaDocument> {
    let mut parser = VnaParser::new(content);
    parser.parse()
}

struct VnaParser {
    content: String,
    lines: Vec<String>,
    current_line: usize,
}

impl VnaParser {
    fn new(content: &str) -> Self {
        let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        Self {
            content: content.to_string(),
            lines,
            current_line: 0,
        }
    }

    fn parse(&mut self) -> Result<VnaDocument> {
        let metadata = self.parse_metadata()?;
        let (sections, comments) = self.parse_body()?;

        Ok(VnaDocument {
            metadata,
            sections,
            comments,
        })
    }

    fn parse_metadata(&mut self) -> Result<Metadata> {
        // Look for YAML frontmatter
        if !self.current_line_starts_with("---") {
            return Err(anyhow!("Missing YAML frontmatter at start of file"));
        }

        self.advance_line(); // Skip opening ---
        let mut yaml_lines = Vec::new();
        
        while self.current_line < self.lines.len() {
            let line = &self.lines[self.current_line];
            if line.trim() == "---" {
                self.advance_line(); // Skip closing ---
                break;
            }
            yaml_lines.push(line.clone());
            self.advance_line();
        }

        if yaml_lines.is_empty() {
            return Err(anyhow!("Empty YAML frontmatter"));
        }

        let yaml_content = yaml_lines.join("\n");
        let metadata: Metadata = serde_yaml::from_str(&yaml_content)
            .map_err(|e| anyhow!("Invalid YAML metadata: {}", e))?;

        // Validate required fields
        if metadata.title.is_empty() {
            return Err(anyhow!("Missing required field: title"));
        }
        if metadata.raga.is_empty() {
            return Err(anyhow!("Missing required field: raga"));
        }
        if metadata.tala.is_empty() {
            return Err(anyhow!("Missing required field: tala"));
        }

        Ok(metadata)
    }

    fn parse_body(&mut self) -> Result<(Vec<Section>, Vec<Comment>)> {
        let mut sections = Vec::new();
        let mut comments = Vec::new();

        while self.current_line < self.lines.len() {
            let line = self.current_line_trimmed();
            
            if line.is_empty() {
                self.advance_line();
                continue;
            }

            if line.starts_with('#') {
                comments.push(Comment {
                    text: line[1..].trim().to_string(),
                    line_number: self.current_line + 1,
                    comment_type: CommentType::Line,
                });
                self.advance_line();
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                let section = self.parse_section()?;
                sections.push(section);
                continue;
            }

            return Err(anyhow!(
                "Unexpected content at line {}: {}",
                self.current_line + 1,
                line
            ));
        }

        Ok((sections, comments))
    }

    fn parse_section(&mut self) -> Result<Section> {
        let line = self.current_line_trimmed();
        let section_line = self.current_line;
        
        if !line.starts_with('[') || !line.ends_with(']') {
            return Err(anyhow!("Invalid section header at line {}", section_line + 1));
        }

        let name = line[1..line.len()-1].to_string();
        self.advance_line();

        let mut phrases = Vec::new();

        while self.current_line < self.lines.len() {
            let line = self.current_line_trimmed();
            
            // Empty line - continue
            if line.is_empty() {
                self.advance_line();
                continue;
            }

            // Comment - skip
            if line.starts_with('#') {
                self.advance_line();
                continue;
            }

            // New section - break
            if line.starts_with('[') && line.ends_with(']') {
                break;
            }

            // Notation line - parse phrase
            if line.contains('|') {
                let phrase = self.parse_phrase()?;
                phrases.push(phrase);
                continue;
            }

            return Err(anyhow!(
                "Unexpected content in section '{}' at line {}: {}",
                name,
                self.current_line + 1,
                line
            ));
        }

        Ok(Section {
            name,
            phrases,
            line_number: section_line + 1,
        })
    }

    fn parse_phrase(&mut self) -> Result<Phrase> {
        let phrase_start_line = self.current_line;

        // Expect exactly 2 lines: swara, sahitya
        if self.current_line + 1 >= self.lines.len() {
            return Err(anyhow!(
                "Incomplete phrase at line {} - need 2 lines (swara, sahitya)",
                phrase_start_line + 1
            ));
        }

        // Parse swara line
        let swara_line = self.current_line_trimmed();
        if !swara_line.contains('|') {
            return Err(anyhow!(
                "Invalid swara line at {}: missing beat markers",
                self.current_line + 1
            ));
        }
        let swaras = self.parse_notation_line(&swara_line)?;
        self.advance_line();

        // Parse sahitya line
        let sahitya_line = self.current_line_trimmed();
        if !sahitya_line.contains('|') {
            return Err(anyhow!(
                "Invalid sahitya line at {}: missing beat markers",
                self.current_line + 1
            ));
        }
        let sahitya = self.parse_notation_line(&sahitya_line)?;
        self.advance_line();

        Ok(Phrase {
            swaras,
            sahitya,
            line_number: phrase_start_line + 1,
        })
    }

    fn parse_notation_line(&self, line: &str) -> Result<Vec<String>> {
        // Remove || at end and split by |
        let clean_line = line.replace("||", "");
        let beats: Vec<&str> = clean_line.split('|').collect();
        
        let mut elements = Vec::new();
        
        for beat in beats {
            let beat_elements: Vec<&str> = beat.trim().split_whitespace().collect();
            for element in beat_elements {
                if !element.is_empty() {
                    elements.push(element.to_string());
                }
            }
        }

        Ok(elements)
    }

    fn current_line_trimmed(&self) -> String {
        if self.current_line < self.lines.len() {
            self.lines[self.current_line].trim().to_string()
        } else {
            String::new()
        }
    }

    fn current_line_starts_with(&self, prefix: &str) -> bool {
        if self.current_line < self.lines.len() {
            self.lines[self.current_line].trim().starts_with(prefix)
        } else {
            false
        }
    }

    fn advance_line(&mut self) {
        self.current_line += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_complete_file() {
        let content = r#"---
title: "Test Varnam"
raga: "mohanam"
tala: "adi"
tempo: 60
---

[pallavi]
G , G , | R , , , | S S R R ||
nin - nu - | ko - - - | ri - - - ||
~ ~ ~ ~ | ~ ~ ~ ~ | ~ ~ ~ ~ ||

[anupallavi]
P D S' D | P G R S ||
pa da sa da | pa ga ra sa ||
~ ~ ~ ~ | ~ ~ ~ ~ ||
"#;

        let result = parse(content);
        assert!(result.is_ok());
        
        let doc = result.unwrap();
        assert_eq!(doc.metadata.title, "Test Varnam");
        assert_eq!(doc.metadata.raga, "mohanam");
        assert_eq!(doc.sections.len(), 2);
        assert_eq!(doc.sections[0].name, "pallavi");
        assert_eq!(doc.sections[0].phrases.len(), 1);
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let content = r#"---
title: "Test"
invalid_yaml: [unclosed
---

[pallavi]
G ||
nin ||
~ ||
"#;
        
        let result = parse(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_required_field() {
        let content = r#"---
title: "Test"
# Missing raga and tala
---

[pallavi]
G ||
nin ||
~ ||
"#;
        
        let result = parse(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_incomplete_phrase() {
        let content = r#"---
title: "Test"
raga: "mohanam"
tala: "adi"
---

[pallavi]
G , G , | R , , , ||
nin - nu - | ko - - - ||
# Missing merge line
"#;
        
        let result = parse(content);
        assert!(result.is_err());
    }
}
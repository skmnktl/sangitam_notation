use crate::types::*;
use anyhow::{anyhow, Result};
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
        let mut pending_comments = Vec::new();
        let mut section_comments = Vec::new();
        let mut section_gati = None;
        let mut section_tala = None;

        while self.current_line < self.lines.len() {
            let line = self.current_line_trimmed();
            
            // Empty line - continue
            if line.is_empty() {
                self.advance_line();
                continue;
            }

            // Comment - collect
            if line.starts_with('#') {
                let comment = Comment {
                    text: line[1..].trim().to_string(),
                    line_number: self.current_line + 1,
                    comment_type: CommentType::Line,
                };
                pending_comments.push(comment);
                self.advance_line();
                continue;
            }

            // New section - break
            if line.starts_with('[') && line.ends_with(']') {
                // Any pending comments belong to the section
                section_comments.extend(pending_comments.drain(..));
                break;
            }

            // Gati annotation
            if line.starts_with("@gati:") {
                // Section-level gati override
                let gati_str = line[6..].trim();
                if let Ok(gati) = gati_str.parse::<u8>() {
                    section_gati = Some(gati);
                } else {
                    return Err(anyhow!("Invalid gati value at line {}: {}", self.current_line + 1, gati_str));
                }
                self.advance_line();
                continue;
            }

            // Tala annotation
            if line.starts_with("@tala:") {
                // Section-level tala override
                let tala_str = line[6..].trim().trim_matches('"');
                section_tala = Some(tala_str.to_string());
                self.advance_line();
                continue;
            }

            // Notation line - parse phrase
            if line.contains('|') {
                let mut phrase = self.parse_phrase()?;
                phrase.preceding_comments = pending_comments.drain(..).collect();
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

        // Any remaining comments belong to the section
        section_comments.extend(pending_comments);

        Ok(Section {
            name,
            phrases,
            line_number: section_line + 1,
            comments: section_comments,
            gati: section_gati,
            tala: section_tala,
        })
    }

    fn parse_phrase(&mut self) -> Result<Phrase> {
        let phrase_start_line = self.current_line;
        let mut line_gati = None;
        let mut line_tala = None;

        // Check for line-level annotations before the swara line
        while self.current_line < self.lines.len() {
            let line = self.current_line_trimmed();
            
            if line.starts_with("@gati:") {
                let gati_str = line[6..].trim();
                if let Ok(gati) = gati_str.parse::<u8>() {
                    line_gati = Some(gati);
                } else {
                    return Err(anyhow!("Invalid gati value at line {}: {}", self.current_line + 1, gati_str));
                }
                self.advance_line();
            } else if line.starts_with("@tala:") {
                let tala_str = line[6..].trim().trim_matches('"');
                line_tala = Some(tala_str.to_string());
                self.advance_line();
            } else {
                break; // Not an annotation, must be the swara line
            }
        }

        // Expect at least 2 lines: swara, sahitya
        if self.current_line + 1 >= self.lines.len() {
            return Err(anyhow!(
                "Incomplete phrase at line {} - need at least 2 lines (swara, sahitya)",
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
        let (swaras, swara_beats) = self.parse_notation_line_with_beats(&swara_line)?;
        self.advance_line();

        // Parse sahitya line
        let sahitya_line = self.current_line_trimmed();
        if !sahitya_line.contains('|') {
            return Err(anyhow!(
                "Invalid sahitya line at {}: missing beat markers",
                self.current_line + 1
            ));
        }
        let (sahitya, sahitya_beats) = self.parse_notation_line_with_beats(&sahitya_line)?;
        self.advance_line();

        // Check for optional phrase analysis line
        let mut phrase_analysis = None;
        if self.current_line < self.lines.len() {
            let next_line = self.current_line_trimmed();
            if next_line.starts_with("phrases = ") {
                phrase_analysis = Some(next_line[10..].to_string());
                self.advance_line();
            }
        }

        // Verify beat alignment
        if swara_beats != sahitya_beats {
            return Err(anyhow!(
                "Beat markers misaligned between swara and sahitya lines at line {}",
                phrase_start_line + 1
            ));
        }

        Ok(Phrase {
            swaras,
            sahitya,
            phrase_analysis,
            line_number: phrase_start_line + 1,
            preceding_comments: Vec::new(), // Will be filled by parse_section
            gati: line_gati,
            tala: line_tala,
            beat_positions: swara_beats,
        })
    }

    fn parse_notation_line(&self, line: &str) -> Result<Vec<String>> {
        let (elements, _) = self.parse_notation_line_with_beats(line)?;
        Ok(elements)
    }

    fn parse_notation_line_with_beats(&self, line: &str) -> Result<(Vec<String>, Vec<usize>)> {
        // Remove || at end
        let clean_line = if line.ends_with("||") {
            &line[..line.len() - 2]
        } else {
            line
        }.trim();
        
        let mut elements = Vec::new();
        let mut beat_positions = Vec::new();
        let mut current_pos = 0;
        
        // Split by | to get beats
        let beats: Vec<&str> = clean_line.split('|').collect();
        
        for (i, beat) in beats.iter().enumerate() {
            let beat_elements: Vec<&str> = beat.trim().split_whitespace().collect();
            for element in beat_elements {
                if !element.is_empty() {
                    elements.push(element.to_string());
                    current_pos += 1;
                }
            }
            
            // Record beat position after this beat (except for last beat)
            if i < beats.len() - 1 && current_pos > 0 {
                beat_positions.push(current_pos);
            }
        }

        Ok((elements, beat_positions))
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
tala: "+234+0+0"
tempo: 60
---

[pallavi]
G , G , | R , , , | S S R R ||
nin - nu - | ko - - - | ri - - - ||

[anupallavi]
P D S' D | P G R S ||
pa da sa da | pa ga ra sa ||
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
"#;
        
        let result = parse(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_incomplete_phrase() {
        let content = r#"---
title: "Test"
raga: "mohanam"
tala: "+234+0+0"
---

[pallavi]
G , G , | R , , , ||
# Missing sahitya line
"#;
        
        let result = parse(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_phrase_analysis() {
        let content = r#"---
title: "Test"
raga: "mohanam"
tala: "+234+0+0"
---

[pallavi]
G , G , | R , , , ||
nin - nu - | ko - - - ||
phrases = (_ *)* *   * *
"#;
        
        let result = parse(content);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.sections[0].phrases[0].phrase_analysis, Some("(_ *)* *   * *".to_string()));
    }
}
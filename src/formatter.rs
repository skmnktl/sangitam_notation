use crate::types::*;
use anyhow::Result;

pub fn format(document: &VnaDocument) -> Result<String> {
    let mut formatter = VnaFormatter::new();
    formatter.format(document)
}

struct VnaFormatter {
    output: String,
}

impl VnaFormatter {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    fn format(&mut self, document: &VnaDocument) -> Result<String> {
        // Format metadata
        self.format_metadata(&document.metadata)?;
        
        // Add blank line after metadata
        self.output.push('\n');

        // Format sections
        for (i, section) in document.sections.iter().enumerate() {
            if i > 0 {
                self.output.push('\n'); // Blank line between sections
            }
            self.format_section(section)?;
        }

        Ok(self.output.clone())
    }

    fn format_metadata(&mut self, metadata: &Metadata) -> Result<()> {
        self.output.push_str("---\n");
        self.output.push_str(&format!("title: \"{}\"\n", metadata.title));
        self.output.push_str(&format!("raga: \"{}\"\n", metadata.raga));
        self.output.push_str(&format!("tala: \"{}\"\n", metadata.tala));
        
        if let Some(tempo) = metadata.tempo {
            self.output.push_str(&format!("tempo: {}\n", tempo));
        }
        
        if let Some(composer) = &metadata.composer {
            self.output.push_str(&format!("composer: \"{}\"\n", composer));
        }
        
        if let Some(language) = &metadata.language {
            self.output.push_str(&format!("language: \"{}\"\n", language));
        }
        
        if let Some(key) = &metadata.key {
            self.output.push_str(&format!("key: \"{}\"\n", key));
        }
        
        self.output.push_str("---\n");
        Ok(())
    }

    fn format_section(&mut self, section: &Section) -> Result<()> {
        // Section header
        self.output.push_str(&format!("[{}]\n", section.name));

        // Format phrases
        for (i, phrase) in section.phrases.iter().enumerate() {
            if i > 0 {
                self.output.push('\n'); // Blank line between phrases
            }
            self.format_phrase(phrase)?;
        }

        Ok(())
    }

    fn format_phrase(&mut self, phrase: &Phrase) -> Result<()> {
        // Calculate the maximum length of elements in each position
        // This ensures proper alignment across both lines
        let max_len = phrase.swaras.len().max(phrase.sahitya.len());
        
        // Pad all lines to the same length for consistent formatting
        let mut swaras = phrase.swaras.clone();
        let mut sahitya = phrase.sahitya.clone();
        
        swaras.resize(max_len, "-".to_string());
        sahitya.resize(max_len, "-".to_string());

        // Calculate column widths for alignment
        let mut col_widths = Vec::new();
        for i in 0..max_len {
            let swara_width = swaras.get(i).map(|s| s.len()).unwrap_or(0);
            let sahitya_width = sahitya.get(i).map(|s| s.len()).unwrap_or(0);
            
            let max_width = swara_width.max(sahitya_width).max(1);
            col_widths.push(max_width);
        }

        // Format swara line
        self.format_notation_line(&swaras, &col_widths)?;
        
        // Format sahitya line
        self.format_notation_line(&sahitya, &col_widths)?;

        Ok(())
    }

    fn format_notation_line(&mut self, elements: &[String], col_widths: &[usize]) -> Result<()> {
        let mut line = String::new();
        
        for (i, element) in elements.iter().enumerate() {
            if i > 0 {
                line.push(' ');
            }
            
            // Left-align element in its column
            let width = col_widths.get(i).copied().unwrap_or(element.len());
            line.push_str(&format!("{:<width$}", element, width = width));
            
            // Add beat markers at appropriate positions
            // This is a simplified version - could be enhanced to detect actual beat boundaries
            if (i + 1) % 4 == 0 && i + 1 < elements.len() {
                line.push_str(" |");
            }
        }
        
        // Add final tala marker
        line.push_str(" ||");
        line.push('\n');
        
        self.output.push_str(&line);
        Ok(())
    }
}

// Alternative simpler formatter that preserves original beat structure
pub fn format_preserve_beats(document: &VnaDocument) -> Result<String> {
    let mut output = String::new();
    
    // Format metadata
    output.push_str("---\n");
    output.push_str(&format!("title: \"{}\"\n", document.metadata.title));
    output.push_str(&format!("raga: \"{}\"\n", document.metadata.raga));
    output.push_str(&format!("tala: \"{}\"\n", document.metadata.tala));
    
    if let Some(tempo) = document.metadata.tempo {
        output.push_str(&format!("tempo: {}\n", tempo));
    }
    if let Some(composer) = &document.metadata.composer {
        output.push_str(&format!("composer: \"{}\"\n", composer));
    }
    if let Some(language) = &document.metadata.language {
        output.push_str(&format!("language: \"{}\"\n", language));
    }
    if let Some(key) = &document.metadata.key {
        output.push_str(&format!("key: \"{}\"\n", key));
    }
    
    output.push_str("---\n\n");

    // Format sections
    for (i, section) in document.sections.iter().enumerate() {
        if i > 0 {
            output.push('\n');
        }
        
        output.push_str(&format!("[{}]\n", section.name));
        
        for phrase in &section.phrases {
            // Simple join with spaces - preserves original structure
            let swara_line = phrase.swaras.join(" ") + " ||";
            let sahitya_line = phrase.sahitya.join(" ") + " ||"; 
            
            output.push_str(&format!("{}\n", swara_line));
            output.push_str(&format!("{}\n", sahitya_line));
            output.push('\n');
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn test_format_roundtrip() {
        let original = r#"---
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

        let doc = parse(original).unwrap();
        let formatted = format(&doc).unwrap();
        
        // Parse the formatted version
        let doc2 = parse(&formatted).unwrap();
        
        // Should have same content
        assert_eq!(doc.metadata.title, doc2.metadata.title);
        assert_eq!(doc.sections.len(), doc2.sections.len());
    }

    #[test]
    fn test_format_preserve_beats() {
        let content = r#"---
title: "Test"
raga: "mohanam"
tala: "adi"
---

[pallavi]
G , G , | R , , , ||
nin - nu - | ko - - - ||
~ ~ ~ ~ | ~ ~ ~ ~ ||
"#;

        let doc = parse(content).unwrap();
        let formatted = format_preserve_beats(&doc).unwrap();
        
        // Should be valid when parsed back
        let result = parse(&formatted);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_alignment() {
        let content = r#"---
title: "Test"
raga: "mohanam"
tala: "adi"
---

[pallavi]
G , G , | R , , , ||
ninnukori - nu - | ko - - - ||
~ ~ ~ ~ | ~ ~ ~ ~ ||
"#;

        let doc = parse(content).unwrap();
        let formatted = format(&doc).unwrap();
        
        // Formatted version should have consistent spacing
        assert!(formatted.contains("||"));
        assert!(!formatted.contains("  ||")); // No double spaces before ||
    }
}
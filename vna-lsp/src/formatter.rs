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
        
        // Output document-level comments (like title comment)
        for comment in &document.comments {
            self.output.push_str(&format!("# {}\n", comment.text));
        }
        if !document.comments.is_empty() {
            self.output.push('\n');
        }

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
        
        if let Some(composer) = &metadata.composer {
            self.output.push_str(&format!("composer: \"{}\"\n", composer));
        }
        
        if let Some(language) = &metadata.language {
            self.output.push_str(&format!("language: \"{}\"\n", language));
        }
        
        if let Some(tempo) = metadata.tempo {
            self.output.push_str(&format!("tempo: {}\n", tempo));
        }
        
        if let Some(gati) = metadata.gati {
            self.output.push_str(&format!("gati: {}\n", gati));
        }
        
        if let Some(comp_type) = &metadata.composition_type {
            self.output.push_str(&format!("type: \"{}\"\n", comp_type));
        }
        
        if let Some(key) = &metadata.key {
            self.output.push_str(&format!("key: \"{}\"\n", key));
        }
        
        if let Some(default_octave) = &metadata.default_octave {
            self.output.push_str(&format!("default_octave: \"{}\"\n", default_octave));
        }
        
        if let Some(arohanam) = &metadata.arohanam {
            self.output.push_str(&format!("arohanam: \"{}\"\n", arohanam));
        }
        
        if let Some(avarohanam) = &metadata.avarohanam {
            self.output.push_str(&format!("avarohanam: \"{}\"\n", avarohanam));
        }
        
        self.output.push_str("---\n");
        Ok(())
    }

    fn format_section(&mut self, section: &Section) -> Result<()> {
        // Section header
        self.output.push_str(&format!("[{}]\n", section.name));
        
        // Output section-level gati if present
        if let Some(gati) = section.gati {
            self.output.push_str(&format!("@gati: {}\n", gati));
        }
        
        // Output any section-level comments
        for comment in &section.comments {
            self.output.push_str(&format!("# {}\n", comment.text));
        }

        // Format phrases
        for (i, phrase) in section.phrases.iter().enumerate() {
            // Output preceding comments for this phrase
            for comment in &phrase.preceding_comments {
                self.output.push_str(&format!("# {}\n", comment.text));
            }
            
            if i > 0 && phrase.preceding_comments.is_empty() {
                self.output.push('\n'); // Blank line between phrases
            }
            self.format_phrase(phrase)?;
        }

        Ok(())
    }

    fn format_phrase(&mut self, phrase: &Phrase) -> Result<()> {
        // Output line-level gati if present
        if let Some(gati) = phrase.gati {
            self.output.push_str(&format!("@gati: {}\n", gati));
        }
        
        // Don't pad - preserve original token structure
        let swaras = &phrase.swaras;
        let sahitya = &phrase.sahitya;
        
        // Calculate column widths for alignment
        let mut col_widths = Vec::new();
        for i in 0..swaras.len() {
            let swara_width = swaras.get(i).map(|s| s.chars().count()).unwrap_or(0);
            let sahitya_width = sahitya.get(i).map(|s| s.chars().count()).unwrap_or(0);
            
            let max_width = swara_width.max(sahitya_width);
            col_widths.push(max_width);
        }

        // Format swara line
        self.format_notation_line(swaras, &col_widths, &phrase.beat_positions)?;
        
        // Format sahitya line
        self.format_notation_line(sahitya, &col_widths, &phrase.beat_positions)?;
        
        // Add phrase analysis if present
        if let Some(analysis) = &phrase.phrase_analysis {
            self.output.push_str(&format!("phrases = {}\n", analysis));
        }

        Ok(())
    }

    fn format_notation_line(&mut self, elements: &[String], col_widths: &[usize], beat_positions: &[usize]) -> Result<()> {
        let mut formatted_elements = Vec::new();
        let mut current_pos = 0;
        let mut beat_idx = 0;
        
        // Format each element with proper width alignment
        for (i, element) in elements.iter().enumerate() {
            let width = col_widths.get(i).copied().unwrap_or(element.chars().count());
            let elem_chars = element.chars().count();
            let padding = width - elem_chars;
            let padded = format!("{}{}", element, " ".repeat(padding));
            formatted_elements.push(padded);
            current_pos += 1;
            
            // Check if we need to insert a beat marker
            if beat_idx < beat_positions.len() && current_pos == beat_positions[beat_idx] {
                formatted_elements.push("|".to_string());
                beat_idx += 1;
            }
        }
        
        // Join elements and add final phrase marker
        let line = formatted_elements.join(" ");
        self.output.push_str(&line);
        self.output.push_str(" ||\n");
        
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
tala: "+234+0+0"
tempo: 60
---

[pallavi]
G, G, | R,,, ||
ni nn | ukō- ||
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
tala: "+234+0+0"
---

[pallavi]
G, G, | R,,, ||
ni nn | ukō- ||
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
tala: "+234+0+0"
---

[pallavi]
G, G, | R,,, ||
ni nn | ukō- ||
"#;

        let doc = parse(content).unwrap();
        let formatted = format(&doc).unwrap();
        
        // Formatted version should have consistent spacing
        assert!(formatted.contains("||"));
        assert!(!formatted.contains("  ||")); // No double spaces before ||
    }
}
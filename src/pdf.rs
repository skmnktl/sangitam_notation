use crate::types::*;
use anyhow::{anyhow, Result};
use std::fs;
use std::process::Command;
use std::io::Write;

pub fn generate(document: &VnaDocument, grid_height: u32, page_size: &str) -> Result<Vec<u8>> {
    let mut generator = LatexPdfGenerator::new(grid_height, page_size)?;
    generator.generate(document)
}

struct LatexPdfGenerator {
    grid_height: u32,
    page_size: String,
}

impl LatexPdfGenerator {
    fn new(grid_height: u32, page_size: &str) -> Result<Self> {
        Ok(Self {
            grid_height,
            page_size: page_size.to_string(),
        })
    }

    fn generate(&mut self, document: &VnaDocument) -> Result<Vec<u8>> {
        // Create LaTeX content
        let latex_content = self.create_latex(document)?;
        
        // Write to temporary file
        let temp_dir = std::env::temp_dir();
        let tex_file = temp_dir.join("vna_temp.tex");
        let pdf_file = temp_dir.join("vna_temp.pdf");
        
        fs::write(&tex_file, latex_content)?;
        
        // Compile with pdflatex
        let output = Command::new("pdflatex")
            .arg("-output-directory")
            .arg(&temp_dir)
            .arg("-interaction=nonstopmode")
            .arg(&tex_file)
            .output();
            
        match output {
            Ok(result) => {
                if !result.status.success() {
                    return Err(anyhow!("pdflatex failed: {}", String::from_utf8_lossy(&result.stderr)));
                }
            }
            Err(e) => {
                return Err(anyhow!("Failed to run pdflatex (is it installed?): {}", e));
            }
        }
        
        // Read the generated PDF
        let pdf_bytes = fs::read(&pdf_file)?;
        
        // Clean up
        let _ = fs::remove_file(&tex_file);
        let _ = fs::remove_file(&pdf_file);
        let _ = fs::remove_file(temp_dir.join("vna_temp.aux"));
        let _ = fs::remove_file(temp_dir.join("vna_temp.log"));
        
        Ok(pdf_bytes)
    }

    fn create_latex(&self, document: &VnaDocument) -> Result<String> {
        let mut latex = String::new();
        
        // Document setup
        latex.push_str(&format!(r#"\documentclass[{}paper,11pt]{{article}}
\usepackage[utf8]{{inputenc}}
\usepackage[margin=0.5in]{{geometry}}
\usepackage{{tikz}}
\usepackage{{tabularx}}
\usetikzlibrary{{positioning}}

% Remove page numbers and headers
\pagestyle{{empty}}

% Custom commands for notation  
\newcommand{{\swara}}[1]{{\texttt{{#1}}}}
\newcommand{{\sahitya}}[1]{{\textit{{#1}}}}
\newcommand{{\merge}}[1]{{\texttt{{\footnotesize #1}}}}

% Staff lines command
\newcommand{{\stafflines}}{{%
\vspace{{0.3em}}
\noindent\begin{{tikzpicture}}[baseline=0pt]
% Note labels on the left - complete octave
\node[anchor=east] at (-0.3,0) {{\footnotesize S}};
\node[anchor=east] at (-0.3,0.5) {{\footnotesize R}};
\node[anchor=east] at (-0.3,1.0) {{\footnotesize G}};
\node[anchor=east] at (-0.3,1.5) {{\footnotesize M}};
\node[anchor=east] at (-0.3,2.0) {{\footnotesize P}};
\node[anchor=east] at (-0.3,2.5) {{\footnotesize D}};
\node[anchor=east] at (-0.3,3.0) {{\footnotesize N}};
% Staff lines - 7 lines for complete octave
\foreach \y in {{0,0.5,1.0,1.5,2.0,2.5,3.0}} {{
  \draw[darkgray, dotted, thick] (0,\y) -- (\textwidth,\y);
}}
\end{{tikzpicture}}
\vspace{{0.5em}}
}}

\setlength{{\parindent}}{{0pt}}
\setlength{{\parskip}}{{0pt}}

\begin{{document}}

"#, self.page_size));

        // Simple header without maketitle
        latex.push_str(&format!(r#"
{{\Large \textbf{{{}}}}}

\textit{{Raga: {} | Tala: {} | Composer: {} | Tempo: {} BPM}}

\vspace{{1em}}

"#, 
            self.escape_latex(&document.metadata.title),
            self.escape_latex(&document.metadata.raga),
            self.escape_latex(&document.metadata.tala),
            self.escape_latex(document.metadata.composer.as_deref().unwrap_or("Unknown")),
            document.metadata.tempo.unwrap_or(60)
        ));

        // Sections
        for section in &document.sections {
            latex.push_str(&format!(r#"
\textbf{{[{}]}}

"#, self.escape_latex(&section.name)));

            for phrase in &section.phrases {
                self.add_phrase_latex(&mut latex, phrase)?;
            }
            
            latex.push_str("\\vspace{0.5em}\n\n");
        }
        
        latex.push_str(r#"\end{document}"#);
        
        Ok(latex)
    }

    fn add_phrase_latex(&self, latex: &mut String, phrase: &Phrase) -> Result<()> {
        // Create aligned table with proper spacing
        let table_content = self.create_aligned_table(&phrase.swaras, &phrase.sahitya);
        
        latex.push_str(&table_content);
        
        // Staff lines for hand notation
        latex.push_str(r#"\stafflines

"#);
        
        Ok(())
    }

    fn create_aligned_table(&self, swaras: &[String], sahitya: &[String]) -> String {
        let mut latex = String::new();
        
        // Calculate number of beats (groups of 4)
        let max_len = swaras.len().max(sahitya.len());
        let num_beats = (max_len + 3) / 4;
        
        // Create table with fixed column widths across full page
        latex.push_str(r#"\noindent\begin{tabularx}{\textwidth}{@{}"#);
        
        // Create column specification - each beat gets equal space with separators
        for i in 0..num_beats {
            if i > 0 {
                latex.push_str("c"); // Center column for beat marker
            }
            latex.push_str("X"); // Expandable column for content
        }
        latex.push_str("r@{}}\n"); // Right align for final ||
        
        // Build swara row
        let mut beat_idx = 0;
        for beat_num in 0..num_beats {
            if beat_num > 0 {
                latex.push_str(" & \\texttt{|} & ");
            }
            
            // Collect elements for this beat
            let mut beat_elements = Vec::new();
            for _ in 0..4 {
                if beat_idx < swaras.len() {
                    beat_elements.push(format!("\\texttt{{{}}}", self.escape_latex(&swaras[beat_idx])));
                    beat_idx += 1;
                }
            }
            
            // Create evenly spaced content within the cell
            if beat_elements.is_empty() {
                latex.push_str("\\phantom{M}");
            } else {
                latex.push_str(&format!("\\makebox[\\linewidth][s]{{{}}}", beat_elements.join("\\hfill")));
            }
        }
        latex.push_str(" & \\texttt{||} \\\\\n");
        
        // Build sahitya row
        beat_idx = 0;
        for beat_num in 0..num_beats {
            if beat_num > 0 {
                latex.push_str(" & \\texttt{|} & ");
            }
            
            // Collect elements for this beat
            let mut beat_elements = Vec::new();
            for _ in 0..4 {
                if beat_idx < sahitya.len() {
                    beat_elements.push(format!("\\textit{{{}}}", self.escape_latex(&sahitya[beat_idx])));
                    beat_idx += 1;
                }
            }
            
            // Create evenly spaced content within the cell
            if beat_elements.is_empty() {
                latex.push_str("\\phantom{M}");
            } else {
                latex.push_str(&format!("\\makebox[\\linewidth][s]{{{}}}", beat_elements.join("\\hfill")));
            }
        }
        latex.push_str(" & \\texttt{||} \\\\\n");
        
        latex.push_str("\\end{tabularx}\n\n");
        
        latex
    }

    fn format_vna_line(&self, elements: &[String]) -> String {
        // Reconstruct the line with beat markers
        let mut result = String::new();
        
        for (i, element) in elements.iter().enumerate() {
            if i > 0 && i % 4 == 0 {
                result.push_str(" | ");
            } else if i > 0 {
                result.push(' ');
            }
            result.push_str(element);
        }
        
        // Add ending marker
        result.push_str(" ||");
        
        result
    }


    fn escape_latex(&self, text: &str) -> String {
        text.replace("&", "\\&")
            .replace("%", "\\%")
            .replace("$", "\\$")
            .replace("#", "\\#")
            .replace("^", "\\^{}")
            .replace("_", "\\_")
            .replace("{", "\\{")
            .replace("}", "\\}")
            .replace("\\", "\\textbackslash{}")
            // Don't escape ~ since we want it to display as ~
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn test_latex_generation() {
        let content = r#"---
title: "Test Composition"
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
        let generator = LatexPdfGenerator::new(40, "a4").unwrap();
        let latex = generator.create_latex(&doc).unwrap();
        
        assert!(latex.contains("\\documentclass"));
        assert!(latex.contains("Test Composition"));
        assert!(latex.contains("mohanam"));
    }
}
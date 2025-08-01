use serde::{Deserialize, Serialize};

/// Core VNA document structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VnaDocument {
    pub metadata: Metadata,
    pub sections: Vec<Section>,
    pub comments: Vec<Comment>,
}

/// YAML frontmatter metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub raga: String,
    pub tala: String,
    #[serde(rename = "type")]
    pub composition_type: Option<String>,
    pub tempo: Option<u32>,
    pub composer: Option<String>,
    pub language: Option<String>,
    pub key: Option<String>,
    pub gati: Option<u8>,
    pub default_octave: Option<String>,
    pub arohanam: Option<String>,
    pub avarohanam: Option<String>,
}

/// A section like [pallavi], [anupallavi]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    pub phrases: Vec<Phrase>,
    pub line_number: usize,
    pub comments: Vec<Comment>,
    pub gati: Option<u8>, // Section-level gati override
    pub tala: Option<String>, // Section-level tala pattern override
}

/// A two-line notation group with optional phrase analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Phrase {
    pub swaras: Vec<String>,
    pub sahitya: Vec<String>,
    pub phrase_analysis: Option<String>,
    pub line_number: usize,
    pub preceding_comments: Vec<Comment>,
    pub gati: Option<u8>, // Line-level gati override
    pub tala: Option<String>, // Line-level tala pattern override
    pub beat_positions: Vec<usize>, // Positions of | markers (not including final ||)
}

/// Comments and annotations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub text: String,
    pub line_number: usize,
    pub comment_type: CommentType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommentType {
    Line,
    Section,
    Performance,
}

/// Token with optional gati override
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub text: String,
    pub gati: Option<u8>, // Token-level gati override (e.g., SRG:3)
}

/// Validation issue for LSP diagnostics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub message: String,
    pub line: usize,
    pub column: Option<usize>,
    pub code: Option<String>,
    pub range: Option<Range>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

// No musical knowledge - this is purely a formatting tool
// Musicians know their music better than code





/// Parse result with location information
#[derive(Debug, Clone)]
pub struct ParseResult<T> {
    pub value: T,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl<T> ParseResult<T> {
    pub fn new(value: T, line: usize, column: usize, offset: usize) -> Self {
        Self {
            value,
            location: Location { line, column, offset },
        }
    }
}

/// LSP-specific types for editor integration
#[cfg(feature = "cli")]
pub mod lsp {
    use tower_lsp::lsp_types::*;

    /// Convert our ValidationIssue to LSP Diagnostic
    pub fn issue_to_diagnostic(issue: &super::ValidationIssue) -> Diagnostic {
        let severity = match issue.severity {
            super::Severity::Error => Some(DiagnosticSeverity::ERROR),
            super::Severity::Warning => Some(DiagnosticSeverity::WARNING),
            super::Severity::Info => Some(DiagnosticSeverity::INFORMATION),
        };

        let range = issue.range.as_ref().map(|r| Range {
            start: Position {
                line: r.start.line as u32,
                character: r.start.character as u32,
            },
            end: Position {
                line: r.end.line as u32,
                character: r.end.character as u32,
            },
        }).unwrap_or_else(|| Range {
            start: Position { line: issue.line as u32, character: 0 },
            end: Position { line: issue.line as u32, character: u32::MAX },
        });

        Diagnostic {
            range,
            severity,
            code: issue.code.as_ref().map(|c| NumberOrString::String(c.clone())),
            code_description: None,
            source: Some("vna".to_string()),
            message: issue.message.clone(),
            related_information: None,
            tags: None,
            data: None,
        }
    }
}
use crate::types::{VnaDocument, ValidationIssue, Severity};
use tower_lsp::lsp_types::*;

pub struct DiagnosticsProvider;

impl DiagnosticsProvider {
    pub fn new() -> Self {
        Self
    }

    pub fn provide_diagnostics(&self, document: &VnaDocument) -> Vec<Diagnostic> {
        match crate::validator::validate(document) {
            Ok(issues) => issues.into_iter().map(|issue| self.convert_issue(issue)).collect(),
            Err(_) => vec![],
        }
    }

    fn convert_issue(&self, issue: ValidationIssue) -> Diagnostic {
        let severity = match issue.severity {
            Severity::Error => DiagnosticSeverity::ERROR,
            Severity::Warning => DiagnosticSeverity::WARNING,
            Severity::Info => DiagnosticSeverity::INFORMATION,
        };

        let range = if let Some(range) = issue.range {
            Range {
                start: Position {
                    line: range.start.line as u32,
                    character: range.start.character as u32,
                },
                end: Position {
                    line: range.end.line as u32,
                    character: range.end.character as u32,
                },
            }
        } else {
            // Default range for line-level issues
            Range {
                start: Position {
                    line: (issue.line.saturating_sub(1)) as u32,
                    character: issue.column.unwrap_or(0) as u32,
                },
                end: Position {
                    line: (issue.line.saturating_sub(1)) as u32,
                    character: (issue.column.unwrap_or(0) + 10) as u32,
                },
            }
        };

        Diagnostic {
            range,
            severity: Some(severity),
            code: issue.code.map(|c| NumberOrString::String(c)),
            source: Some("vna".to_string()),
            message: issue.message,
            related_information: None,
            tags: None,
            code_description: None,
            data: None,
        }
    }
}
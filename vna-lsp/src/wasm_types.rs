use serde::{Deserialize, Serialize};

// Simplified types for WASM serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmVnaDocument {
    pub metadata: WasmMetadata,
    pub sections: Vec<WasmSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmMetadata {
    pub title: String,
    pub raga: String,
    pub tala: String,
    pub composer: Option<String>,
    pub language: Option<String>,
    pub tempo: Option<u32>,
    pub gati: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmSection {
    pub name: String,
    pub phrases: Vec<WasmPhrase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmPhrase {
    pub swaras: Vec<String>,
    pub sahitya: Vec<String>,
    pub phrase_analysis: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmValidationIssue {
    pub severity: String,
    pub message: String,
    pub line: usize,
}

// Conversion implementations
use crate::types::{VnaDocument, Section, Phrase, ValidationIssue, Severity};

impl From<&VnaDocument> for WasmVnaDocument {
    fn from(doc: &VnaDocument) -> Self {
        WasmVnaDocument {
            metadata: WasmMetadata {
                title: doc.metadata.title.clone(),
                raga: doc.metadata.raga.clone(),
                tala: doc.metadata.tala.clone(),
                composer: doc.metadata.composer.clone(),
                language: doc.metadata.language.clone(),
                tempo: doc.metadata.tempo,
                gati: doc.metadata.gati,
            },
            sections: doc.sections.iter().map(|s| s.into()).collect(),
        }
    }
}

impl From<&Section> for WasmSection {
    fn from(section: &Section) -> Self {
        WasmSection {
            name: section.name.clone(),
            phrases: section.phrases.iter().map(|p| p.into()).collect(),
        }
    }
}

impl From<&Phrase> for WasmPhrase {
    fn from(phrase: &Phrase) -> Self {
        WasmPhrase {
            swaras: phrase.swaras.clone(),
            sahitya: phrase.sahitya.clone(),
            phrase_analysis: phrase.phrase_analysis.clone(),
        }
    }
}

impl From<&ValidationIssue> for WasmValidationIssue {
    fn from(issue: &ValidationIssue) -> Self {
        WasmValidationIssue {
            severity: match issue.severity {
                Severity::Error => "error".to_string(),
                Severity::Warning => "warning".to_string(),
                Severity::Info => "info".to_string(),
            },
            message: issue.message.clone(),
            line: issue.line,
        }
    }
}
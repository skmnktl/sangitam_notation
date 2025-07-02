use crate::types::VnaDocument;
use tower_lsp::lsp_types::*;

pub fn create_document_symbols(document: &VnaDocument) -> Vec<DocumentSymbol> {
    let mut symbols = Vec::new();

    // Add metadata symbol
    symbols.push(DocumentSymbol {
        name: "Metadata".to_string(),
        detail: Some(format!("{} - {} - {}", 
            document.metadata.title,
            document.metadata.raga,
            document.metadata.tala
        )),
        kind: SymbolKind::NAMESPACE,
        tags: None,
        deprecated: None,
        range: Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 10, character: 0 }, // Approximate metadata range
        },
        selection_range: Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 3 },
        },
        children: None,
    });

    // Add section symbols
    for section in &document.sections {
        let mut children = Vec::new();
        
        // Add phrase symbols as children
        for (i, phrase) in section.phrases.iter().enumerate() {
            children.push(DocumentSymbol {
                name: format!("Phrase {}", i + 1),
                detail: Some(format!("{} elements", phrase.swaras.len())),
                kind: SymbolKind::FUNCTION,
                tags: None,
        deprecated: None,
                range: Range {
                    start: Position { 
                        line: (phrase.line_number - 1) as u32,
                        character: 0 
                    },
                    end: Position { 
                        line: (phrase.line_number + 2) as u32,
                        character: 0 
                    },
                },
                selection_range: Range {
                    start: Position { 
                        line: (phrase.line_number - 1) as u32,
                        character: 0 
                    },
                    end: Position { 
                        line: (phrase.line_number - 1) as u32,
                        character: 10 
                    },
                },
                children: None,
            });
        }

        symbols.push(DocumentSymbol {
            name: section.name.clone(),
            detail: Some(format!("{} phrases", section.phrases.len())),
            kind: SymbolKind::CLASS,
            tags: None,
        deprecated: None,
            range: Range {
                start: Position { 
                    line: (section.line_number - 1) as u32,
                    character: 0 
                },
                end: Position { 
                    line: if let Some(last_phrase) = section.phrases.last() {
                        (last_phrase.line_number + 3) as u32
                    } else {
                        (section.line_number + 1) as u32
                    },
                    character: 0 
                },
            },
            selection_range: Range {
                start: Position { 
                    line: (section.line_number - 1) as u32,
                    character: 0 
                },
                end: Position { 
                    line: (section.line_number - 1) as u32,
                    character: (section.name.len() + 2) as u32 
                },
            },
            children: if children.is_empty() { None } else { Some(children) },
        });
    }

    symbols
}

pub fn create_code_actions(_document: &VnaDocument, _range: &Range) -> CodeActionResponse {
    let mut actions = Vec::new();

    // Add format action
    actions.push(CodeActionOrCommand::CodeAction(CodeAction {
        title: "Format VNA Document".to_string(),
        kind: Some(CodeActionKind::SOURCE_FIX_ALL),
        diagnostics: None,
        edit: None, // Will be handled by formatting provider
        command: Some(Command {
            title: "Format".to_string(),
            command: "vna.format".to_string(),
            arguments: None,
        }),
        is_preferred: Some(true),
        disabled: None,
        data: None,
    }));

    // Add auto-fix for common issues
    actions.push(CodeActionOrCommand::CodeAction(CodeAction {
        title: "Add missing beat markers".to_string(),
        kind: Some(CodeActionKind::QUICKFIX),
        diagnostics: None,
        edit: None, // Would need line-specific logic
        command: Some(Command {
            title: "Fix Beat Markers".to_string(),
            command: "vna.fixBeatMarkers".to_string(),
            arguments: None,
        }),
        is_preferred: Some(false),
        disabled: None,
        data: None,
    }));

    CodeActionResponse::from(actions)
}

pub fn get_word_at_position(line: &str, character: u32) -> Option<String> {
    let chars: Vec<char> = line.chars().collect();
    let pos = character as usize;
    
    if pos >= chars.len() {
        return None;
    }

    // Find word boundaries
    let mut start = pos;
    let mut end = pos;

    // Move start backwards to find word beginning
    while start > 0 && chars[start - 1].is_alphanumeric() {
        start -= 1;
    }

    // Move end forwards to find word end
    while end < chars.len() && chars[end].is_alphanumeric() {
        end += 1;
    }

    if start < end {
        Some(chars[start..end].iter().collect())
    } else {
        None
    }
}
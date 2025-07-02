use crate::types::VnaDocument;
use tower_lsp::lsp_types::*;

pub fn provide_completions(_document: &VnaDocument, position: Position) -> Vec<CompletionItem> {
    let mut completions = Vec::new();

    // Section name completions
    completions.extend(create_section_completions());

    // YAML metadata completions
    if position.line < 10 {
        completions.extend(create_metadata_completions());
    }

    // Beat marker completions
    completions.extend(create_notation_completions());

    completions
}

fn create_section_completions() -> Vec<CompletionItem> {
    let sections = [
        "pallavi",
        "anupallavi", 
        "muktayisvaram",
        "charanam",
        "cittasvarams",
        "ragamalika",
        "madhyamakala",
        "drut",
    ];

    sections
        .iter()
        .map(|section| CompletionItem {
            label: format!("[{}]", section),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("VNA section".to_string()),
            documentation: Some(Documentation::String(format!(
                "Insert {} section header",
                section
            ))),
            insert_text: Some(format!("[{}]", section)),
            ..Default::default()
        })
        .collect()
}

fn create_metadata_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "YAML frontmatter".to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some("Complete YAML metadata block".to_string()),
            insert_text: Some(
                "---\ntitle: \"$1\"\nraga: \"$2\"\ntala: \"$3\"\ntempo: ${4:60}\ncomposer: \"$5\"\n---\n$0"
                    .to_string(),
            ),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "title".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("title: \"$1\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "raga".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("raga: \"$1\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "tala".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("tala: \"$1\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "tempo".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("tempo: ${1:60}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "composer".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("composer: \"$1\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
    ]
}

fn create_notation_completions() -> Vec<CompletionItem> {
    let mut completions = Vec::new();

    // Beat markers
    completions.extend(vec![
        CompletionItem {
            label: "||".to_string(),
            kind: Some(CompletionItemKind::OPERATOR),
            detail: Some("End of phrase marker".to_string()),
            insert_text: Some("||".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "|".to_string(),
            kind: Some(CompletionItemKind::OPERATOR),
            detail: Some("Beat boundary marker".to_string()),
            insert_text: Some("|".to_string()),
            ..Default::default()
        },
    ]);

    // Common swara patterns (without musical validation)
    let common_patterns = [
        ("S R G M", "Basic ascent"),
        ("G , G ,", "Repeated note with gaps"),
        ("- - - -", "Rest pattern"),
        (", , , ,", "Continuation pattern"),
    ];

    for (pattern, description) in common_patterns {
        completions.push(CompletionItem {
            label: pattern.to_string(),
            kind: Some(CompletionItemKind::TEXT),
            detail: Some(description.to_string()),
            insert_text: Some(pattern.to_string()),
            ..Default::default()
        });
    }

    // Merge indicators
    let merge_patterns = [
        ("~ ~ ~ ~", "Merge all notes in gamaka"),
        (". . . .", "Separate all notes"),
        ("- - - -", "Rest indicators"),
    ];

    for (pattern, description) in merge_patterns {
        completions.push(CompletionItem {
            label: pattern.to_string(),
            kind: Some(CompletionItemKind::CONSTANT),
            detail: Some(description.to_string()),
            insert_text: Some(pattern.to_string()),
            ..Default::default()
        });
    }

    completions
}
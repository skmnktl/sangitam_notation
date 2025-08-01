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
        "muktasvara",
        "charanam",
        "cittasvaras",
        "geetam",
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
                "---\ntitle: \"$1\"\nraga: \"$2\"\ntala: \"$3\"\ntempo: ${4:60}\ncomposer: \"$5\"\nlanguage: \"$6\"\n---\n$0"
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
        CompletionItem {
            label: "language".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("language: \"$1\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "type".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("type: \"$1\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "key".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("key: \"${1:C}\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "arohanam".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("arohanam: \"$1\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "avarohanam".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("avarohanam: \"$1\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "default_octave".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            insert_text: Some("default_octave: \"${1:middle}\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "gati".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some("Default gati/nadai for the composition".to_string()),
            insert_text: Some("gati: ${1|3,4,5,7,9|}".to_string()),
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
        ("S R G M P D N S'", "Basic ascent"),
        ("S' N D P M G R S", "Basic descent"),
        ("G , G ,", "Repeated note with gaps"),
        ("- - - -", "Rest pattern"),
        (", , , ,", "Sustain pattern"),
        ("S.", "Lower octave Sa"),
        ("S'", "Upper octave Sa"),
        ("S''", "Two octaves up"),
        ("SRGR", "Catusra gati (4 notes)"),
        ("SRG", "Tisra gati (3 notes)"),
        ("SRGMP", "Khanda gati (5 notes)"),
        ("SRGMPDN", "Misra gati (7 notes)"),
        ("SRGMPDNS'", "Sankirna gati (9 notes)"),
        ("SRG:3", "Token with tisra gati override"),
        ("SRGR:4", "Token with catusra gati override"),
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

    // Phrase analysis patterns
    completions.push(CompletionItem {
        label: "phrases =".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Phrase analysis line".to_string()),
        insert_text: Some("phrases = $1".to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    });

    // Gati annotation
    let annotations = [
        ("@gati:", "Gati override for section/line"),
    ];

    for (annotation, description) in annotations {
        completions.push(CompletionItem {
            label: annotation.to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some(description.to_string()),
            insert_text: Some(format!("{} $1", annotation)),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });
    }

    // Swara variants
    let variants = [
        ("R1", "Shuddha Rishabha"),
        ("R2", "Chatushruti Rishabha"),
        ("R3", "Shatshruti Rishabha"),
        ("G1", "Shuddha Gandhara"),
        ("G2", "Sadharana Gandhara"),
        ("G3", "Antara Gandhara"),
        ("M1", "Shuddha Madhyama"),
        ("M2", "Prati Madhyama"),
        ("D1", "Shuddha Dhaivata"),
        ("D2", "Chatushruti Dhaivata"),
        ("D3", "Shatshruti Dhaivata"),
        ("N1", "Shuddha Nishada"),
        ("N2", "Kaisiki Nishada"),
        ("N3", "Kakali Nishada"),
    ];

    for (variant, description) in variants {
        completions.push(CompletionItem {
            label: variant.to_string(),
            kind: Some(CompletionItemKind::VALUE),
            detail: Some(description.to_string()),
            insert_text: Some(variant.to_string()),
            ..Default::default()
        });
    }

    completions
}
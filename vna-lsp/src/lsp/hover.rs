use crate::types::VnaDocument;
use tower_lsp::lsp_types::*;

pub fn provide_hover(document: &VnaDocument, position: Position) -> Option<Hover> {
    // Simple hover implementation based on position
    // In a real implementation, we'd parse the line content to determine context
    
    let line_num = position.line as usize;
    
    // Check if we're in a section
    for section in &document.sections {
        if line_num == section.line_number - 1 {
            return Some(create_section_hover(&section.name));
        }
        
        // Check if we're in phrases of this section
        for phrase in &section.phrases {
            let phrase_start = phrase.line_number - 1;
            let phrase_lines = if phrase.phrase_analysis.is_some() { 3 } else { 2 };
            if line_num >= phrase_start && line_num < phrase_start + phrase_lines {
                return Some(create_phrase_hover(line_num - phrase_start, phrase.phrase_analysis.is_some()));
            }
        }
    }

    // Default hover for common symbols
    None
}

fn create_section_hover(section_name: &str) -> Hover {
    let content = match section_name {
        "pallavi" => "**Pallavi**: The main theme or refrain of the composition. Usually the most important melodic phrase that returns throughout the piece.",
        "anupallavi" => "**Anupallavi**: The second section that typically explores the middle octave and provides contrast to the pallavi.",
        "muktasvara" => "**Muktasvara**: A section of swara passages that showcases the melodic beauty of the raga without lyrics.",
        "charanam" => "**Charanam**: Verse section with different lyrics, usually exploring higher octaves and complex melodic patterns.",
        "cittasvaras" => "**Cittasvaras**: Intricate swara passages that highlight the technical and aesthetic aspects of the raga.",
        "geetam" => "**Geetam**: A simple musical form used for learning, with straightforward melodic patterns.",
        _ => "**Section**: A structural division of the composition with specific melodic and lyrical content.",
    };

    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content.to_string(),
        }),
        range: None,
    }
}

fn create_phrase_hover(line_type: usize, has_phrase_analysis: bool) -> Hover {
    let content = match line_type {
        0 => "**Swara Line**: Musical notes using S R G M P D N. \
               Octave markers: `.` (lower), `'` (upper), `''` (two up). \
               Variants: R1/R2/R3, G1/G2/G3, M1/M2, D1/D2/D3, N1/N2/N3.\n\
               Gati notation: `SRG:3` (tisra), `SRGR:4` (catusra), `SRGMP:5` (khanda)",
        1 => "**Sahitya Line**: Lyrics or syllables that align with the swaras above. \
               Use `-` for continuation or empty positions. \
               Each token must match the character count of the swara above.",
        2 if has_phrase_analysis => "**Phrase Analysis**: Optional phrasing patterns:\n\
               - `_` = Held/sustained notes\n\
               - `*` = Quick/crisp notes\n\
               - `()` = Phrase groupings\n\
               - `**` = Fast passages",
        _ => "**VNA Notation**: Two-line format (swara + sahitya) with optional phrase analysis",
    };

    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content.to_string(),
        }),
        range: None,
    }
}

pub fn create_symbol_hover(symbol: &str) -> Option<Hover> {
    let content = match symbol {
        "||" => "**Phrase End**: Marks the end of a complete musical phrase or line",
        "|" => "**Beat Boundary**: Indicates tala beat divisions within the phrase",
        "-" => "**Rest/Silence**: Indicates a pause or empty position in sahitya",
        "," => "**Sustain/Pause**: When standalone, sustains the previous note",
        "_" => "**Held Note**: In phrase analysis, indicates sustained notes",
        "*" => "**Quick Note**: In phrase analysis, indicates crisp notes",
        "S" => "**Shadja (Sa)**: The tonic note, foundation of the scale",
        "R" | "R1" | "R2" | "R3" => "**Rishabha (Ri)**: The second note with variants",
        "G" | "G1" | "G2" | "G3" => "**Gandhara (Ga)**: The third note with variants",
        "M" | "M1" | "M2" => "**Madhyama (Ma)**: The fourth note with variants",
        "P" => "**Panchama (Pa)**: The fifth note, perfect fifth",
        "D" | "D1" | "D2" | "D3" => "**Dhaivata (Dha)**: The sixth note with variants",
        "N" | "N1" | "N2" | "N3" => "**Nishada (Ni)**: The seventh note with variants",
        "@gati:" => "**Gati Override**: Sets the rhythmic subdivision for a section or line. Values: 3 (tisra), 4 (catusra), 5 (khanda), 7 (misra), 9 (sankirna)",
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content.to_string(),
        }),
        range: None,
    })
}
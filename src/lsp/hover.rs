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
            if line_num >= phrase_start && line_num < phrase_start + 3 {
                return Some(create_phrase_hover(line_num - phrase_start));
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
        "muktayisvaram" => "**Muktayisvaram**: A section of swara passages that showcases the melodic beauty of the raga without lyrics.",
        "charanam" => "**Charanam**: Verse section with different lyrics, usually exploring higher octaves and complex melodic patterns.",
        "cittasvarams" => "**Cittasvarams**: Intricate swara passages that highlight the technical and aesthetic aspects of the raga.",
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

fn create_phrase_hover(line_type: usize) -> Hover {
    let content = match line_type {
        0 => "**Swara Line**: Musical notes using the seven-note system (S R G M P D N). \
               May include octave indicators (', \") and ornament notations.",
        1 => "**Sahitya Line**: Lyrics or syllables that correspond to the swaras above. \
               Each syllable should align with the timing of the swara.",
        2 => "**Merge Line**: Indicates how notes flow together:\n\
               - `~` = Notes merge into continuous gamaka\n\
               - `.` = Notes are separate and distinct\n\
               - `-` = Rest or pause",
        _ => "**VNA Notation**: Three-line format for Carnatic music notation",
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
        "~" => "**Merge Indicator**: Notes flow together as a continuous gamaka (ornament)",
        "." => "**Separate Indicator**: Notes are played as distinct, separate sounds",
        "-" => "**Rest/Gap**: Indicates a pause or sustained previous note",
        "," => "**Continuation**: Sustains the previous note or indicates timing",
        "S" | "R" | "G" | "M" | "P" | "D" | "N" => {
            "**Swara**: One of the seven musical notes in Carnatic music"
        }
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
use shlesha::Shlesha;

/// Parse sahitya tokens into syllable units
/// Each dash counts as a separate unit
/// Backticks (`) mark explicit syllable boundaries
/// If backticks exist in token, ONLY split on backticks (ignore auto syllabification)
/// Examples:
///   "nin`nu" → ["nin", "nu"]
///   "yun`---" → ["yun", "-", "-", "-"]
///   "ninnu" → ["ni", "nnu"] (auto split based on script)
pub fn parse_sahitya_token(token: &str) -> Vec<String> {
    parse_sahitya_token_with_lang(token, None)
}

/// Parse sahitya tokens with optional language hint for better syllabification
pub fn parse_sahitya_token_with_lang(token: &str, language: Option<&str>) -> Vec<String> {
    // Check if token contains any backticks
    if token.contains('`') {
        // Manual mode: split ONLY on backticks, then handle dashes
        parse_with_backticks(token)
    } else {
        // Automatic mode: use shlesha for syllabification
        parse_automatic_with_shlesha(token, language)
    }
}

/// Parse token with manual backtick boundaries
fn parse_with_backticks(token: &str) -> Vec<String> {
    let mut units = Vec::new();
    let mut current = String::new();
    
    for ch in token.chars() {
        match ch {
            '`' => {
                // Backtick marks syllable boundary
                if !current.is_empty() {
                    // Process accumulated text for dashes
                    units.extend(split_on_dashes(&current));
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }
    
    // Process any remaining text
    if !current.is_empty() {
        units.extend(split_on_dashes(&current));
    }
    
    units
}

/// Split a segment on dashes
fn split_on_dashes(segment: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    
    for ch in segment.chars() {
        if ch == '-' {
            if !current.is_empty() {
                result.push(current.clone());
                current.clear();
            }
            result.push("-".to_string());
        } else {
            current.push(ch);
        }
    }
    
    if !current.is_empty() {
        result.push(current);
    }
    
    result
}

/// Parse token with automatic syllabification using shlesha
fn parse_automatic_with_shlesha(token: &str, language: Option<&str>) -> Vec<String> {
    let mut units = Vec::new();
    
    // Special case: if token is all dashes, just split each dash
    if token.chars().all(|c| c == '-') {
        return token.chars().map(|_| "-".to_string()).collect();
    }
    
    // Otherwise, handle mixed content
    let mut chars = token.chars().peekable();
    let mut current_segment = String::new();
    
    while let Some(ch) = chars.next() {
        if ch == '-' {
            // Process any accumulated segment
            if !current_segment.is_empty() {
                units.extend(syllabify_segment(&current_segment, language));
                current_segment.clear();
            }
            // Add the dash
            units.push("-".to_string());
        } else {
            current_segment.push(ch);
        }
    }
    
    // Process final segment if any
    if !current_segment.is_empty() {
        units.extend(syllabify_segment(&current_segment, language));
    }
    
    units
}

/// Syllabify a segment (no dashes) using shlesha
fn syllabify_segment(segment: &str, language: Option<&str>) -> Vec<String> {
    if segment.is_empty() {
        return vec![];
    }
    
    let transliterator = Shlesha::new();
    
    // Determine target script based on language
    let target_script = match language {
        Some("telugu") => "telugu",
        Some("tamil") => "tamil",
        Some("kannada") => "devanagari", // Kannada might not be supported yet
        Some("malayalam") => "devanagari", // Malayalam might not be supported yet
        Some("sanskrit") | Some("hindi") => "devanagari",
        _ => "devanagari", // Default to Devanagari
    };
    
    // Transliterate to the target script to get proper syllable boundaries
    match transliterator.transliterate(segment, "iso", target_script) {
        Ok(native_script) => {
            // Split by graphemes (which correspond to syllables in native scripts)
            use unicode_segmentation::UnicodeSegmentation;
            let graphemes: Vec<&str> = native_script.graphemes(true).collect();
            
            let mut syllables = Vec::new();
            
            // Transliterate each grapheme back to ISO
            for grapheme in graphemes {
                match transliterator.transliterate(grapheme, target_script, "iso") {
                    Ok(syllable) => syllables.push(syllable),
                    Err(_) => {
                        // If we can't transliterate back, fall back to simple parsing
                        return parse_vowel_based(segment);
                    }
                }
            }
            
            syllables
        }
        Err(_) => {
            // Fallback to simple vowel-based parsing if transliteration fails
            parse_vowel_based(segment)
        }
    }
}

/// Simple vowel-based syllabification
fn parse_vowel_based(segment: &str) -> Vec<String> {
    let mut syllables = Vec::new();
    let mut current = String::new();
    let mut chars = segment.chars().peekable();
    
    while let Some(ch) = chars.next() {
        current.push(ch);
        
        // Check if this character is a vowel
        if is_simple_vowel(ch) {
            // Look ahead to see if we should continue the syllable
            let mut should_end = true;
            
            if let Some(&next_ch) = chars.peek() {
                // If next is also a vowel (like 'aa', 'ii'), keep together
                if is_simple_vowel(next_ch) && could_be_long_vowel(ch, next_ch) {
                    should_end = false;
                }
                // If next is a consonant, we might want to include it
                else if !is_simple_vowel(next_ch) {
                    // Check further ahead
                    let mut temp_chars = chars.clone();
                    temp_chars.next(); // Skip the consonant
                    
                    if let Some(&after_cons) = temp_chars.peek() {
                        // If pattern is vowel-consonant-vowel, end here
                        if is_simple_vowel(after_cons) {
                            should_end = true;
                        } else {
                            // Pattern is vowel-consonant-consonant, include first consonant
                            should_end = false;
                        }
                    } else {
                        // End of string after consonant, include it
                        should_end = false;
                    }
                }
            }
            
            if should_end && !current.is_empty() {
                syllables.push(current.clone());
                current.clear();
            }
        }
    }
    
    // Add any remaining content
    if !current.is_empty() {
        syllables.push(current);
    }
    
    // If no syllables were created, just return the whole segment
    if syllables.is_empty() {
        vec![segment.to_string()]
    } else {
        syllables
    }
}

/// Check if character is a simple vowel
fn is_simple_vowel(ch: char) -> bool {
    matches!(ch, 'a' | 'ā' | 'i' | 'ī' | 'u' | 'ū' | 'e' | 'ē' | 'o' | 'ō' | 
                 'A' | 'I' | 'U' | 'E' | 'O')
}

/// Check if two vowels could form a long vowel
fn could_be_long_vowel(first: char, second: char) -> bool {
    matches!((first, second), 
        ('a', 'a') | ('a', 'ā') | ('ā', 'a') |
        ('i', 'i') | ('i', 'ī') | ('ī', 'i') |
        ('u', 'u') | ('u', 'ū') | ('ū', 'u') |
        ('e', 'e') | ('e', 'ē') | ('ē', 'e') |
        ('o', 'o') | ('o', 'ō') | ('ō', 'o')
    )
}

/// Fallback parsing when shlesha is not available or fails
fn parse_automatic_fallback(segment: &str) -> Vec<String> {
    // Simple fallback: just return the segment as a single unit
    vec![segment.to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_sahitya_token() {
        // Basic cases (automatic mode)
        assert_eq!(parse_sahitya_token("ri--"), vec!["ri", "-", "-"]);
        
        // "ninn" in Devanagari has graphemes ["नि", "न्न"] which map back to ["ni", "nn"]
        assert_eq!(parse_sahitya_token("ninn"), vec!["ni", "nn"]);
        
        // "uko" (short o) works correctly: ["ఉ", "కొ"] → ["u", "ko"]
        assert_eq!(parse_sahitya_token("uko-"), vec!["u", "ko", "-"]);
        
        // With shlesha v0.4.1, macron ō is now properly handled!
        // "ukō" → ["ఉ", "కో"] → ["u", "kō"]
        assert_eq!(parse_sahitya_token("ukō-"), vec!["u", "kō", "-"]);
        
        assert_eq!(parse_sahitya_token("----"), vec!["-", "-", "-", "-"]);
        assert_eq!(parse_sahitya_token("nā---"), vec!["nā", "-", "-", "-"]);
        
        // With explicit syllable boundaries (manual mode)
        assert_eq!(parse_sahitya_token("nin`nu"), vec!["nin", "nu"]);
        assert_eq!(parse_sahitya_token("ka`la"), vec!["ka", "la"]);
        assert_eq!(parse_sahitya_token("nin`nu-"), vec!["nin", "nu", "-"]);
        assert_eq!(parse_sahitya_token("yun`---"), vec!["yun", "-", "-", "-"]);
        assert_eq!(parse_sahitya_token("nā`---"), vec!["nā", "-", "-", "-"]);
        assert_eq!(parse_sahitya_token("nin`nu`ko`ri"), vec!["nin", "nu", "ko", "ri"]);
        
        // Mixed with dashes (automatic mode)
        // "nin" splits as ["ni", "n"] because final 'n' with halant is separate grapheme
        assert_eq!(parse_sahitya_token("nin-nu-"), vec!["ni", "n", "-", "nu", "-"]);
        
        // Automatic syllabification using shlesha
        assert_eq!(parse_sahitya_token("ni---"), vec!["ni", "-", "-", "-"]);
        assert_eq!(parse_sahitya_token("khi"), vec!["khi"]);
        
        // Shlesha correctly handles these complex cases!
        assert_eq!(parse_sahitya_token("ninnukori"), vec!["ni", "nnu", "ko", "ri"]);
        // "saṅgīta" → ["स", "ङ्गी", "त"] → ["sa", "ṅgī", "ta"]
        assert_eq!(parse_sahitya_token("saṅgīta"), vec!["sa", "ṅgī", "ta"]);
    }
}
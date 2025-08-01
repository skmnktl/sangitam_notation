use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use crate::{parse, validate, format};
use crate::wasm_types::{WasmVnaDocument, WasmValidationIssue};


#[wasm_bindgen(start)]
pub fn init() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
}

#[wasm_bindgen]
pub struct VnaParser;

#[wasm_bindgen]
impl VnaParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        VnaParser
    }

    /// Parse VNA content and return the document structure as JSON
    #[wasm_bindgen]
    pub fn parse(&self, content: &str) -> Result<JsValue, JsError> {
        match parse(content) {
            Ok(document) => {
                let wasm_doc: WasmVnaDocument = (&document).into();
                to_value(&wasm_doc).map_err(|e| JsError::new(&e.to_string()))
            }
            Err(e) => Err(JsError::new(&e.to_string())),
        }
    }

    /// Validate VNA content and return validation issues as JSON
    #[wasm_bindgen]
    pub fn validate(&self, content: &str) -> Result<JsValue, JsError> {
        match parse(content) {
            Ok(document) => {
                match validate(&document) {
                    Ok(issues) => {
                        let wasm_issues: Vec<WasmValidationIssue> = issues.iter().map(|i| i.into()).collect();
                        to_value(&wasm_issues).map_err(|e| JsError::new(&e.to_string()))
                    }
                    Err(e) => Err(JsError::new(&e.to_string())),
                }
            }
            Err(e) => Err(JsError::new(&e.to_string())),
        }
    }

    /// Format VNA content and return the formatted string
    #[wasm_bindgen]
    pub fn format(&self, content: &str) -> Result<String, JsError> {
        match parse(content) {
            Ok(document) => {
                format(&document).map_err(|e| JsError::new(&e.to_string()))
            }
            Err(e) => Err(JsError::new(&e.to_string())),
        }
    }

    /// Parse a single line and return token information
    #[wasm_bindgen]
    pub fn parse_line(&self, line: &str) -> Result<JsValue, JsError> {
        // Simple tokenization for a single line
        let tokens: Vec<String> = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        to_value(&tokens).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Get metadata fields
    #[wasm_bindgen]
    pub fn get_metadata_fields(&self) -> Result<JsValue, JsError> {
        let fields = vec![
            ("title", true),
            ("raga", true),
            ("tala", true),
            ("tempo", false),
            ("composer", false),
            ("language", false),
            ("type", false),
            ("key", false),
            ("gati", false),
            ("default_octave", false),
            ("arohanam", false),
            ("avarohanam", false),
        ];
        
        to_value(&fields).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Get valid section names
    #[wasm_bindgen]
    pub fn get_section_names(&self) -> Result<JsValue, JsError> {
        let sections = vec![
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
        
        to_value(&sections).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Get swara tokens
    #[wasm_bindgen]
    pub fn get_swara_tokens(&self) -> Result<JsValue, JsError> {
        let swaras = vec![
            "S", "R", "G", "M", "P", "D", "N",
            "R1", "R2", "R3",
            "G1", "G2", "G3",
            "M1", "M2",
            "D1", "D2", "D3",
            "N1", "N2", "N3",
        ];
        
        to_value(&swaras).map_err(|e| JsError::new(&e.to_string()))
    }
}
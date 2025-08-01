// Core modules that are always available
pub mod types;
pub mod parser;
pub mod validator;
pub mod formatter;
pub mod sahitya_parser;

// Re-export core functionality
pub use parser::parse;
pub use validator::validate;
pub use formatter::format;
pub use types::*;

// CLI-only modules
#[cfg(feature = "cli")]
pub mod pdf;
#[cfg(feature = "cli")]
pub mod lsp;

// WASM bindings
#[cfg(feature = "wasm")]
pub mod wasm;
#[cfg(feature = "wasm")]
pub mod wasm_types;
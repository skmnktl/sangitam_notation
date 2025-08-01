# VNA Language Server Protocol Implementation

## Overview
We're building a full LSP-compliant language server for `.vna` files that works with any LSP-compatible editor (VSCode, Neovim, Emacs, etc.).

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   VSCode/Vim    │◄──►│  VNA LSP Server  │◄──►│  VNA CLI Tool   │
│   (LSP Client)  │    │  (Rust binary)   │    │  (Same binary)  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                              │
                              ▼
                       ┌──────────────────┐
                       │  VNA Parser &    │
                       │  Validator Core  │
                       └──────────────────┘
```

## LSP Features We'll Implement

### ✅ Core Protocol Features
- **Text Synchronization**: Keep document state in sync
- **Diagnostics**: Real-time error/warning reporting
- **Hover**: Show information about swaras, ragas, talas
- **Completion**: Auto-complete swaras, section names, metadata

### 🎯 Notation-Specific Features
- **Semantic Highlighting**: Color-code swaras, octaves, merge patterns
- **Document Symbols**: Outline view of sections and phrases
- **Code Actions**: Quick fixes for formatting errors
- **Formatting**: Auto-format notation alignment
- **Completion**: Auto-complete section names, common patterns

### 🎵 Advanced Features
- **Document Symbols**: Navigate sections and phrases
- **Find References**: Find similar patterns across files
- **Rename**: Rename sections, update metadata  
- **Workspace Symbols**: Search across all .vna files
- **Document Links**: Link to related compositions

## LSP Communication

### Client → Server Messages
```json
{
  "jsonrpc": "2.0",
  "method": "textDocument/didChange",
  "params": {
    "textDocument": { "uri": "file:///path/to/ninnukori.vna" },
    "contentChanges": [...]
  }
}
```

### Server → Client Responses
```json
{
  "jsonrpc": "2.0",
  "method": "textDocument/publishDiagnostics",
  "params": {
    "uri": "file:///path/to/ninnukori.vna",
    "diagnostics": [
      {
        "range": { "start": { "line": 5, "character": 10 }, ... },
        "severity": 1,
        "message": "Invalid swara 'X' for raga Mohanam",
        "code": "invalid_swara"
      }
    ]
  }
}
```

## Implementation Structure

### Core LSP Server (`src/lsp/`)
```rust
// src/lsp/mod.rs
pub mod server;
pub mod handlers;
pub mod diagnostics;
pub mod completion;
pub mod hover;

// LSP server implementation using tower-lsp
use tower_lsp::{LspService, Server};
```

### VNA-Specific Logic (`src/`)
```rust
// Shared between CLI and LSP
mod types;      // VNA document types
mod parser;     // Parse .vna files
mod validator;  // Musical validation rules
mod formatter;  // Auto-formatting
mod raga;       // Raga definitions and rules
mod tala;       // Tala patterns and validation
```

## Editor Integration

### VSCode Extension
```json
{
  "name": "vna-language-support",
  "contributes": {
    "languages": [{
      "id": "vna",
      "extensions": [".vna"],
      "configuration": "./language-configuration.json"
    }],
    "grammars": [{
      "language": "vna",
      "scopeName": "source.vna",
      "path": "./syntaxes/vna.tmGrammar.json"
    }]
  }
}
```

### Language Configuration
```json
{
  "comments": {
    "lineComment": "#"
  },
  "brackets": [
    ["[", "]"],
    ["|", "|"]
  ],
  "autoClosingPairs": [
    ["[", "]"],
    ["|", "|"]
  ]
}
```

## Development Workflow

### 1. Single Binary, Multiple Modes
```bash
# CLI mode
vna lint *.vna
vna pdf ninnukori.vna

# LSP mode (started by editor)
vna lsp
```

### 2. Shared Core Logic
Both CLI and LSP use the same:
- Parser (VNA → AST)
- Validator (Musical rules)
- Formatter (Consistent style)

### 3. Editor Setup
```bash
# Install the tool
cargo install vna

# VSCode finds it automatically via PATH
# Or configure explicit path in settings
```

## Example LSP Capabilities

### Real-time Validation (Structure Only)
```vna
[pallavi]
G , G , | R , , , | S S R R ||
nin - nu - | ko - - - | ri - - - ||
~ ~ ~ ~ | ~ ~ ~ ~ | ~ ~ ~ ~ 
                         ^
                         Error: Missing || at line end
```

### Smart Completion (Structure Only)
```vna
[<TAB>]  →  Complete section brackets

---<TAB>  →  YAML frontmatter template

|<TAB>  →  Complete beat markers
```

### Hover Information
```vna
[pallavi]
^^^^^^^^
Hover: Section type - Main theme of the composition

~ ~ ~ ~
^
Hover: Merge indicator - Notes flow as continuous gamaka
```

## Standards Compliance

### LSP Specification
- Follows [LSP 3.17 specification](https://microsoft.github.io/language-server-protocol/)
- Uses JSON-RPC 2.0 for communication
- Implements standard methods: initialize, textDocument/*, workspace/*

### Editor Agnostic
Works with any LSP client:
- VSCode (official extension)
- Neovim (via nvim-lspconfig)
- Emacs (via lsp-mode)
- Vim (via vim-lsp)
- Sublime Text (via LSP package)

This gives us professional-grade tooling that integrates seamlessly with modern editors! 🎵
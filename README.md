# Sangita Annotation

A comprehensive toolset for Carnatic music notation using the Veena Notation Archive (VNA) format.

## Repository Structure

```
.
├── VNA_FORMAT_SPEC.md    # VNA format specification
├── vna-lsp/              # Linter, formatter, and LSP implementation
├── data/                 # Sample VNA files
├── editor/               # VSCode extension for VNA files
└── docs/                 # Additional documentation
```

## Components

### VNA Format Specification

The [VNA_FORMAT_SPEC.md](VNA_FORMAT_SPEC.md) defines the Veena Notation Archive format - a human-readable notation system for Carnatic music.

### VNA LSP (`vna-lsp/`)

A Rust-based Language Server Protocol implementation providing:
- **Linter**: Syntax validation and musical correctness checks
- **Formatter**: Consistent spacing and alignment
- **LSP Server**: Real-time editing support for VSCode
- **PDF Generator**: Export to staff notation with frequency grids

#### Building
```bash
cd vna-lsp
cargo build --release
```

#### Usage
```bash
# Validate a VNA file
./vna-lsp/target/release/vna validate data/ninnukori_mohanam.vna

# Format a VNA file
./vna-lsp/target/release/vna format data/ninnukori_mohanam.vna

# Start LSP server
./vna-lsp/target/release/vna lsp
```

### Sample Data (`data/`)

Example VNA files including:
- `ninnukori_mohanam.vna` - Ninnukori Varnam in Mohanam
- `sami_ninne_śrī.vna` - Sami Ninne in Śrī raga
- `vara_veena_mohanam.vna` - Vara Veena in Mohanam

### Editor Extension (`editor/`)

VSCode extension providing:
- Syntax highlighting for VNA files
- Integration with the VNA LSP server
- Real-time validation and formatting
- WASM-based parser for client-side features

## Quick Start

1. Build the LSP:
   ```bash
   cd vna-lsp && cargo build --release
   ```

2. Install the VSCode extension:
   ```bash
   cd editor && npm install && npm run compile
   ```

3. Open a `.vna` file in VSCode to start editing with full language support!

## License

[Add license information]
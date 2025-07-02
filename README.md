# VNA - Veena Notation Archive

A structured notation system for Carnatic music with professional-grade tooling including LSP support, PDF generation, and format validation.

## 🎵 What is VNA?

VNA (Veena Notation Archive) is a human-readable text format for Carnatic music notation that supports:
- Two-line notation (swaras, sahitya)
- Frequency-time grids for drawing gamakas
- Standard Carnatic music metadata
- Format-only validation (no musical constraints imposed)

## 🚀 Quick Start

### Installation
```bash
cargo install vna
```

### Basic Usage
```bash
# Lint your notation files
vna lint *.vna

# Format for consistent style
vna format *.vna

# Generate PDF with frequency grids
vna pdf composition.vna

# Show file information
vna info composition.vna

# Start LSP server (for editor integration)
vna lsp
```

## 📝 VNA Syntax Reference

### File Structure
Every VNA file has two parts:
1. **YAML Frontmatter** - Metadata about the composition
2. **Notation Sections** - Musical content in two-line groups

### Basic Example
```vna
---
title: "Ninnukori Varnam"
raga: "mohanam"
tala: "adi"
tempo: 60
composer: "Ramanatapuram Srinivasa Iyengar"
nadaka: 4
line_length: 12
---

[pallavi]
G , G , | R , , , | S S R R ||
nin - nu - | ko - - - | ri - - - ||
~ ~ ~ ~ | ~ ~ ~ ~ | ~ ~ ~ ~ ||
```

### YAML Frontmatter
| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `title` | ✅ | String | Composition title |
| `raga` | ✅ | String | Raga name |  
| `tala` | ✅ | String | Tala name |
| `tempo` | ❌ | Number | BPM (default: 60) |
| `composer` | ❌ | String | Composer name |
| `language` | ❌ | String | Lyrics language |
| `key` | ❌ | String | Starting pitch (default: C) |
| `nadaka` | ❌ | Number | Beats per unit (1=chaturasra, 3=tisra, 5=khanda, 7=misra, 9=sankeerna) |
| `line_length` | ❌ | Number | Max elements per line (default: 16) |

### Section Headers
```vna
[pallavi]          # Main theme
[anupallavi]       # Second section
[muktayisvaram]    # Swara improvisation
[charanam]         # Verse section  
[cittasvarams]     # Complex swaras
[custom_name]      # Any custom section
```

### Three-Line Notation Groups

Each musical phrase consists of exactly **3 lines**:

#### 1. Swara Line (Musical Notes)
```vna
G , G , | R , , , | S S R R ||
```
- **Basic swaras**: `S` `R` `G` `M` `P` `D` `N`
- **Octave markers**: 
  - `S..` = two octaves down (two dots after)
  - `S.` = lower octave (dot after)
  - `S` = middle octave (no marker)  
  - `S'` = upper octave (apostrophe after)
  - `S''` = two octaves up (two apostrophes after)
- **Timing**:
  - `,` = sustain/pause
  - `-` = rest/silence
  - `|` = beat division
  - `||` = phrase end

#### 2. Sahitya Line (Lyrics)
```vna
nin - nu - | ko - - - | ri - - - ||
```
- Any Unicode text for lyrics
- `-` for syllable continuation
- Must align with swara timing


### Comments
```vna
# Line comments
G , G , | R , , , ||  # End-of-line comments
```

## ⚡ CLI Commands

### `vna lint`
Validates file syntax and structure:
```bash
vna lint *.vna              # Lint all VNA files
vna lint --fix *.vna        # Auto-fix formatting issues
vna lint --watch *.vna      # Watch for changes (coming soon)
```

### `vna format` 
Formats files with consistent spacing:
```bash
vna format *.vna            # Format all files
vna format --check *.vna    # Check if formatted (CI mode)
```

### `vna pdf`
Generates PDFs with frequency grids:
```bash
vna pdf composition.vna                    # Basic PDF
vna pdf --output result.pdf composition.vna    # Custom output
vna pdf --grid-height 80 composition.vna       # Taller grids
vna pdf --page-size letter composition.vna     # US Letter size
```

### `vna validate`
Validates a single file:
```bash
vna validate composition.vna
```

### `vna info`
Shows composition metadata:
```bash
vna info composition.vna
```

## 🔍 Validation Rules

The VNA linter checks for **format and structure only** - no musical knowledge is imposed.

### ❌ Errors (Will fail linting)
- Missing required metadata (`title`, `raga`, `tala`)
- Empty section names
- Empty notation lines
- Malformed YAML frontmatter
- Missing beat markers (`|` and `||`)

### ⚠️ Warnings (Pass with warnings)
- Line length mismatches between swara and sahitya lines
- Unusual tempo values (< 20 or > 300 BPM)
- Mixed case in swaras (e.g., `Ga` instead of `G` or `ga`)

### ✅ What's NOT Validated
- Musical correctness (ragas, swara sequences)
- Tala adherence 
- Octave jump restrictions
- Language-specific rules

*This is intentional - VNA is a formatting tool, not a music theory enforcer.*

## 🛠️ Editor Integration

### LSP Server
VNA includes a Language Server Protocol implementation for real-time editor support:

- **Real-time validation** - Errors/warnings as you type
- **Auto-completion** - Section headers, metadata fields, notation patterns
- **Hover help** - Information about sections and notation elements
- **Document formatting** - Format-on-save support
- **Document outline** - Navigate sections and phrases

### Supported Editors
Any LSP-compatible editor:
- VSCode (extension coming soon)
- Neovim (via nvim-lspconfig)
- Emacs (via lsp-mode)
- Vim (via vim-lsp)
- Sublime Text (via LSP package)

### Manual LSP Setup
```json
{
  "command": "vna",
  "args": ["lsp"],
  "filetypes": ["vna"]
}
```

## 📄 PDF Generation

The PDF generator creates printable sheets with:

1. **Metadata header** - Title, raga, tala, composer
2. **Formatted notation** - Three-line groups with proper spacing
3. **Frequency grids** - For drawing gamakas/ornaments
4. **Plucking areas** - Space for fingering annotations

### Grid Behavior
- **Merged notes** (`~`): Single continuous grid for gamaka curves
- **Separate notes** (`.`): Individual grids for distinct ornaments
- **Configurable height**: Adjust grid size for different detail levels

## 🎯 Example Workflow

1. **Create** a new VNA file:
```vna
---
title: "My Composition"
raga: "shankarabharanam"
tala: "adi"
---

[pallavi]
S R G M | P D N S' ||
sa ri ga ma | pa da ni sa ||
~ ~ ~ ~ | ~ ~ ~ ~ ||
```

2. **Validate** the syntax:
```bash
vna lint my-composition.vna
```

3. **Format** for consistency:
```bash
vna format my-composition.vna
```

4. **Generate** practice PDF:
```bash
vna pdf my-composition.vna
```

5. **Edit** with LSP support in your favorite editor for real-time feedback

## 📚 Documentation

- [VNA Format Specification](VNA_FORMAT_SPEC.md) - Complete syntax reference
- [LSP Architecture](LSP_ARCHITECTURE.md) - Technical details of language server

## 🤝 Contributing

The VNA format prioritizes:
- **Simplicity** - Easy to write by hand
- **Flexibility** - No musical constraints imposed
- **Tooling** - Professional development experience
- **Printability** - Clean PDF output for practice

---

*VNA is designed for Carnatic musicians who want structured, tool-friendly notation without artificial constraints on their musical expression.* 🎶
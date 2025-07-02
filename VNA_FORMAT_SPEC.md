# Veena Notation Archive (.vna) Format Specification

## Overview
The `.vna` format is a structured, human-readable text format for Carnatic music notation specifically designed for veena players.

## File Structure

### Basic Format
```vna
---
title: "Ninnukori Varnam"
raga: "mohanam"
tala: "adi"
tempo: 60
composer: "Ramanatapuram Srinivasa Iyengar"
language: "telugu"
nadaka: 4
line_length: 12
---

[pallavi]
G   , G  , | R  , , , | S  S R R | G G R R ||
nin - nu - | ko - - - | ri - - - | - - - - ||

[anupallavi]
...
```

## Metadata Section
Uses YAML frontmatter format:

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `title` | Yes | String | Song title |
| `raga` | Yes | String | Raga name |
| `tala` | Yes | String | Tala name |
| `tempo` | No | Number | BPM (default: 60) |
| `composer` | No | String | Composer name |
| `language` | No | String | Sahitya language |
| `key` | No | String | Starting pitch (default: C) |
| `nadaka` | No | Number | Beats per unit (1=chaturasra, 3=tisra, 5=khanda, 7=misra, 9=sankeerna) |
| `line_length` | No | Number | Max elements per line (default: 16) |

## Section Structure

### Section Headers
```vna
[pallavi]          # Main theme
[anupallavi]       # Second section  
[muktayisvaram]    # Free-form swaras
[charanam]         # Verse section
[cittasvarams]     # Improvisation section
[custom_name]      # Any custom section
```

### Two-Line Notation Groups
Each musical phrase consists of exactly 2 lines:

1. **Swara Line**: Musical notes
2. **Sahitya Line**: Lyrics/syllables

```vna
G   , G  , | R  , , , | S  S R R ||    # Swara line
nin - nu - | ko - - - | ri - - - ||    # Sahitya line  
```

## Swara Line Syntax

### Basic Swaras
- `S` `R` `G` `M` `P` `D` `N` - Natural swaras
- `R1` `R2` `R3` - Ri variants (only when not natural to raga)
- `G1` `G2` `G3` - Ga variants
- `M1` `M2` - Ma variants  
- `D1` `D2` `D3` - Dha variants
- `N1` `N2` `N3` - Ni variants

### Octave Markers
- `S..` - Two octaves down (two dots after)
- `S.` - Lower octave (dot after)
- `S` - Middle octave (no marker)
- `S'` - Upper octave (apostrophe after)
- `S''` - Two octaves up (two apostrophes after)

### Timing and Rests
- `,` - Sustain/pause (when standalone)
- `-` - Rest/silence
- `|` - Beat division marker
- `||` - Tala cycle end

### Connected Notes
- Example: `S:R:G` = connected phrase

## Sahitya Line Syntax

### Syllables
- Any valid Unicode text
- `-` for syllable continuation
- `_` for extended holds (alternative to `-`)

### Alignment
Each syllable aligns with the swara position where it begins:
```vna
G   , G  , | R  , , , | S  S R R ||
nin - nu - | ko - - - | ri - - - ||
```


## Comments and Annotations

### Line Comments
```vna
# This is a comment
G , G , | R , , , ||  # End of line comment
```

### Section Comments
```vna
[pallavi]
# This section has a special gamaka on the third note
G , G , | R , , , ||
```

### Performance Notes
```vna
@tempo_change: andante
@dynamic: forte
@technique: use_chikari_strings
```

## Validation Rules

### Syntax Rules
1. Metadata must be valid YAML in frontmatter
2. Each notation group must have exactly 3 lines
3. All three lines must have matching beat structure
4. Tala markers (`|` and `||`) must align across lines
5. Section headers must be in `[brackets]`

### Musical Rules
1. Swaras must be valid for the specified raga (warning)
2. Octave jumps > 5 notes generate warnings
3. Tala cycle must match specified tala (error)
4. Tempo should be reasonable (20-200 BPM)

### Formatting Rules
1. Consistent spacing around beat markers
2. Aligned three-line groups
3. Proper Unicode handling for non-ASCII sahitya

## File Extensions
- `.vna` - Veena Notation Archive
- `.vna.md` - Markdown-compatible version with code blocks

## Example Complete File

```vna
---
title: "Ninnukori Varnam"
raga: "mohanam"
tala: "adi"
tempo: 60
composer: "Ramanatapuram Srinivasa Iyengar"
language: "telugu"
nadaka: 4
line_length: 12
---

# Pallavi section with traditional ornamentation
[pallavi]
G   , G  , | R  , , , | S  S R R | G G R R ||
nin - nu - | ko - - - | ri - - - | - - - - ||

G  P G G | R   S R G | R  R S D | , S  R G | R ||
ni - - - | khi - - - | la - - - | - lo - - | - ||

[anupallavi]
G   , G  , | P  , , , | G   G P P  | D  D P , ||
nan - nu - | pa - - - | lim - - pa | sa - - - ||

# Continue with remaining sections...
```

## LSP Features Supported
- Syntax highlighting
- Error detection and warnings
- Auto-completion for swaras and ragas
- Hover information for musical terms
- Format-on-save
- Real-time validation
- Snippet insertion for common patterns
# Veena Notation Archive (.vna) Format Specification

## Overview
Human-readable Carnatic notation. Format-only validation—no musical constraints imposed.

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
---

[pallavi]
G, G, R, ,, SS RR GG RR SR GR SR SD SR GP GR SR
nin nu kō - ri - - - yun - nā - nu - rā -
phrases = (_ *)* *   * *    * *    * * *_

GP GG RS RG RR SD SR GR GP GP DP DS DD PG DP GR
ni - -khi - -la - lō -ka - nā - -ya - kā
phrases = (_ **)** **    ** **    ** **    **)**

[anupallavi]
...
```

## Metadata Section
Uses YAML frontmatter format:

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `title` | Yes | String | Song title |
| `raga` | Yes | String | Raga name |
| `tala` | Yes | String | Tala pattern (e.g., "D3DUDU" for Adi) |
| `type` | No | String | Composition type (kriti, varnam, swarajati, etc.) |
| `tempo` | No | Number | BPM (default: 60) |
| `composer` | No | String | Composer name |
| `language` | No | String | Sahitya language |
| `key` | No | String | Starting pitch (default: C) |
| `gati` | No | Number | Default gati/nadai (default: 4) |


## Section Structure

### Section Headers
```vna
[pallavi]          # Main theme
[anupallavi]       # Second section  
[muktasvara]       # Free-form swaras
[charanam]         # Verse section
[cittasvaras]      # Improvisation section
[custom_name]      # Any custom section
```

### Two-Line Notation Groups
Each musical phrase consists of exactly 2 lines:

1. **Swara Line**: Musical notes
2. **Sahitya Line**: Lyrics/syllables

```vna
G, G, R, ,, SS RR GG RR ||    # Swara line
nin nu kō - ri - - - yun ||    # Sahitya line  
```

### Optional Phrase Analysis
```vna
phrases = (_ *)* *   * *    # Musical phrasing (optional)
```

## Swara Line Syntax

### Basic Swaras
- `S` `R` `G` `M` `P` `D` `N` - Natural swaras
- `R1` `R2` `R3` - Ri variants (when needed)
- `G1` `G2` `G3` - Ga variants
- `M1` `M2` - Ma variants  
- `D1` `D2` `D3` - Dha variants
- `N1` `N2` `N3` - Ni variants

### Octave Markers
- `S..` - Two octaves down
- `S.` - Lower octave
- `S` - Middle octave (default)
- `S'` - Upper octave
- `S''` - Two octaves up

### Timing and Rests
- `,` - Sustain/pause (when standalone)
- `-` - Rest/silence
- `|` - Beat division marker
- `||` - Phrase/cycle end

### Compound Tokens and Timing
When multiple notes are played within a single time unit (e.g., in faster passages or different gatis):
- `SSRR` - Four notes in one time unit (catusra gati - 4)
- `SRG` - Three notes in one time unit (tisra gati - 3)
- `SRGMP` - Five notes in one time unit (khanda gati - 5)
- `MPMGRGMP` - Seven notes in one time unit (misra gati - 7)
- `SRGMPMGRS` - Nine notes in one time unit (sankirna gati - 9)
- Spacing indicates timing: `S S R R` vs `SSRR` (spaced = normal timing, compound = compressed)

### Gati (Rhythmic Subdivision)
Gati defines how many subdivisions occur within each time unit. The default is 4 (catusra).

#### Gati Hierarchy
Gati can be specified at four levels, with each level overriding the previous:

1. **File-level (Global Default)**
   ```yaml
   ---
   gati: 4  # Default for entire composition
   ---
   ```

2. **Section-level Override**
   ```vna
   [muktasvara]
   @gati: 3  # This entire section uses tisra gati
   SRG MPD | GRS DPM | SGR MPD ||
   ```

3. **Line-level Override**
   ```vna
   [charanam]
   SRGR GRSR | PDSR SRGR ||  # Uses section/file default
   @gati: 3  # This line only uses tisra gati
   SRG MPD | GRS DPM | SGR MPD ||
   SRGR GRSR | PDSR SRGR ||  # Back to section/file default
   ```

4. **Token-level Override**
   ```vna
   # Section default is catusra (4)
   SRGR GRSR | SRG:3 MPD:3 | SRGR SRGMP:5 ||
   #             ^tisra ^tisra        ^khanda
   ```

#### Understanding Tokens vs Beats
- **Token**: A space-separated group of notes (e.g., `SRGR`, `SRG`, `S`)
- **Beat**: The musical time unit marked by `|` symbols
- Multiple tokens can exist within one beat, or one token can represent one beat

```vna
# Example: Different token arrangements
S R G R | P D S' R' ||  # 4 tokens per beat, each is single note
SRGR | PDSR ||          # 1 token per beat, each has 4 notes
SR GR | PD S'R' ||      # 2 tokens per beat, each has 2 notes
SRGR | S R G:3 | PDSR ||  # Mixed: 1, 3, and 1 tokens per beat
```

#### Documentation in Comments
For clarity, you can document the gati structure in comments:
```vna
# Gati pattern: || catusra | tisra | khanda | catusra ||
SRGR GRSR | SRG MPD | SRGMP DPMGR | GRSR SRGR ||
```

### Tala Pattern Notation

Tala patterns define the beat structure using traditional Carnatic notation:

#### Notation System
- `+` = Tali (clap)
- `[2-9]` = Finger counts  
- `0` = Khali/Visarjitam (wave)

#### Examples
- `+234+0+0` = Adi tala (clap + 3 fingers, clap-wave, clap-wave)
- `0++234` = Rupaka tala (wave-clap, clap + 3 fingers)
- `+230+00` = Misra Chapu (clap + 2 fingers, wave, clap-wave-wave)
- `+23+0+0` = Triputa tala (clap + 2 fingers, clap-wave, clap-wave)

#### Tala Hierarchy
Like gati, tala patterns can be specified at three levels:

1. **Composition-level (Global Default)**
   ```yaml
   ---
   tala: "+234+0+0"  # Adi tala pattern
   ---
   ```

2. **Section-level Override**
   ```vna
   [charanam]
   @tala: "+234+23+"  # Modified pattern for this section
   ```

3. **Line-level Override**
   ```vna
   @tala: "+++++++++"  # All claps for emphasis
   S R G M | P D | N S ||
   ```

The beat markers `|` and `||` in notation lines remain as visual guides but the actual beat structure is determined by the tala pattern.

### Token Matching Rule
**Each token in the sahitya line must have the same number of characters as the corresponding swara token:**
```vna
SRGR GRSR | S R G R | DPDS' ||
yun- na-- | va ra vee na | ---nā ||
```
- `yun-` (4 chars) matches `SRGR` (4 chars)  
- `na--` (4 chars) matches `GRSR` (4 chars)
- Single swaras match single syllables/dashes (1:1)
- `---nā` (5 chars) matches `DPDS'` (5 chars, counting the apostrophe)
- Use `-` to fill positions where no new syllable begins

**Note on octave markers**: Octave markers (`.`, `'`, `''`) count as part of the swara character count.

## Sahitya Line Syntax

### Syllables
- Any valid Unicode text
- `-` for syllable continuation or empty positions
- `` ` `` (backtick) for explicit syllable boundaries when needed

### Syllable Parsing Rules
1. Each `-` always counts as one unit (sustain marker)
2. Use `` ` `` to mark syllable boundaries within text
3. Token boundaries (spaces) mark the end automatically
4. Examples:
   - `nin`nu` → ["nin", "nu"] (2 units)
   - `yun`--- → ["yun", "-", "-", "-"] (4 units)
   - `nā`--- → ["nā", "-", "-", "-"] (4 units)
   - `nin`nu`ko`ri → ["nin", "nu", "ko", "ri"] (4 units)

**Best practice**: When using backticks in a token, mark all internal syllable boundaries for clarity.

### When to Use Syllable Markers
The parser attempts automatic syllabification, but may need help with:
- Compound words: `ninnukori` → `nin`nu`ko`ri` (4 syllables)
- Sanskrit compounds: `saṅgīta` → `saṅ`gī`ta` (3 syllables)
- Tamil/Telugu clusters: `kṣetra` → `kṣe`tra` (2 syllables)

Example:
```vna
G,G, R,,, | SSRR GGRR ||
nin`nu`kō`ri | ---- ---- ||  # Explicitly marked as 4 syllables
```

Without markers, the parser might incorrectly guess:
- `ninnukori` → ["ni", "nnu", "ko", "ri"] ❌
- `nin`nu`ko`ri` → ["nin", "nu", "ko", "ri"] ✓

### Unit Counting
Each token must have equal units in swara and sahitya:
```vna
SSRR  →  4 units (S, S, R, R)
ri--  →  3 units (ri, -, -)  ❌ Mismatch!
ri--- →  4 units (ri, -, -, -) ✓ Correct
```

### Alignment
Each syllable aligns with the swara position where it begins:
```vna
G, G, R, ,, SS RR GG RR ||
nin nu kō - ri - - - ||
```

## Phrase Analysis (Optional)

### Symbols
- `_` - Held/sustained notes
- `*` - Quick/crisp notes
- `()` - Phrase groupings
- `**` - Fast passages

### Example
```vna
phrases = (__ *)(* ** **   ** *****)(***** ** __)
```

## Comments and Annotations

### Line Comments
```vna
# This is a comment
G, G, R, ,, ||  # End of line comment
```

### Tala Pattern Documentation
```vna
# Adi tala structure with beat types
# tala: +234+0+0 (laghu + 2 drutams)
# Beat: + 2 3 4 | + 0 | + 0 ||
# || , , , , | , , | , , ||

# With explicit gati notation
# gati: catusra (4)
# || , , , , | , , | , , ||

# Tisra gati (3 subdivisions per beat)
# gati: tisra (3)
# || ,,, ,,, | ,,, | ,,, ||

# Mixed gati patterns
# || catusra(4) | tisra(3) | khanda(5) ||
```


## Validation Rules

### Syntax Rules
1. Metadata must be valid YAML in frontmatter
2. Each notation group must have exactly 2 lines
3. Beat markers (`|` and `||`) must align across lines
4. Section headers must be in `[brackets]`

### Format Validation (Errors)
1. Missing required metadata fields
2. Empty section names or notation lines
3. Malformed YAML frontmatter

### Style Warnings
1. Line length mismatches between swara and sahitya lines
2. Unusual tempo values (< 20 or > 300 BPM)
3. Mixed case in swaras

### No Musical Validation
- No raga correctness checks
- No tala adherence enforcement
- No octave jump restrictions
- Musicians know their music better than code

## File Extensions
- `.vna` - Veena Notation Archive
- `.vna.md` - Markdown-compatible version

## Example Complete File

```vna
---
title: "Ninnukori Varnam"
raga: "mohanam"
tala: "+234+0+0"
tempo: 60
composer: "Ramanatapuram Srinivasa Iyengar"
language: "telugu"
---

# Pallavi section with traditional ornamentation
[pallavi]
G, G, R, ,, SS RR GG RR SR GR SR SD SR GP GR SR
nin nu kō * ri * * *yun * nā * nu * rā *

GP GG RS RG RR SD SR GR GP GP DP DS DD PG DP GR
ni * *khi * *la * lō *ka * nā * *ya * kā

[anupallavi]
G, G, P, ,, GG PP DD P, DS DD PG DP DG DP GR SR
nan nu pā * lim * * pa sa * * ma *ya mu rā

# Continue with remaining sections...
```

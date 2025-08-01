# Tala Track Variations (Based on Option 5)

## Variation 5A: Minimal Beat Markers
```vna
[pallavi]
tala: | + + + + | + + | + + ||  # just beats, gati implied by note grouping
G,G, R,,, | SSRR GGRR | SRGR SRSD | SRGP GRSR ||
nin nu kō - | ri - - - | yun - nā - | nu - rā - ||
```

## Variation 5B: Beat + Subdivision Count
```vna
[pallavi]
tala: | 4 | 2 | 2 ||  # just counts, total 8 beats
G,G, R,,, | SSRR GGRR | SRGR SRSD | SRGP GRSR ||
nin nu kō - | ri - - - | yun - nā - | nu - rā - ||
```

## Variation 5C: Inline Gati Markers
```vna
[pallavi]
# Default gati: 4
G,G, R,,, | SSRR GGRR | SRGR SRSD | SRGP GRSR ||
nin nu kō - | ri - - - | yun - nā - | nu - rā - ||

# Gati change for specific beats
G,G, R,,, | SRG:3 MPD:3 | SRGR SRSD | SRGMP:5 - ||  # :3 = tisra, :5 = khanda
```

## Variation 5D: Gati as Prefix
```vna
[pallavi]
# Using prefixes: 4: = catusra, 3: = tisra, 5: = khanda
G,G, R,,, | 4:SSRR GGRR | 3:SRG MPD | 5:SRGMP DPMGR ||
```

## Variation 5E: Section-Level Tala Definition
```vna
[pallavi]
@tala: 4+2+2  # Total 8 beats in adi tala
@gati: 4      # Default catusra

# Exceptional gatis marked inline
G,G, R,,, | SSRR GGRR | 3:SRG MPD | SRGP GRSR ||  # 3: marks tisra for that segment
```

## Variation 5F: Beat Structure Line
```vna
[pallavi]
#beat: | 1 2 3 4 | 1 2 | 1 2 ||
G,G,   R,,,     | SSRR GGRR | SRGR SRSD | SRGP GRSR ||
nin    nu kō -  | ri - - -  | yun - nā - | nu - rā - ||
```

## Variation 5G: Compact Tala+Gati Header
```vna
[pallavi] <adi 8 = 4+2+2, gati: 4>
G,G, R,,, | SSRR GGRR | SRGR SRSD | SRGP GRSR ||
nin nu kō - | ri - - - | yun - nā - | nu - rā - ||

# For gati changes
[madhyamakala] <adi 8, gati: 3>  # Everything in tisra
```

## Variation 5H: Smart Defaults + Overrides
```vna
---
tala: adi  # Parser knows adi = 4+2+2
gati: 4    # Default for whole composition
---

[pallavi]
# Normal - uses defaults
G,G, R,,, | SSRR GGRR | SRGR SRSD | SRGP GRSR ||

# With override
@gati: 3 for line  # Entire line in tisra
SRG SRG | MPD MPD | SRG MPD | SRG SRG ||

# Or beat-specific
G,G, R,,, | SSRR GGRR | @3{SRG MPD} | SRGP GRSR ||  # Just this beat in tisra
```

## Variation 5I: Implicit from Grouping
```vna
[pallavi]
# Parser infers gati from note grouping
G, G, R, ,, | SS RR GG RR | SR GR SR SD | SR GP GR SR ||  # Spaces = catusra (4)
G,G, R,,, | SSRR GGRR | SRGR SRSD | SRGP GRSR ||          # No spaces = catusra (4)
G,G, R,,, | SRG MPD | SRGMP DPMGR | SRG SRG ||            # 3,3,5,3 = mixed gati
```

## For Linter/Formatter Implementation:

### Most Practical: Hybrid of 5E + 5H
```vna
---
tala: adi
default_gati: 4
---

[pallavi]
# Beat structure is known from tala (4+2+2)
# Default gati applies unless overridden

# Standard notation
G, G, R, ,, | S S R R G G R R | S R G R S R S D | S R G P G R S R ||

# Inline gati change (two ways)
G, G, R, ,, | 3:SRG MPD | S R G R S R S D | S R G P G R S R ||  # Method 1
G, G, R, ,, | SRG:3 MPD:3 | S R G R S R S D | S R G P G R S R ||  # Method 2

# Formatter rules:
# 1. Count notes per beat based on tala structure
# 2. Verify against expected gati
# 3. Flag mismatches as errors
# 4. Auto-space based on gati (optional)
```

## Questions for Implementation:

1. **Gati notation**: Prefix (`3:SRG`) or suffix (`SRG:3`)?
2. **Scope**: Beat-level, measure-level, or note-group level?
3. **Defaults**: Explicit everywhere or smart defaults?
4. **Validation**: Strict (must match) or flexible (just warn)?

Which variation feels most intuitive and practical to you?
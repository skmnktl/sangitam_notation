# Sahitya Gati/Rhythm Encoding Options

## The Problem Illustrated
```vna
SRGR GRSR | ...
yun- nā-- | ...  # Is this yu-n- nā-- or yun- nā--- or yu-n-nā--?

# We need to know:
# 1. How many time units each syllable takes
# 2. Where the syllable boundaries are
# 3. How to align with swara gati
```

## Option 1: Explicit Duration Markers
```vna
SRGR GRSR | DPDS' SRGP | ...
yu:2n:2 nā:4 | --:2nā:2- ---- | ...  # :n shows duration
```

## Option 2: Matching Token Length
```vna
# Make sahitya tokens match swara grouping exactly
SRGR GRSR | DPDS' SRGP | ...
yunn nā-- | --nā ---- | ...  # 4-char tokens to match SRGR
```

## Option 3: Compound Sahitya Tokens
```vna
# Group sahitya like swaras
SRGR GRSR | DPDS' SRGP | ...
yun- nā-- | --nā ---- | ...  # Each token is one unit
# Explicitly: (yu)(n-)/(nā)(--)
```

## Option 4: Beat Division Markers
```vna
# Use . to show beat divisions in sahitya
SRGR GRSR | DPDS' SRGP | ...
yu.n. nā.. | ..nā. .... | ...  # . shows subdivision
```

## Option 5: Parallel Notation
```vna
# Swara and sahitya with explicit beat markers
S.R.G.R. G.R.S.R. | D.P.D.S'. S.R.G.P. | ...
yu.n.-.nā. | -.-.nā.-. -.-.-.-. | ...
```

## Option 6: Spacing = Timing
```vna
# Single space = single unit
SRGR GRSR | DPDS' SRGP | ...
yu n - nā | - - nā - | - - - - | ...  # Spaces define units
```

## Option 7: Sahitya Gati Notation
```vna
# Add gati notation to sahitya when needed
SRGR GRSR | DPDS' SRGP | ...
yun-:4 nā--:4 | --nā-:5 | ...  # :n overrides default
```

## RECOMMENDED: Strict Token Matching

### Rule: Each token in sahitya = one rhythmic unit
```vna
[pallavi]
tala: | + + + + | + + | + + ||

# For ungrouped swaras (each swara = one unit)
S R G R | G R S R | ...
va ra vee na | mru du pa ni | ...  # Each syllable/dash = one unit

# For grouped swaras (compound tokens)
SRGR GRSR | DPDS' SRGP | ...
yun- nā-- | --nā ---- | ...  # Token count must match

# Mixed grouping
SR GR | SRGR | S R G R | ...
va ra | vee- | na mru du | ...
```

### Validation Rules:
1. Token count in sahitya must equal token count in swara line
2. Spaces indicate token boundaries
3. Compound tokens (like SRGR) require matching sahitya tokens
4. Use - for continuation within a token

### Examples with Different Gatis:
```vna
# Catusra (4)
SSRR GGPP | ...
vara ---- | ...  # 'vara' sung across SSRR

# Tisra (3) 
SRG MPD | ...
va- ra- | ...  # Each syllable gets equal time

# Mixed
SRGR GR SR | ...  # 4 + 2 + 2
yun- na mā | ...  # Matching: 4 + 2 + 2
```

### Complex Example:
```vna
[cittasvaram]
# Some swaras grouped, some not
SRGR G R PMDP S' | ...
taka ta ki tana ta | ...  # 4 + 1 + 1 + 4 + 1

# Formatter validates:
# ✓ SRGR (4) matches taka (4)
# ✓ G (1) matches ta (1)
# ✓ R (1) matches ki (1)
# ✓ PMDP (4) matches tana (4)
# ✓ S' (1) matches ta (1)
```

This approach:
1. Is visually clear
2. Enforces rhythmic accuracy
3. Works with the linter/formatter
4. Handles all gati combinations

What do you think?
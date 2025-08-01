# Sahitya Grouping Options for VNA Format

## The Problem
```vna
# Swara grouping doesn't match sahitya grouping
D P D S' | ...
- - nā - | ...  # Where does "nā" align? With D or P or both?
```

## Option 1: Compound Sahitya Tokens (Matching Swara Style)
```vna
[pallavi]
tala: | + + + + | + + | + + ||
SRGR GRSR | DPDS' SRGP | ...
yun- nā-- | --nā- ---- | ...  # Group syllables like swaras
```

## Option 2: Explicit Alignment Markers
```vna
[pallavi]
SRGR GRSR | DPDS' SRGP | ...
yun- nā-- | ^^nā^ ---- | ...  # ^ shows syllable spans multiple swaras
# or
yun- nā-- | ..nā. ---- | ...  # . shows continuation
```

## Option 3: Bracket Notation
```vna
[pallavi]
SRGR GRSR | DPDS' SRGP | ...
yun- nā-- | [-nā-] --- | ...  # Brackets show grouping
```

## Option 4: Underscore Continuation
```vna
[pallavi]
SRGR GRSR | DPDS' SRGP | ...
yun- nā-- | __nā_ ---- | ...  # _ prefix shows continuation
```

## Option 5: Smart Spacing (Visual Alignment)
```vna
[pallavi]
SRGR GRSR | DPDS' SRGP | ...
yun- nā-- | - nā - --- | ...  # Formatter aligns based on context
```

## Option 6: Explicit Position Markers
```vna
[pallavi]
SRGR GRSR | DPDS' SRGP | ...
yun- nā-- | @2nā@ ---- | ...  # @2 = starts at position 2
```

## Option 7: Compound with Internal Markers
```vna
[pallavi]
# Using ~ to show where syllable actually sounds
SRGR GRSR | DPDS' SRGP | ...
yun- nā-- | -~nā~ ---- | ...  # ~ marks syllable position within group
```

## Option 8: Alignment Track
```vna
[pallavi]
tala: | + + + + | + + | + + ||
SRGR GRSR | DPDS' SRGP | ...
yun- nā-- | --nā- ---- | ...
align: xxxx | .xx. | ...  # x = new syllable, . = continuation
```

## Recommended Approach: Keep It Simple

### For Most Cases (1:1 alignment):
```vna
S R G R | D P D S' | ...
yu n- nā -- | ra - - - | ...
```

### For Grouped Swaras with Single Syllable:
```vna
SRGR GRSR | DPDS' SRGP | ...
yun- nā-- | --nā- ---- | ...  # Position syllable where it starts
```

### Complex Example:
```vna
[madhyamakala]
tala: | + + + + | + + | + + ||
# When swara grouping and sahitya grouping differ
SRGR GRSR | DPDS' SRGP | MGRS RGMP ||
vara vee- | --naa mru- | dupa niva ||

# The rule: Place syllable at its starting position
# Use - for continuation/empty positions
# Let context determine the actual pronunciation span
```

## Linter Rules:
1. Syllable must start at a valid swara position
2. Number of non-dash elements should make musical sense
3. Warning if syllable alignment seems incorrect
4. Formatter can adjust spacing for visual clarity

What do you think? Should we keep it simple and rely on position, or do we need explicit grouping notation?
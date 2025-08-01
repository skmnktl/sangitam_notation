import { VnaParser } from './parser';

describe('VnaParser', () => {
  let parser: VnaParser;

  beforeEach(() => {
    parser = new VnaParser();
  });

  test('parses basic VNA document', () => {
    const content = `---
title: "Test Varnam"
raga: "mohanam"
tala: "adi"
tempo: 60
---

[pallavi]
G,G, R,,, | SSRR GGRR ||
ninn ukō- | ri-- ---- ||`;

    const doc = parser.parse(content);
    
    expect(doc.metadata.title).toBe('Test Varnam');
    expect(doc.metadata.raga).toBe('mohanam');
    expect(doc.metadata.tala).toBe('adi');
    expect(doc.metadata.tempo).toBe(60);
    
    expect(doc.sections).toHaveLength(1);
    expect(doc.sections[0].name).toBe('pallavi');
    expect(doc.sections[0].phrases).toHaveLength(1);
    
    const phrase = doc.sections[0].phrases[0];
    expect(phrase.swaras).toEqual(['G,G,', 'R,,,', 'SSRR', 'GGRR']);
    expect(phrase.sahitya).toEqual(['ninn', 'ukō-', 'ri--', '----']);
    expect(phrase.beat_positions).toEqual([2]);
  });

  test('validates token length matching', () => {
    const content = `---
title: "Test"
raga: "mohanam"
tala: "adi"
---

[pallavi]
G,G, R,,, ||
ninn uk ||`;

    const issues = parser.validate(content);
    
    expect(issues).toHaveLength(1);
    expect(issues[0].severity).toBe('error');
    expect(issues[0].message).toContain('Token length mismatch');
  });

  test('parses gati at different levels', () => {
    const content = `---
title: "Test"
raga: "mohanam"
tala: "adi"
gati: 4
---

[pallavi]
@gati: 3
SRG MPD ||
--- --- ||

[anupallavi]
@gati: 5
SRGMP DPMGR ||
----- ----- ||`;

    const doc = parser.parse(content);
    
    expect(doc.metadata.gati).toBe(4);
    expect(doc.sections[0].gati).toBe(3);
    expect(doc.sections[1].name).toBe('anupallavi');
  });

  test('preserves beat markers', () => {
    const content = `---
title: "Test"
raga: "mohanam"
tala: "adi"
---

[pallavi]
G,G, R,,, | SSRR GGRR | SRGR SRSD. ||
ninn ukō- | ri-- ---- | yun- nānu- ||`;

    const doc = parser.parse(content);
    const phrase = doc.sections[0].phrases[0];
    
    expect(phrase.beat_positions).toEqual([2, 4]);
    expect(phrase.swaras).toHaveLength(6);
  });

  test('handles phrase analysis', () => {
    const content = `---
title: "Test"
raga: "mohanam"
tala: "adi"
---

[pallavi]
G,G, R,,, ||
ninn ukō- ||
phrases = (_ _ _) _`;

    const doc = parser.parse(content);
    const phrase = doc.sections[0].phrases[0];
    
    expect(phrase.phrase_analysis).toBe('(_ _ _) _');
  });
});
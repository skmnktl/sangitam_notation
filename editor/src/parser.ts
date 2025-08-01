/**
 * VNA Parser for TypeScript/JavaScript
 * This parser is designed to work in the editor environment
 */

export interface Metadata {
  title: string;
  raga: string;
  tala: string;
  tempo?: number;
  composer?: string;
  language?: string;
  type?: string;
  key?: string;
  gati?: number;
  default_octave?: string;
  arohanam?: string;
  avarohanam?: string;
}

export interface Phrase {
  swaras: string[];
  sahitya: string[];
  phrase_analysis?: string;
  line_number: number;
  beat_positions: number[];
  gati?: number;
}

export interface Section {
  name: string;
  phrases: Phrase[];
  line_number: number;
  gati?: number;
}

export interface VnaDocument {
  metadata: Metadata;
  sections: Section[];
}

export interface ValidationIssue {
  severity: 'error' | 'warning' | 'info';
  message: string;
  line: number;
  column?: number;
}

export class VnaParser {
  private lines: string[];
  private currentLine: number;

  constructor() {
    this.lines = [];
    this.currentLine = 0;
  }

  parse(content: string): VnaDocument {
    this.lines = content.split('\n');
    this.currentLine = 0;

    const metadata = this.parseMetadata();
    const sections = this.parseSections();

    return { metadata, sections };
  }

  private parseMetadata(): Metadata {
    if (!this.currentLineStartsWith('---')) {
      throw new Error('Missing YAML frontmatter');
    }

    this.advanceLine(); // Skip opening ---
    const yamlLines: string[] = [];

    while (this.currentLine < this.lines.length) {
      const line = this.lines[this.currentLine];
      if (line.trim() === '---') {
        this.advanceLine();
        break;
      }
      yamlLines.push(line);
      this.advanceLine();
    }

    // Simple YAML parsing (for basic key-value pairs)
    const metadata: any = {};
    for (const line of yamlLines) {
      const match = line.match(/^(\w+):\s*"?([^"]*)"?$/);
      if (match) {
        const [, key, value] = match;
        if (key === 'tempo' || key === 'gati') {
          metadata[key] = parseInt(value, 10);
        } else {
          metadata[key] = value;
        }
      }
    }

    // Validate required fields
    if (!metadata.title || !metadata.raga || !metadata.tala) {
      throw new Error('Missing required metadata fields');
    }

    return metadata as Metadata;
  }

  private parseSections(): Section[] {
    const sections: Section[] = [];

    while (this.currentLine < this.lines.length) {
      const line = this.currentLineTrimmed();

      if (line.startsWith('[') && line.endsWith(']')) {
        const section = this.parseSection();
        sections.push(section);
      } else if (line.trim() !== '' && !line.startsWith('#')) {
        throw new Error(`Unexpected content at line ${this.currentLine + 1}`);
      } else {
        this.advanceLine();
      }
    }

    return sections;
  }

  private parseSection(): Section {
    const line = this.currentLineTrimmed();
    const sectionLine = this.currentLine;

    const name = line.slice(1, -1);
    this.advanceLine();

    const phrases: Phrase[] = [];
    let sectionGati: number | undefined;

    while (this.currentLine < this.lines.length) {
      const line = this.currentLineTrimmed();

      if (line === '') {
        this.advanceLine();
        continue;
      }

      if (line.startsWith('#')) {
        this.advanceLine();
        continue;
      }

      if (line.startsWith('@gati:')) {
        sectionGati = parseInt(line.substring(6).trim(), 10);
        this.advanceLine();
        continue;
      }

      if (line.startsWith('[') && line.endsWith(']')) {
        break;
      }

      if (line.includes('|')) {
        const phrase = this.parsePhrase();
        phrases.push(phrase);
      } else {
        throw new Error(`Unexpected content in section at line ${this.currentLine + 1}`);
      }
    }

    return {
      name,
      phrases,
      line_number: sectionLine + 1,
      gati: sectionGati,
    };
  }

  private parsePhrase(): Phrase {
    const phraseStartLine = this.currentLine;
    let lineGati: number | undefined;

    // Check for line-level gati
    if (this.currentLineTrimmed().startsWith('@gati:')) {
      lineGati = parseInt(this.currentLineTrimmed().substring(6).trim(), 10);
      this.advanceLine();
    }

    // Parse swara line
    const swaraLine = this.currentLineTrimmed();
    const { tokens: swaras, beatPositions: swaraBeatPositions } = this.parseNotationLine(swaraLine);
    this.advanceLine();

    // Parse sahitya line
    const sahityaLine = this.currentLineTrimmed();
    const { tokens: sahitya, beatPositions: sahityaBeatPositions } = this.parseNotationLine(sahityaLine);
    this.advanceLine();

    // Verify beat alignment
    if (JSON.stringify(swaraBeatPositions) !== JSON.stringify(sahityaBeatPositions)) {
      throw new Error(`Beat markers misaligned at line ${phraseStartLine + 1}`);
    }

    // Check for phrase analysis
    let phraseAnalysis: string | undefined;
    if (this.currentLineTrimmed().startsWith('phrases = ')) {
      phraseAnalysis = this.currentLineTrimmed().substring(10);
      this.advanceLine();
    }

    return {
      swaras,
      sahitya,
      phrase_analysis: phraseAnalysis,
      line_number: phraseStartLine + 1,
      beat_positions: swaraBeatPositions,
      gati: lineGati,
    };
  }

  private parseNotationLine(line: string): { tokens: string[]; beatPositions: number[] } {
    // Remove || at end
    const cleanLine = line.endsWith('||') ? line.slice(0, -2).trim() : line.trim();

    const tokens: string[] = [];
    const beatPositions: number[] = [];
    let currentPos = 0;

    // Split by | to get beats
    const beats = cleanLine.split('|');

    beats.forEach((beat, i) => {
      const beatTokens = beat.trim().split(/\s+/).filter(t => t !== '');
      beatTokens.forEach(token => {
        tokens.push(token);
        currentPos++;
      });

      // Record beat position after this beat (except for last beat)
      if (i < beats.length - 1 && currentPos > 0) {
        beatPositions.push(currentPos);
      }
    });

    return { tokens, beatPositions };
  }

  private currentLineTrimmed(): string {
    return this.currentLine < this.lines.length ? this.lines[this.currentLine].trim() : '';
  }

  private currentLineStartsWith(prefix: string): boolean {
    return this.currentLineTrimmed().startsWith(prefix);
  }

  private advanceLine(): void {
    this.currentLine++;
  }

  // Validation
  validate(content: string): ValidationIssue[] {
    const issues: ValidationIssue[] = [];

    try {
      const document = this.parse(content);
      
      // Validate metadata
      if (document.metadata.tempo && (document.metadata.tempo < 20 || document.metadata.tempo > 300)) {
        issues.push({
          severity: 'warning',
          message: `Unusual tempo: ${document.metadata.tempo} BPM (typical range: 20-300)`,
          line: 1,
        });
      }

      if (document.metadata.gati && ![3, 4, 5, 7, 9].includes(document.metadata.gati)) {
        issues.push({
          severity: 'warning',
          message: `Unusual gati value: ${document.metadata.gati} (typical values: 3, 4, 5, 7, 9)`,
          line: 1,
        });
      }

      // Validate phrases
      for (const section of document.sections) {
        for (const phrase of section.phrases) {
          // Check token count
          if (phrase.swaras.length !== phrase.sahitya.length) {
            issues.push({
              severity: 'error',
              message: `Token count mismatch: ${phrase.swaras.length} swaras vs ${phrase.sahitya.length} sahitya`,
              line: phrase.line_number,
            });
            continue;
          }

          // Check token length matching
          for (let i = 0; i < phrase.swaras.length; i++) {
            const swaraLen = phrase.swaras[i].length;
            const sahityaLen = phrase.sahitya[i].length;
            if (swaraLen !== sahityaLen) {
              issues.push({
                severity: 'error',
                message: `Token length mismatch at position ${i + 1}: "${phrase.swaras[i]}" (${swaraLen}) vs "${phrase.sahitya[i]}" (${sahityaLen})`,
                line: phrase.line_number + 1,
              });
            }
          }
        }
      }
    } catch (e) {
      issues.push({
        severity: 'error',
        message: e instanceof Error ? e.message : 'Unknown parsing error',
        line: this.currentLine + 1,
      });
    }

    return issues;
  }

  // Utility functions for editor features
  getSwaraTokens(): string[] {
    return ['S', 'R', 'G', 'M', 'P', 'D', 'N',
            'R1', 'R2', 'R3', 'G1', 'G2', 'G3',
            'M1', 'M2', 'D1', 'D2', 'D3', 'N1', 'N2', 'N3'];
  }

  getSectionNames(): string[] {
    return ['pallavi', 'anupallavi', 'muktasvara', 'charanam',
            'cittasvaras', 'geetam', 'ragamalika', 'madhyamakala', 'drut'];
  }

  getMetadataFields(): Array<{ name: string; required: boolean }> {
    return [
      { name: 'title', required: true },
      { name: 'raga', required: true },
      { name: 'tala', required: true },
      { name: 'tempo', required: false },
      { name: 'composer', required: false },
      { name: 'language', required: false },
      { name: 'type', required: false },
      { name: 'key', required: false },
      { name: 'gati', required: false },
      { name: 'default_octave', required: false },
      { name: 'arohanam', required: false },
      { name: 'avarohanam', required: false },
    ];
  }
}
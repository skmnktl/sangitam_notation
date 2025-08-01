import { VNADocument, VNAMetadata, VNASection, VNAPhrase, ParsedSwara, Swara, OctaveMarker } from '../types/vna';

export class VNAParser {
  private static readonly SWARA_REGEX = /([SRGMPDN])(\d)?(\.{1,2}|'{1,2})?/g;
  private static readonly SECTION_REGEX = /^\[(.+)\]$/;
  private static readonly METADATA_DELIMITER = '---';

  static parse(content: string): VNADocument {
    const lines = content.split('\n');
    const metadata = this.parseMetadata(lines);
    const sections = this.parseSections(lines);

    return {
      metadata,
      sections
    };
  }

  private static parseMetadata(lines: string[]): VNAMetadata {
    const startIdx = lines.findIndex(line => line.trim() === this.METADATA_DELIMITER);
    const endIdx = lines.findIndex((line, idx) => 
      idx > startIdx && line.trim() === this.METADATA_DELIMITER
    );

    if (startIdx === -1 || endIdx === -1) {
      throw new Error('Invalid VNA file: missing metadata section');
    }

    const metadataLines = lines.slice(startIdx + 1, endIdx);
    const metadata: any = {};

    metadataLines.forEach(line => {
      const match = line.match(/^(\w+):\s*"?(.+?)"?$/);
      if (match) {
        const [, key, value] = match;
        metadata[key] = key === 'tempo' ? parseInt(value) : value;
      }
    });

    if (!metadata.title || !metadata.raga || !metadata.tala) {
      throw new Error('Invalid VNA file: missing required metadata fields');
    }

    return metadata as VNAMetadata;
  }

  private static parseSections(lines: string[]): VNASection[] {
    const sections: VNASection[] = [];
    let currentSection: VNASection | null = null;
    let currentPhrase: Partial<VNAPhrase> = {};
    let inMetadata = true;

    lines.forEach(line => {
      const trimmed = line.trim();

      if (trimmed === this.METADATA_DELIMITER) {
        if (inMetadata && lines.filter(l => l.trim() === this.METADATA_DELIMITER).length === 2) {
          inMetadata = !inMetadata;
        }
        return;
      }

      if (inMetadata) return;

      if (trimmed.startsWith('#')) {
        if (currentSection) {
          currentSection.comments = currentSection.comments || [];
          currentSection.comments.push(trimmed.substring(1).trim());
        }
        return;
      }

      if (trimmed.startsWith('@')) {
        if (currentSection) {
          const [key, value] = trimmed.substring(1).split(':').map(s => s.trim());
          currentSection.annotations = currentSection.annotations || {};
          currentSection.annotations[key] = value;
        }
        return;
      }

      const sectionMatch = trimmed.match(this.SECTION_REGEX);
      if (sectionMatch) {
        if (currentSection) {
          sections.push(currentSection);
        }
        currentSection = {
          name: sectionMatch[1],
          phrases: []
        };
        currentPhrase = {};
        return;
      }

      if (!currentSection) return;

      if (trimmed.startsWith('phrases =')) {
        currentPhrase.phrases = trimmed.substring('phrases ='.length).trim();
        if (currentPhrase.swaras && currentPhrase.sahitya) {
          currentSection.phrases.push(currentPhrase as VNAPhrase);
          currentPhrase = {};
        }
        return;
      }

      if (trimmed && !currentPhrase.swaras) {
        currentPhrase.swaras = trimmed;
      } else if (trimmed && currentPhrase.swaras && !currentPhrase.sahitya) {
        currentPhrase.sahitya = trimmed;
        if (!currentPhrase.phrases) {
          currentSection.phrases.push(currentPhrase as VNAPhrase);
          currentPhrase = {};
        }
      }
    });

    if (currentSection) {
      sections.push(currentSection);
    }

    return sections;
  }

  static parseSwaraLine(swaraLine: string): ParsedSwara[] {
    const tokens = swaraLine.split(/\s+/);
    const parsedSwaras: ParsedSwara[] = [];

    tokens.forEach(token => {
      if (token === '-') {
        parsedSwaras.push({
          swara: 'S' as Swara,
          octave: '' as OctaveMarker,
          duration: 1,
          isRest: true,
          isSustain: false
        });
      } else if (token === ',') {
        parsedSwaras.push({
          swara: 'S' as Swara,
          octave: '' as OctaveMarker,
          duration: 1,
          isRest: false,
          isSustain: true
        });
      } else if (token === '|' || token === '||') {
        // Beat markers - skip for now
      } else if (token.includes('-') || token.includes(',')) {
        // Handle tokens like "RSR-" or "G,G," where - is rest and , is sustain
        const chars = token.split('');
        let i = 0;
        while (i < chars.length) {
          if (chars[i] === '-') {
            parsedSwaras.push({
              swara: 'S' as Swara,
              octave: '' as OctaveMarker,
              duration: 1,
              isRest: true,
              isSustain: false
            });
            i++;
          } else if (chars[i] === ',') {
            parsedSwaras.push({
              swara: 'S' as Swara,
              octave: '' as OctaveMarker,
              duration: 1,
              isRest: false,
              isSustain: true
            });
            i++;
          } else if (/[SRGMPDN]/.test(chars[i])) {
            // Look for swara with possible variant and octave
            let notation = chars[i];
            i++;
            // Check for variant number
            if (i < chars.length && /\d/.test(chars[i])) {
              notation += chars[i];
              i++;
            }
            // Check for octave markers
            while (i < chars.length && (chars[i] === '.' || chars[i] === "'")) {
              notation += chars[i];
              i++;
            }
            
            const matches = Array.from(notation.matchAll(this.SWARA_REGEX));
            if (matches.length > 0) {
              const [, swara, variant, octave = ''] = matches[0];
              parsedSwaras.push({
                swara: swara as Swara,
                variant: variant ? parseInt(variant) : undefined,
                octave: (octave || '') as OctaveMarker,
                duration: 1,
                isRest: false,
                isSustain: false
              });
            }
          } else {
            i++; // Skip unknown characters
          }
        }
      } else {
        const matches = Array.from(token.matchAll(this.SWARA_REGEX));
        matches.forEach(match => {
          const [, swara, variant, octave = ''] = match;
          parsedSwaras.push({
            swara: swara as Swara,
            variant: variant ? parseInt(variant) : undefined,
            octave: (octave || '') as OctaveMarker,
            duration: 1,
            isRest: false,
            isSustain: false
          });
        });
      }
    });

    return parsedSwaras;
  }

  static getSwaraPosition(swara: Swara, octave: OctaveMarker = ''): number {
    const basePositions: Record<Swara, number> = {
      'S': 0,
      'R': 1,
      'G': 2,
      'M': 3,
      'P': 4,
      'D': 5,
      'N': 6
    };

    let position = basePositions[swara];
    
    switch (octave) {
      case '..': position -= 14; break;
      case '.': position -= 7; break;
      case "'": position += 7; break;
      case "''": position += 14; break;
    }

    return position;
  }
}
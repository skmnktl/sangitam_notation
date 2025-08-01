// WASM-based VNA parser integration
import { parseVna, validateVna, parseLine } from '../wasm-parser-safe';
import { VNADocument, VNASection, VNAPhrase, VNAMetadata, ParsedSwara, Swara, OctaveMarker } from '../types/vna';
import { normalizeVNADocument } from './vna-parser-adapter';

export class VNAParser {
  static async parse(content: string): Promise<VNADocument> {
    try {
      const result = await parseVna(content);
      return normalizeVNADocument(result);
    } catch (error: any) {
      console.error('VNA Parser error:', error);
      throw new Error(`Failed to parse VNA: ${error.message || 'Unknown error'}`);
    }
  }

  static async validate(content: string): Promise<any[]> {
    return await validateVna(content);
  }

  static parseSwaraLine(token: string): ParsedSwara[] {
    const swaras: ParsedSwara[] = [];
    
    // Handle rest marker
    if (token === '-') {
      return [{ swara: '-' as Swara, isRest: true }];
    }
    
    // Handle sustain marker
    if (token === ',') {
      return [{ swara: 'S' as Swara, isSustain: true }];
    }
    
    let i = 0;
    while (i < token.length) {
      const char = token[i];
      
      // Check if it's a sustain marker within compound token
      if (char === ',') {
        swaras.push({ swara: 'S' as Swara, isSustain: true });
        i++;
        continue;
      }
      
      // Check if it's a rest marker within compound token
      if (char === '-') {
        swaras.push({ swara: '-' as Swara, isRest: true });
        i++;
        continue;
      }
      
      // Check if it's a valid swara
      if (['S', 'R', 'G', 'M', 'P', 'D', 'N'].includes(char)) {
        const swara: ParsedSwara = { swara: char as Swara };
        
        // Check for variant (1, 2, 3)
        if (i + 1 < token.length && ['1', '2', '3'].includes(token[i + 1])) {
          swara.variant = token[i + 1];
          i++;
        }
        
        // Check for octave markers
        let octaveCount = 0;
        let octaveType: OctaveMarker | undefined;
        
        // Check for lower octave markers (.)
        while (i + 1 < token.length && token[i + 1] === '.') {
          octaveCount++;
          i++;
        }
        if (octaveCount === 1) octaveType = '.';
        else if (octaveCount === 2) octaveType = '..';
        
        // Check for upper octave markers (')
        if (!octaveType) {
          while (i + 1 < token.length && token[i + 1] === "'") {
            octaveCount++;
            i++;
          }
          if (octaveCount === 1) octaveType = "'";
          else if (octaveCount === 2) octaveType = "''";
        }
        
        if (octaveType) {
          swara.octave = octaveType;
        }
        
        swaras.push(swara);
      }
      
      i++;
    }
    
    return swaras;
  }

  static parseMetadata(lines: string[]): { metadata: VNAMetadata; endIndex: number } {
    const metadata: Partial<VNAMetadata> = {};
    let inFrontMatter = false;
    let endIndex = 0;

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i].trim();
      
      if (line === '---') {
        if (!inFrontMatter) {
          inFrontMatter = true;
        } else {
          endIndex = i + 1;
          break;
        }
      } else if (inFrontMatter) {
        const colonIndex = line.indexOf(':');
        if (colonIndex > 0) {
          const key = line.substring(0, colonIndex).trim();
          const value = line.substring(colonIndex + 1).trim().replace(/^["']|["']$/g, '');
          
          switch (key) {
            case 'title':
              metadata.title = value;
              break;
            case 'raga':
              metadata.raga = value;
              break;
            case 'tala':
              metadata.tala = value;
              break;
            case 'tempo':
              metadata.tempo = parseInt(value);
              break;
            case 'composer':
              metadata.composer = value;
              break;
            case 'language':
              metadata.language = value;
              break;
            case 'gati':
              metadata.gati = parseInt(value);
              break;
          }
        }
      }
    }

    return {
      metadata: metadata as VNAMetadata,
      endIndex
    };
  }
}
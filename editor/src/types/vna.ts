export interface VNAMetadata {
  title: string;
  raga: string;
  tala: string;
  type?: string;
  tempo?: number;
  composer?: string;
  language?: string;
  key?: string;
  gati?: number;
}

export interface VNAPhrase {
  swaras: string[] | string;  // Can be array from WASM or string from TS parser
  sahitya: string[] | string;  // Can be array from WASM or string from TS parser
  phrase_analysis?: string;
  beat_positions?: number[];
  gati?: number;
  tala?: string;
}

export interface VNASection {
  name: string;
  phrases: VNAPhrase[];
  comments?: string[];
  annotations?: Record<string, string>;
}

export interface VNADocument {
  metadata: VNAMetadata;
  sections: VNASection[];
}

export type Swara = 'S' | 'R' | 'G' | 'M' | 'P' | 'D' | 'N';
export type OctaveMarker = '..' | '.' | '' | "'" | "''";

export interface ParsedSwara {
  swara: Swara | '-';
  variant?: string;
  octave?: OctaveMarker;
  duration?: number;
  isRest?: boolean;
  isSustain?: boolean;
}
export interface CurvePoint {
  x: number;
  y: number;
  pressure: number;
}

export type GamakaType = 'kampita' | 'andolita' | 'jaru' | 'tribhinna' | 'custom';

export interface GamakaMarkers {
  start?: 'circle' | 'square' | 'diamond' | 'triangle';
  end?: 'circle' | 'square' | 'diamond' | 'triangle' | 'arrow';
}

export interface Gamaka {
  id: string;
  points: CurvePoint[];
  type: GamakaType;
  swaraStart: string;
  swaraEnd: string;
  phraseIndex: number;
  sectionName: string;
  markers?: GamakaMarkers;
}

export interface GamakaTemplate {
  name: string;
  type: GamakaType;
  points: CurvePoint[];
  description?: string;
}

export interface CurveData {
  version: string;
  fileHash: string;
  gamakas: Gamaka[];
  createdAt: string;
  updatedAt: string;
}
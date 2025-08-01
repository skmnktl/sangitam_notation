export interface StaffPosition {
  swara: string;
  y: number;
}

export interface BeatMarker {
  x: number;
  type: 'single' | 'double' | 'start';
}

export interface StaffLayout {
  width: number;
  height: number;
  marginTop: number;
  marginBottom: number;
  marginLeft: number;
  marginRight: number;
  lineSpacing: number;
  staffPositions: StaffPosition[];
  beatMarkers: BeatMarker[];
}

export interface DrawingState {
  isDrawing: boolean;
  currentPath: CurvePoint[];
  selectedGamaka: string | null;
  selectedTemplate: string | null;
}

export interface CanvasOptions {
  snapToStaff: boolean;
  showGrid: boolean;
  showBeatMarkers: boolean;
  smoothingFactor: number;
  pressureSensitivity: boolean;
}
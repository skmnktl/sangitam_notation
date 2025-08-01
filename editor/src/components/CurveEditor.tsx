import React, { useRef, useEffect, useState, useCallback } from 'react';
import { Gamaka } from '../types/gamaka';
import { StaffLayout, DrawingState, CanvasOptions } from '../types/canvas';
import { CurveEngine } from '../lib/curve-engine';
import type { CurvePoint } from '../types/gamaka';

interface CurveEditorProps {
  layout: StaffLayout;
  gamakas: Gamaka[];
  onGamakaUpdate: (gamaka: Gamaka) => void;
  onGamakaAdd: (gamaka: Gamaka) => void;
  options?: Partial<CanvasOptions>;
  sectionName: string;
  phraseIndex: number;
}

export const CurveEditor: React.FC<CurveEditorProps> = ({
  layout,
  gamakas,
  onGamakaUpdate,
  onGamakaAdd,
  options = {},
  sectionName,
  phraseIndex
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [drawingState, setDrawingState] = useState<DrawingState>({
    isDrawing: false,
    currentPath: [],
    selectedGamaka: null,
    selectedTemplate: null
  });

  const canvasOptions: CanvasOptions = {
    snapToStaff: true,
    showGrid: false,
    showBeatMarkers: true,
    smoothingFactor: 0.3,
    pressureSensitivity: true,
    ...options
  };

  // Default markers for different gamaka types
  const getDefaultMarkers = (type: string) => {
    switch (type) {
      case 'kampita':
        return { start: 'circle', end: 'circle' };
      case 'jaru':
        return { start: 'square', end: 'arrow' };
      case 'andolita':
        return { start: 'diamond', end: 'diamond' };
      case 'tribhinna':
        return { start: 'triangle', end: 'arrow' };
      default:
        return { start: 'circle', end: 'arrow' };
    }
  };

  useEffect(() => {
    drawCurves();
  }, [gamakas, layout]);

  const drawCurves = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    ctx.clearRect(0, 0, layout.width, layout.height);

    // Draw existing gamakas
    gamakas.forEach(gamaka => {
      const color = gamaka.id === drawingState.selectedGamaka ? '#0066cc' : '#333';
      const markers = gamaka.markers || getDefaultMarkers(gamaka.type);
      CurveEngine.drawBezierCurve(ctx, gamaka.points, color, 3, markers);
    });

    // Draw current path
    if (drawingState.currentPath.length > 0) {
      CurveEngine.drawBezierCurve(ctx, drawingState.currentPath, '#cc0000', 2);
    }
  };

  const getPointerPosition = (e: React.PointerEvent<HTMLCanvasElement>): CurvePoint => {
    const canvas = canvasRef.current;
    if (!canvas) return { x: 0, y: 0, pressure: 0.5 };

    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    let y = e.clientY - rect.top;

    if (canvasOptions.snapToStaff) {
      y = CurveEngine.snapToStaff(y, layout.staffPositions);
    }

    const pressure = canvasOptions.pressureSensitivity 
      ? CurveEngine.interpolatePressure(e.pressure)
      : 0.5;

    return { x, y, pressure };
  };

  const handlePointerDown = useCallback((e: React.PointerEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    canvas.setPointerCapture(e.pointerId);
    const point = getPointerPosition(e);

    setDrawingState(prev => ({
      ...prev,
      isDrawing: true,
      currentPath: [point]
    }));
  }, [canvasOptions]);

  const handlePointerMove = useCallback((e: React.PointerEvent<HTMLCanvasElement>) => {
    if (!drawingState.isDrawing) return;

    const point = getPointerPosition(e);
    
    setDrawingState(prev => ({
      ...prev,
      currentPath: [...prev.currentPath, point]
    }));

    requestAnimationFrame(drawCurves);
  }, [drawingState.isDrawing, canvasOptions]);

  const handlePointerUp = useCallback((e: React.PointerEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    canvas.releasePointerCapture(e.pointerId);

    if (drawingState.currentPath.length > 2) {
      const simplifiedPath = CurveEngine.simplifyPath(drawingState.currentPath);
      const smoothedPath = CurveEngine.smoothCurve(simplifiedPath, canvasOptions.smoothingFactor);

      const newGamaka: Gamaka = {
        id: `gamaka_${Date.now()}`,
        points: smoothedPath,
        type: 'custom',
        swaraStart: '',
        swaraEnd: '',
        phraseIndex,
        sectionName,
        markers: getDefaultMarkers('custom')
      };

      onGamakaAdd(newGamaka);
    }

    setDrawingState(prev => ({
      ...prev,
      isDrawing: false,
      currentPath: []
    }));
  }, [drawingState.currentPath, canvasOptions, phraseIndex, sectionName, onGamakaAdd]);

  const handlePointerCancel = useCallback((e: React.PointerEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    canvas.releasePointerCapture(e.pointerId);
    
    setDrawingState(prev => ({
      ...prev,
      isDrawing: false,
      currentPath: []
    }));
  }, []);

  const handleKeyDown = useCallback((e: KeyboardEvent) => {
    if (e.key === 'Escape') {
      setDrawingState(prev => ({
        ...prev,
        isDrawing: false,
        currentPath: []
      }));
    } else if (e.key === 'Delete' && drawingState.selectedGamaka) {
      // Handle deletion of selected gamaka
    }
  }, [drawingState.selectedGamaka]);

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);

  return (
    <div style={{ position: 'relative' }}>
      <canvas
        ref={canvasRef}
        width={layout.width}
        height={layout.height}
        style={{
          position: 'absolute',
          top: 0,
          left: 0,
          touchAction: 'none',
          cursor: drawingState.isDrawing ? 'crosshair' : 'default'
        }}
        onPointerDown={handlePointerDown}
        onPointerMove={handlePointerMove}
        onPointerUp={handlePointerUp}
        onPointerCancel={handlePointerCancel}
      />
    </div>
  );
};
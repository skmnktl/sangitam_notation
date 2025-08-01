import React, { useRef, useEffect, useState, useCallback } from 'react';
import { 
  InflectionCurveEngine, 
  InflectionCurve, 
  InflectionPoint,
  CurveSegmentType 
} from '../lib/inflection-curve-engine';
import { CurvePoint } from '../types/gamaka';

interface InflectionCurveEditorProps {
  width: number;
  height: number;
  onCurveUpdate?: (curve: InflectionCurve) => void;
  initialCurve?: InflectionCurve;
}

interface EditorState {
  curve: InflectionCurve | null;
  selectedPointId: string | null;
  isDragging: boolean;
  isAddingPoint: boolean;
  hoveredSegmentIndex: number | null;
}

export const InflectionCurveEditor: React.FC<InflectionCurveEditorProps> = ({
  width,
  height,
  onCurveUpdate,
  initialCurve
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [state, setState] = useState<EditorState>({
    curve: initialCurve || null,
    selectedPointId: null,
    isDragging: false,
    isAddingPoint: false,
    hoveredSegmentIndex: null
  });

  const [segmentTypeMenu, setSegmentTypeMenu] = useState<{
    show: boolean;
    x: number;
    y: number;
    segmentIndex: number;
  }>({ show: false, x: 0, y: 0, segmentIndex: -1 });

  useEffect(() => {
    draw();
  }, [state.curve, state.selectedPointId, state.hoveredSegmentIndex]);

  const draw = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    ctx.clearRect(0, 0, width, height);

    // Draw grid
    ctx.strokeStyle = '#f0f0f0';
    ctx.lineWidth = 1;
    for (let x = 0; x < width; x += 20) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, height);
      ctx.stroke();
    }
    for (let y = 0; y < height; y += 20) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(width, y);
      ctx.stroke();
    }

    // Draw curve if exists
    if (state.curve) {
      // Highlight hovered segment
      if (state.hoveredSegmentIndex !== null && state.curve.segments[state.hoveredSegmentIndex]) {
        const segment = state.curve.segments[state.hoveredSegmentIndex];
        const points = InflectionCurveEngine.interpolateSegment(segment);
        
        ctx.strokeStyle = 'rgba(0, 102, 204, 0.3)';
        ctx.lineWidth = 8;
        ctx.beginPath();
        points.forEach((point, index) => {
          if (index === 0) ctx.moveTo(point.x, point.y);
          else ctx.lineTo(point.x, point.y);
        });
        ctx.stroke();
      }

      InflectionCurveEngine.drawInflectionCurve(ctx, state.curve, {
        selectedPointId: state.selectedPointId,
        showControlPoints: true
      });
    }

    // Draw add point mode indicator
    if (state.isAddingPoint) {
      ctx.fillStyle = 'rgba(0, 255, 0, 0.5)';
      ctx.font = '14px Arial';
      ctx.fillText('Click to add inflection point', 10, 20);
    }
  };

  const getMousePosition = (e: React.MouseEvent<HTMLCanvasElement>): CurvePoint => {
    const canvas = canvasRef.current;
    if (!canvas) return { x: 0, y: 0, pressure: 0.5 };

    const rect = canvas.getBoundingClientRect();
    return {
      x: e.clientX - rect.left,
      y: e.clientY - rect.top,
      pressure: 0.5
    };
  };

  const handleMouseDown = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
    const pos = getMousePosition(e);

    if (state.isAddingPoint && state.curve) {
      // Add new inflection point
      const updatedCurve = InflectionCurveEngine.addInflectionPoint(state.curve, pos);
      setState(prev => ({ 
        ...prev, 
        curve: updatedCurve, 
        isAddingPoint: false 
      }));
      onCurveUpdate?.(updatedCurve);
      return;
    }

    if (!state.curve) {
      // Create new curve with first point
      const firstPoint = InflectionCurveEngine.createInflectionPoint(pos.x, pos.y);
      const newCurve: InflectionCurve = {
        id: `curve_${Date.now()}`,
        inflectionPoints: [firstPoint],
        segments: []
      };
      setState(prev => ({ ...prev, curve: newCurve }));
      return;
    }

    // Check if clicking on existing point
    const clickedPoint = InflectionCurveEngine.findClosestPoint(state.curve, pos.x, pos.y);
    if (clickedPoint && !clickedPoint.locked) {
      setState(prev => ({
        ...prev,
        selectedPointId: clickedPoint.id,
        isDragging: true
      }));
    } else if (state.curve.inflectionPoints.length < 10) {
      // Add new point if not at limit
      const updatedCurve = InflectionCurveEngine.addInflectionPoint(state.curve, pos);
      setState(prev => ({ ...prev, curve: updatedCurve }));
      onCurveUpdate?.(updatedCurve);
    }
  }, [state, onCurveUpdate]);

  const handleMouseMove = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
    const pos = getMousePosition(e);

    // Check for hovered segment
    if (state.curve && !state.isDragging) {
      let hoveredIndex: number | null = null;
      
      state.curve.segments.forEach((segment, index) => {
        const points = InflectionCurveEngine.interpolateSegment(segment);
        for (const point of points) {
          const distance = Math.sqrt(Math.pow(pos.x - point.x, 2) + Math.pow(pos.y - point.y, 2));
          if (distance < 10) {
            hoveredIndex = index;
            break;
          }
        }
      });
      
      if (hoveredIndex !== state.hoveredSegmentIndex) {
        setState(prev => ({ ...prev, hoveredSegmentIndex: hoveredIndex }));
      }
    }

    // Handle dragging
    if (state.isDragging && state.selectedPointId && state.curve) {
      const updatedCurve = { ...state.curve };
      const point = updatedCurve.inflectionPoints.find(p => p.id === state.selectedPointId);
      
      if (point && !point.locked) {
        point.x = pos.x;
        point.y = pos.y;
        
        // Rebuild segments
        updatedCurve.segments = InflectionCurveEngine.buildSegments(updatedCurve.inflectionPoints);
        
        setState(prev => ({ ...prev, curve: updatedCurve }));
        onCurveUpdate?.(updatedCurve);
      }
    }
  }, [state, onCurveUpdate]);

  const handleMouseUp = useCallback(() => {
    setState(prev => ({ ...prev, isDragging: false }));
  }, []);

  const handleRightClick = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
    e.preventDefault();
    
    if (state.hoveredSegmentIndex !== null) {
      const rect = canvasRef.current?.getBoundingClientRect();
      if (rect) {
        setSegmentTypeMenu({
          show: true,
          x: e.clientX - rect.left,
          y: e.clientY - rect.top,
          segmentIndex: state.hoveredSegmentIndex
        });
      }
    }
  }, [state.hoveredSegmentIndex]);

  const changeSegmentType = (type: CurveSegmentType) => {
    if (state.curve && segmentTypeMenu.segmentIndex >= 0) {
      const segment = state.curve.segments[segmentTypeMenu.segmentIndex];
      if (segment) {
        const updatedCurve = InflectionCurveEngine.updateSegmentType(
          state.curve,
          segment.startPoint.id,
          type
        );
        setState(prev => ({ ...prev, curve: updatedCurve }));
        onCurveUpdate?.(updatedCurve);
      }
    }
    setSegmentTypeMenu({ show: false, x: 0, y: 0, segmentIndex: -1 });
  };

  const handleKeyDown = useCallback((e: KeyboardEvent) => {
    if (e.key === 'Delete' && state.selectedPointId && state.curve) {
      // Remove selected point
      const updatedPoints = state.curve.inflectionPoints.filter(p => p.id !== state.selectedPointId);
      if (updatedPoints.length > 0) {
        const updatedCurve = {
          ...state.curve,
          inflectionPoints: updatedPoints,
          segments: InflectionCurveEngine.buildSegments(updatedPoints)
        };
        setState(prev => ({ 
          ...prev, 
          curve: updatedCurve, 
          selectedPointId: null 
        }));
        onCurveUpdate?.(updatedCurve);
      }
    } else if (e.key === 'a' || e.key === 'A') {
      // Toggle add point mode
      setState(prev => ({ ...prev, isAddingPoint: !prev.isAddingPoint }));
    } else if (e.key === 'l' && state.selectedPointId && state.curve) {
      // Lock/unlock selected point
      const point = state.curve.inflectionPoints.find(p => p.id === state.selectedPointId);
      if (point) {
        point.locked = !point.locked;
        setState(prev => ({ ...prev }));
      }
    }
  }, [state, onCurveUpdate]);

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);

  return (
    <div style={{ position: 'relative' }}>
      <canvas
        ref={canvasRef}
        width={width}
        height={height}
        style={{
          border: '1px solid #ccc',
          cursor: state.isAddingPoint ? 'crosshair' : 'default'
        }}
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseUp}
        onContextMenu={handleRightClick}
      />
      
      {/* Segment type menu */}
      {segmentTypeMenu.show && (
        <div
          style={{
            position: 'absolute',
            left: segmentTypeMenu.x,
            top: segmentTypeMenu.y,
            background: 'white',
            border: '1px solid #ccc',
            borderRadius: '4px',
            padding: '4px',
            boxShadow: '0 2px 8px rgba(0,0,0,0.1)'
          }}
        >
          <div style={{ fontWeight: 'bold', marginBottom: '4px', padding: '4px' }}>
            Curve Type:
          </div>
          {(['linear', 'quadratic', 'cubic', 'sinusoidal', 'exponential'] as CurveSegmentType[]).map(type => (
            <div
              key={type}
              style={{
                padding: '4px 8px',
                cursor: 'pointer',
                ':hover': { background: '#f0f0f0' }
              }}
              onClick={() => changeSegmentType(type)}
              onMouseEnter={(e) => e.currentTarget.style.background = '#f0f0f0'}
              onMouseLeave={(e) => e.currentTarget.style.background = 'white'}
            >
              {type.charAt(0).toUpperCase() + type.slice(1)}
            </div>
          ))}
        </div>
      )}
      
      {/* Instructions */}
      <div style={{ marginTop: '10px', fontSize: '12px', color: '#666' }}>
        Click to add points • Drag points to move • Right-click segments to change type • 
        Press 'A' to toggle add mode • 'Delete' to remove selected • 'L' to lock/unlock
      </div>
    </div>
  );
};
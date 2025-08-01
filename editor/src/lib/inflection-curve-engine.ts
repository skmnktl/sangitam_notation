import { CurvePoint } from '../types/gamaka';

export type CurveSegmentType = 'linear' | 'quadratic' | 'cubic' | 'sinusoidal' | 'exponential';

export interface InflectionPoint extends CurvePoint {
  id: string;
  locked?: boolean; // Prevent accidental movement
}

export interface CurveSegment {
  startPoint: InflectionPoint;
  endPoint: InflectionPoint;
  type: CurveSegmentType;
  tension?: number; // Controls curve tightness (0-1)
  controlPoints?: CurvePoint[]; // For custom bezier curves
}

export interface InflectionCurve {
  id: string;
  inflectionPoints: InflectionPoint[];
  segments: CurveSegment[];
  closed?: boolean; // Connect last point to first
}

export class InflectionCurveEngine {
  // Create a new inflection point
  static createInflectionPoint(x: number, y: number, pressure: number = 0.5): InflectionPoint {
    return {
      id: `ip_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      x,
      y,
      pressure,
      locked: false
    };
  }

  // Add inflection point to curve at specific position
  static addInflectionPoint(curve: InflectionCurve, point: CurvePoint, afterId?: string): InflectionCurve {
    const newPoint = this.createInflectionPoint(point.x, point.y, point.pressure);
    
    if (!afterId || curve.inflectionPoints.length === 0) {
      // Add at the end
      curve.inflectionPoints.push(newPoint);
    } else {
      // Insert after specified point
      const index = curve.inflectionPoints.findIndex(p => p.id === afterId);
      if (index !== -1) {
        curve.inflectionPoints.splice(index + 1, 0, newPoint);
      }
    }
    
    // Rebuild segments
    curve.segments = this.buildSegments(curve.inflectionPoints);
    
    return curve;
  }

  // Build segments between inflection points
  static buildSegments(points: InflectionPoint[]): CurveSegment[] {
    const segments: CurveSegment[] = [];
    
    for (let i = 0; i < points.length - 1; i++) {
      segments.push({
        startPoint: points[i],
        endPoint: points[i + 1],
        type: 'cubic', // Default to smooth cubic curves
        tension: 0.5
      });
    }
    
    return segments;
  }

  // Generate interpolated points for a segment
  static interpolateSegment(segment: CurveSegment, resolution: number = 50): CurvePoint[] {
    const points: CurvePoint[] = [];
    const { startPoint, endPoint, type, tension = 0.5 } = segment;
    
    for (let t = 0; t <= 1; t += 1 / resolution) {
      let point: CurvePoint;
      
      switch (type) {
        case 'linear':
          point = this.linearInterpolation(startPoint, endPoint, t);
          break;
          
        case 'quadratic':
          point = this.quadraticInterpolation(startPoint, endPoint, t, tension);
          break;
          
        case 'cubic':
          point = this.cubicInterpolation(startPoint, endPoint, t, tension, segment.controlPoints);
          break;
          
        case 'sinusoidal':
          point = this.sinusoidalInterpolation(startPoint, endPoint, t, tension);
          break;
          
        case 'exponential':
          point = this.exponentialInterpolation(startPoint, endPoint, t, tension);
          break;
          
        default:
          point = this.linearInterpolation(startPoint, endPoint, t);
      }
      
      points.push(point);
    }
    
    return points;
  }

  // Linear interpolation
  static linearInterpolation(start: CurvePoint, end: CurvePoint, t: number): CurvePoint {
    return {
      x: start.x + (end.x - start.x) * t,
      y: start.y + (end.y - start.y) * t,
      pressure: start.pressure + (end.pressure - start.pressure) * t
    };
  }

  // Quadratic interpolation with tension
  static quadraticInterpolation(start: CurvePoint, end: CurvePoint, t: number, tension: number): CurvePoint {
    const midX = (start.x + end.x) / 2;
    const midY = (start.y + end.y) / 2 - tension * Math.abs(end.x - start.x) * 0.5;
    
    const x = (1 - t) * (1 - t) * start.x + 2 * (1 - t) * t * midX + t * t * end.x;
    const y = (1 - t) * (1 - t) * start.y + 2 * (1 - t) * t * midY + t * t * end.y;
    
    return {
      x,
      y,
      pressure: start.pressure + (end.pressure - start.pressure) * t
    };
  }

  // Cubic interpolation with optional control points
  static cubicInterpolation(
    start: CurvePoint,
    end: CurvePoint,
    t: number,
    tension: number,
    controlPoints?: CurvePoint[]
  ): CurvePoint {
    let cp1: CurvePoint, cp2: CurvePoint;
    
    if (controlPoints && controlPoints.length >= 2) {
      cp1 = controlPoints[0];
      cp2 = controlPoints[1];
    } else {
      // Auto-generate control points based on tension
      const dx = end.x - start.x;
      const dy = end.y - start.y;
      
      cp1 = {
        x: start.x + dx * 0.25,
        y: start.y + dy * 0.25 - tension * Math.abs(dx) * 0.3,
        pressure: start.pressure
      };
      
      cp2 = {
        x: end.x - dx * 0.25,
        y: end.y - dy * 0.25 + tension * Math.abs(dx) * 0.3,
        pressure: end.pressure
      };
    }
    
    // Cubic Bezier formula
    const t2 = t * t;
    const t3 = t2 * t;
    const mt = 1 - t;
    const mt2 = mt * mt;
    const mt3 = mt2 * mt;
    
    return {
      x: mt3 * start.x + 3 * mt2 * t * cp1.x + 3 * mt * t2 * cp2.x + t3 * end.x,
      y: mt3 * start.y + 3 * mt2 * t * cp1.y + 3 * mt * t2 * cp2.y + t3 * end.y,
      pressure: start.pressure + (end.pressure - start.pressure) * t
    };
  }

  // Sinusoidal interpolation for wave-like gamakas
  static sinusoidalInterpolation(start: CurvePoint, end: CurvePoint, t: number, tension: number): CurvePoint {
    const linearX = start.x + (end.x - start.x) * t;
    const linearY = start.y + (end.y - start.y) * t;
    
    // Add sinusoidal variation
    const amplitude = tension * Math.abs(end.x - start.x) * 0.2;
    const frequency = Math.PI * 2; // One complete wave
    const waveOffset = amplitude * Math.sin(t * frequency);
    
    return {
      x: linearX,
      y: linearY + waveOffset,
      pressure: start.pressure + (end.pressure - start.pressure) * t
    };
  }

  // Exponential interpolation for sharp curves
  static exponentialInterpolation(start: CurvePoint, end: CurvePoint, t: number, tension: number): CurvePoint {
    // Use exponential easing
    const easedT = tension > 0.5 
      ? Math.pow(t, 2 + (tension - 0.5) * 4) // Ease in
      : 1 - Math.pow(1 - t, 2 + (0.5 - tension) * 4); // Ease out
    
    return {
      x: start.x + (end.x - start.x) * easedT,
      y: start.y + (end.y - start.y) * easedT,
      pressure: start.pressure + (end.pressure - start.pressure) * t
    };
  }

  // Draw the complete curve with all segments
  static drawInflectionCurve(
    ctx: CanvasRenderingContext2D,
    curve: InflectionCurve,
    options: {
      curveColor?: string;
      pointColor?: string;
      selectedPointId?: string;
      showControlPoints?: boolean;
      pointSize?: number;
    } = {}
  ) {
    const {
      curveColor = '#333',
      pointColor = '#0066cc',
      selectedPointId,
      showControlPoints = false,
      pointSize = 6
    } = options;

    // Draw each segment
    curve.segments.forEach(segment => {
      const points = this.interpolateSegment(segment);
      
      ctx.strokeStyle = curveColor;
      ctx.lineWidth = 2;
      ctx.beginPath();
      
      points.forEach((point, index) => {
        if (index === 0) {
          ctx.moveTo(point.x, point.y);
        } else {
          ctx.lineTo(point.x, point.y);
        }
      });
      
      ctx.stroke();
      
      // Draw control points if enabled
      if (showControlPoints && segment.controlPoints) {
        ctx.fillStyle = 'rgba(255, 0, 0, 0.5)';
        segment.controlPoints.forEach(cp => {
          ctx.beginPath();
          ctx.arc(cp.x, cp.y, 3, 0, Math.PI * 2);
          ctx.fill();
        });
      }
    });

    // Draw inflection points
    curve.inflectionPoints.forEach(point => {
      const isSelected = point.id === selectedPointId;
      
      ctx.fillStyle = isSelected ? '#ff6600' : pointColor;
      ctx.strokeStyle = isSelected ? '#ff6600' : pointColor;
      ctx.lineWidth = 2;
      
      ctx.beginPath();
      ctx.arc(point.x, point.y, pointSize, 0, Math.PI * 2);
      ctx.fill();
      
      if (point.locked) {
        // Draw lock indicator
        ctx.strokeStyle = '#666';
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.arc(point.x, point.y, pointSize + 2, 0, Math.PI * 2);
        ctx.stroke();
      }
    });
  }

  // Find closest inflection point to a position
  static findClosestPoint(curve: InflectionCurve, x: number, y: number, threshold: number = 10): InflectionPoint | null {
    let closest: InflectionPoint | null = null;
    let minDistance = threshold;
    
    curve.inflectionPoints.forEach(point => {
      const distance = Math.sqrt(Math.pow(x - point.x, 2) + Math.pow(y - point.y, 2));
      if (distance < minDistance) {
        minDistance = distance;
        closest = point;
      }
    });
    
    return closest;
  }

  // Update segment type between two points
  static updateSegmentType(curve: InflectionCurve, startId: string, type: CurveSegmentType): InflectionCurve {
    const segment = curve.segments.find(s => s.startPoint.id === startId);
    if (segment) {
      segment.type = type;
    }
    return curve;
  }

  // Export curve to LaTeX commands
  static exportToLatex(curve: InflectionCurve): string[] {
    const commands: string[] = [];
    
    curve.segments.forEach((segment, index) => {
      const points = this.interpolateSegment(segment, 100);
      
      if (segment.type === 'cubic' && segment.controlPoints) {
        // Use bezier curve command
        const cp1 = segment.controlPoints[0];
        const cp2 = segment.controlPoints[1];
        commands.push(
          `\\draw (${segment.startPoint.x},${segment.startPoint.y}) .. controls ` +
          `(${cp1.x},${cp1.y}) and (${cp2.x},${cp2.y}) .. ` +
          `(${segment.endPoint.x},${segment.endPoint.y});`
        );
      } else {
        // Use plot for other curve types
        const coords = points.map(p => `(${p.x.toFixed(2)},${p.y.toFixed(2)})`).join(' -- ');
        commands.push(`\\draw ${coords};`);
      }
    });
    
    // Add inflection point markers
    curve.inflectionPoints.forEach((point, index) => {
      commands.push(`\\fill (${point.x},${point.y}) circle (2pt);`);
    });
    
    return commands;
  }
}
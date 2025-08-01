import { CurvePoint } from '../types/gamaka';

export class CurveEngine {
  static smoothCurve(points: CurvePoint[], smoothingFactor: number = 0.3): CurvePoint[] {
    if (points.length < 3) return points;

    const smoothed: CurvePoint[] = [points[0]];

    for (let i = 1; i < points.length - 1; i++) {
      const prev = points[i - 1];
      const curr = points[i];
      const next = points[i + 1];

      const smoothedX = curr.x + (prev.x + next.x - 2 * curr.x) * smoothingFactor;
      const smoothedY = curr.y + (prev.y + next.y - 2 * curr.y) * smoothingFactor;

      smoothed.push({
        x: smoothedX,
        y: smoothedY,
        pressure: curr.pressure
      });
    }

    smoothed.push(points[points.length - 1]);
    return smoothed;
  }

  static drawBezierCurve(
    ctx: CanvasRenderingContext2D,
    points: CurvePoint[],
    color: string = '#000',
    baseWidth: number = 2,
    markers?: { start?: string; end?: string; type?: string }
  ) {
    if (points.length < 2) return;

    ctx.strokeStyle = color;
    ctx.lineCap = 'round';
    ctx.lineJoin = 'round';

    // Draw the curve
    for (let i = 0; i < points.length - 1; i++) {
      const p1 = points[i];
      const p2 = points[i + 1];

      ctx.beginPath();
      ctx.lineWidth = baseWidth * ((p1.pressure + p2.pressure) / 2);
      ctx.moveTo(p1.x, p1.y);

      if (i < points.length - 2) {
        const p3 = points[i + 2];
        const cp1x = p2.x - (p3.x - p1.x) * 0.15;
        const cp1y = p2.y - (p3.y - p1.y) * 0.15;
        const cp2x = p2.x + (p3.x - p1.x) * 0.15;
        const cp2y = p2.y + (p3.y - p1.y) * 0.15;

        ctx.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, p2.x, p2.y);
      } else {
        ctx.lineTo(p2.x, p2.y);
      }

      ctx.stroke();
    }

    // Draw markers if specified
    if (markers) {
      const startPoint = points[0];
      const endPoint = points[points.length - 1];
      
      // Draw start marker
      if (markers.start) {
        this.drawMarker(ctx, startPoint.x, startPoint.y, markers.start, 'start', color);
      }
      
      // Draw end marker
      if (markers.end) {
        const angle = this.getEndAngle(points);
        this.drawMarker(ctx, endPoint.x, endPoint.y, markers.end, 'end', color, angle);
      }
    }
  }

  static drawMarker(
    ctx: CanvasRenderingContext2D,
    x: number,
    y: number,
    type: string,
    position: 'start' | 'end',
    color: string,
    angle: number = 0
  ) {
    ctx.save();
    ctx.translate(x, y);
    ctx.rotate(angle);
    
    const size = 6;
    
    switch (type) {
      case 'circle':
        ctx.beginPath();
        ctx.arc(0, 0, size, 0, Math.PI * 2);
        ctx.fillStyle = position === 'start' ? 'white' : color;
        ctx.fill();
        ctx.strokeStyle = color;
        ctx.lineWidth = 2;
        ctx.stroke();
        break;
        
      case 'square':
        ctx.fillStyle = position === 'start' ? 'white' : color;
        ctx.fillRect(-size, -size, size * 2, size * 2);
        ctx.strokeStyle = color;
        ctx.lineWidth = 2;
        ctx.strokeRect(-size, -size, size * 2, size * 2);
        break;
        
      case 'diamond':
        ctx.beginPath();
        ctx.moveTo(0, -size);
        ctx.lineTo(size, 0);
        ctx.lineTo(0, size);
        ctx.lineTo(-size, 0);
        ctx.closePath();
        ctx.fillStyle = position === 'start' ? 'white' : color;
        ctx.fill();
        ctx.strokeStyle = color;
        ctx.lineWidth = 2;
        ctx.stroke();
        break;
        
      case 'triangle':
        ctx.beginPath();
        ctx.moveTo(0, -size);
        ctx.lineTo(size, size);
        ctx.lineTo(-size, size);
        ctx.closePath();
        ctx.fillStyle = position === 'start' ? 'white' : color;
        ctx.fill();
        ctx.strokeStyle = color;
        ctx.lineWidth = 2;
        ctx.stroke();
        break;
        
      case 'arrow':
        ctx.beginPath();
        ctx.moveTo(-size * 1.5, -size);
        ctx.lineTo(0, 0);
        ctx.lineTo(-size * 1.5, size);
        ctx.strokeStyle = color;
        ctx.lineWidth = 3;
        ctx.stroke();
        break;
    }
    
    ctx.restore();
  }

  static getEndAngle(points: CurvePoint[]): number {
    if (points.length < 2) return 0;
    
    const lastPoint = points[points.length - 1];
    const secondLastPoint = points[points.length - 2];
    
    return Math.atan2(
      lastPoint.y - secondLastPoint.y,
      lastPoint.x - secondLastPoint.x
    );
  }

  static snapToStaff(y: number, staffPositions: { y: number }[], threshold: number = 10): number {
    let closestY = y;
    let minDistance = threshold;

    staffPositions.forEach(pos => {
      const distance = Math.abs(y - pos.y);
      if (distance < minDistance) {
        minDistance = distance;
        closestY = pos.y;
      }
    });

    return closestY;
  }

  static simplifyPath(points: CurvePoint[], tolerance: number = 2): CurvePoint[] {
    if (points.length < 3) return points;

    const simplified: CurvePoint[] = [points[0]];
    let prevPoint = points[0];

    for (let i = 1; i < points.length - 1; i++) {
      const distance = Math.sqrt(
        Math.pow(points[i].x - prevPoint.x, 2) +
        Math.pow(points[i].y - prevPoint.y, 2)
      );

      if (distance >= tolerance) {
        simplified.push(points[i]);
        prevPoint = points[i];
      }
    }

    simplified.push(points[points.length - 1]);
    return simplified;
  }

  static interpolatePressure(pressure: number | undefined, defaultPressure: number = 0.5): number {
    return pressure !== undefined ? pressure : defaultPressure;
  }
}
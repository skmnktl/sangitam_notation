import React, { useState } from 'react';
import { InflectionCurveEditor } from './InflectionCurveEditor';
import { InflectionCurve, InflectionCurveEngine } from '../lib/inflection-curve-engine';

export const GamakaInflectionDemo: React.FC = () => {
  const [curves, setCurves] = useState<InflectionCurve[]>([]);
  const [selectedCurve, setSelectedCurve] = useState<InflectionCurve | null>(null);
  const [latexOutput, setLatexOutput] = useState<string>('');

  const handleCurveUpdate = (curve: InflectionCurve) => {
    setSelectedCurve(curve);
    
    // Update curves list
    setCurves(prev => {
      const existing = prev.find(c => c.id === curve.id);
      if (existing) {
        return prev.map(c => c.id === curve.id ? curve : c);
      } else {
        return [...prev, curve];
      }
    });

    // Generate LaTeX output
    const latexCommands = InflectionCurveEngine.exportToLatex(curve);
    setLatexOutput(latexCommands.join('\n'));
  };

  const createPredefinedGamaka = (type: string) => {
    let curve: InflectionCurve;
    
    switch (type) {
      case 'kampita':
        // Oscillating pattern
        curve = {
          id: `curve_kampita_${Date.now()}`,
          inflectionPoints: [
            InflectionCurveEngine.createInflectionPoint(50, 150),
            InflectionCurveEngine.createInflectionPoint(150, 100),
            InflectionCurveEngine.createInflectionPoint(250, 200),
            InflectionCurveEngine.createInflectionPoint(350, 150)
          ],
          segments: []
        };
        curve.segments = InflectionCurveEngine.buildSegments(curve.inflectionPoints);
        curve.segments.forEach(s => s.type = 'sinusoidal');
        break;
        
      case 'jaru':
        // Gliding pattern
        curve = {
          id: `curve_jaru_${Date.now()}`,
          inflectionPoints: [
            InflectionCurveEngine.createInflectionPoint(50, 200),
            InflectionCurveEngine.createInflectionPoint(200, 100),
            InflectionCurveEngine.createInflectionPoint(350, 100)
          ],
          segments: []
        };
        curve.segments = InflectionCurveEngine.buildSegments(curve.inflectionPoints);
        curve.segments[0].type = 'exponential';
        break;
        
      case 'andolita':
        // Swinging pattern
        curve = {
          id: `curve_andolita_${Date.now()}`,
          inflectionPoints: [
            InflectionCurveEngine.createInflectionPoint(50, 150),
            InflectionCurveEngine.createInflectionPoint(150, 100),
            InflectionCurveEngine.createInflectionPoint(250, 200),
            InflectionCurveEngine.createInflectionPoint(350, 120)
          ],
          segments: []
        };
        curve.segments = InflectionCurveEngine.buildSegments(curve.inflectionPoints);
        curve.segments.forEach(s => s.type = 'cubic');
        break;
        
      default:
        return;
    }
    
    setSelectedCurve(curve);
    handleCurveUpdate(curve);
  };

  return (
    <div style={{ padding: '20px' }}>
      <h2>Gamaka Inflection Curve Editor</h2>
      
      <div style={{ marginBottom: '20px' }}>
        <h3>Predefined Gamakas:</h3>
        <button onClick={() => createPredefinedGamaka('kampita')} style={{ marginRight: '10px' }}>
          Kampita (Oscillation)
        </button>
        <button onClick={() => createPredefinedGamaka('jaru')} style={{ marginRight: '10px' }}>
          Jaru (Glide)
        </button>
        <button onClick={() => createPredefinedGamaka('andolita')}>
          Andolita (Swing)
        </button>
      </div>
      
      <div style={{ display: 'flex', gap: '20px' }}>
        <div>
          <h3>Interactive Editor</h3>
          <InflectionCurveEditor
            width={600}
            height={300}
            onCurveUpdate={handleCurveUpdate}
            initialCurve={selectedCurve || undefined}
          />
        </div>
        
        <div style={{ flex: 1 }}>
          <h3>LaTeX Output</h3>
          <pre style={{
            background: '#f5f5f5',
            padding: '10px',
            borderRadius: '4px',
            fontSize: '12px',
            overflow: 'auto',
            maxHeight: '300px'
          }}>
            {latexOutput || 'Draw a curve to see LaTeX output...'}
          </pre>
          
          <h3>Curve Properties</h3>
          {selectedCurve && (
            <div style={{ fontSize: '14px' }}>
              <p>Inflection Points: {selectedCurve.inflectionPoints.length}</p>
              <p>Segments: {selectedCurve.segments.length}</p>
              <div>
                <strong>Segment Types:</strong>
                <ul>
                  {selectedCurve.segments.map((segment, index) => (
                    <li key={index}>
                      Segment {index + 1}: {segment.type}
                      {segment.tension && ` (tension: ${segment.tension.toFixed(2)})`}
                    </li>
                  ))}
                </ul>
              </div>
            </div>
          )}
        </div>
      </div>
      
      <div style={{ marginTop: '20px' }}>
        <h3>Features:</h3>
        <ul>
          <li>Click to add inflection points</li>
          <li>Drag points to adjust curve shape</li>
          <li>Right-click on segments to change curve type (linear, quadratic, cubic, sinusoidal, exponential)</li>
          <li>Each segment between inflection points can have a different curve type</li>
          <li>Press 'A' to toggle add point mode</li>
          <li>Press 'Delete' to remove selected point</li>
          <li>Press 'L' to lock/unlock selected point</li>
        </ul>
      </div>
    </div>
  );
};
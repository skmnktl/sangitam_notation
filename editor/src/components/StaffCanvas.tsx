import React, { useRef, useEffect, useState } from 'react';
import { StaffLayout, StaffPosition, BeatMarker } from '../types/canvas';
import { VNASection, ParsedSwara, Swara, OctaveMarker } from '../types/vna';
import { VNAParser } from '../lib/vna-parser-wasm';

interface StaffCanvasProps {
  section: VNASection;
  phraseIndex: number;
  width?: number;
  height?: number;
  onLayoutReady?: (layout: StaffLayout) => void;
  documentGati?: number;
  documentTala?: string;
}

export const StaffCanvas: React.FC<StaffCanvasProps> = ({
  section,
  phraseIndex,
  width = 800,
  height = 400,
  onLayoutReady,
  documentGati = 4,
  documentTala = "+234+0+0"
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [layout, setLayout] = useState<StaffLayout | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const phrase = section.phrases[phraseIndex];
    if (!phrase) return;

    const staffLayout = calculateLayout(phrase, width, height);
    setLayout(staffLayout);
    onLayoutReady?.(staffLayout);

    // Get effective gati and tala: phrase > section > document
    const effectiveGati = phrase.gati || 
                         (section.annotations?.gati ? parseInt(section.annotations.gati) : documentGati);
    const effectiveTala = phrase.tala || 
                         section.annotations?.tala || 
                         documentTala;
    
    drawStaff(ctx, staffLayout);
    drawGatiDivisions(ctx, staffLayout, phrase, effectiveGati);
    drawTalaIndicators(ctx, staffLayout, effectiveTala);
    drawSwaras(ctx, staffLayout, phrase, effectiveTala, effectiveGati);
    drawSahitya(ctx, staffLayout, phrase);

  }, [section, phraseIndex, width, height, onLayoutReady, documentGati, documentTala]);

  const calculateLayout = (phrase: { swaras: string[] | string; sahitya: string[] | string }, width: number, height: number): StaffLayout => {
    const marginTop = 80; // Back to normal since no tala bar
    const marginBottom = 60;
    const marginLeft = 30;
    const marginRight = 30;
    const lineSpacing = (height - marginTop - marginBottom) / 25; // Extended for octaves

    const staffPositions: StaffPosition[] = [
      { swara: "S''", y: marginTop },
      { swara: "R''", y: marginTop + lineSpacing },
      { swara: "G''", y: marginTop + lineSpacing * 2 },
      { swara: "P''", y: marginTop + lineSpacing * 3 },
      { swara: "D''", y: marginTop + lineSpacing * 4 },
      { swara: "N''", y: marginTop + lineSpacing * 5 },
      { swara: "S'", y: marginTop + lineSpacing * 6 },
      { swara: "R'", y: marginTop + lineSpacing * 7 },
      { swara: "G'", y: marginTop + lineSpacing * 8 },
      { swara: "P'", y: marginTop + lineSpacing * 9 },
      { swara: "D'", y: marginTop + lineSpacing * 10 },
      { swara: "N'", y: marginTop + lineSpacing * 11 },
      { swara: 'N', y: marginTop + lineSpacing * 12 },
      { swara: 'D', y: marginTop + lineSpacing * 13 },
      { swara: 'P', y: marginTop + lineSpacing * 14 },
      { swara: 'M', y: marginTop + lineSpacing * 15 },
      { swara: 'G', y: marginTop + lineSpacing * 16 },
      { swara: 'R', y: marginTop + lineSpacing * 17 },
      { swara: 'S', y: marginTop + lineSpacing * 18 },
      { swara: 'N.', y: marginTop + lineSpacing * 19 },
      { swara: 'D.', y: marginTop + lineSpacing * 20 },
      { swara: 'P.', y: marginTop + lineSpacing * 21 },
      { swara: 'M.', y: marginTop + lineSpacing * 22 },
      { swara: 'G.', y: marginTop + lineSpacing * 23 },
      { swara: 'R.', y: marginTop + lineSpacing * 24 },
      { swara: 'S.', y: marginTop + lineSpacing * 25 }
    ];

    const beatMarkers = calculateBeatMarkers(phrase.swaras, marginLeft, width - marginRight);

    return {
      width,
      height,
      marginTop,
      marginBottom,
      marginLeft,
      marginRight,
      lineSpacing,
      staffPositions,
      beatMarkers
    };
  };

  const calculateBeatMarkers = (swaraLine: string[] | string, startX: number, endX: number): BeatMarker[] => {
    const markers: BeatMarker[] = [];
    const tokens = Array.isArray(swaraLine) ? swaraLine : swaraLine.split(/\s+/);
    const totalWidth = endX - startX;
    const tokenSpacing = totalWidth / tokens.length;
    let noteTokenCount = 0; // Count only non-beat-marker tokens

    // Always add a marker at the start
    markers.push({ x: startX, type: 'start' });

    tokens.forEach((token, index) => {
      if (token === '|' || token === '||') {
        // Calculate x position based on the NEXT note token's first note
        // Look ahead to find the next non-beat-marker token
        let nextNoteTokenIndex = index + 1;
        while (nextNoteTokenIndex < tokens.length && (tokens[nextNoteTokenIndex] === '|' || tokens[nextNoteTokenIndex] === '||')) {
          nextNoteTokenIndex++;
        }
        
        if (nextNoteTokenIndex < tokens.length) {
          // Position at the start of the next token
          const x = startX + (nextNoteTokenIndex + 0.5) * tokenSpacing - (tokenSpacing * 0.4);
          markers.push({ x, type: token === '||' ? 'double' : 'single' });
        }
      } else {
        noteTokenCount++;
      }
    });

    // Add end marker if we have any markers
    if (markers.length > 0) {
      markers.push({ x: endX, type: 'end' });
    }

    console.log('Beat markers calculated:', markers);
    return markers;
  };

  const drawStaff = (ctx: CanvasRenderingContext2D, layout: StaffLayout) => {
    ctx.clearRect(0, 0, layout.width, layout.height);
    
    // Draw beat groups with alternating shading
    for (let i = 0; i < layout.beatMarkers.length - 1; i++) {
      const startMarker = layout.beatMarkers[i];
      const endMarker = layout.beatMarkers[i + 1];
      
      // Alternate shading for beat groups
      if (i % 2 === 0) {
        ctx.fillStyle = 'rgba(0, 0, 0, 0.02)';
        ctx.fillRect(startMarker.x, layout.marginTop, endMarker.x - startMarker.x, layout.height - layout.marginTop - layout.marginBottom);
      }
      
      // Draw beat group borders
      ctx.strokeStyle = 'rgba(0, 0, 0, 0.1)';
      ctx.lineWidth = 1;
      ctx.strokeRect(startMarker.x, layout.marginTop - 25, endMarker.x - startMarker.x, layout.height - layout.marginTop - layout.marginBottom + 25);
    }
    
    ctx.strokeStyle = '#666';
    ctx.lineWidth = 1;

    // Draw only the main 7 staff lines (S R G M P D N)
    const mainStaffIndices = [12, 13, 14, 15, 16, 17, 18]; // N D P M G R S
    mainStaffIndices.forEach(i => {
      const y = layout.staffPositions[i].y;
      ctx.beginPath();
      ctx.moveTo(layout.marginLeft, y);
      ctx.lineTo(layout.width - layout.marginRight, y);
      ctx.stroke();
    });

    // Draw beat markers with tala indicators
    layout.beatMarkers.forEach((marker, idx) => {
      if (marker.type === 'start') {
        // Don't draw start marker line
        return;
      } else if (marker.type === 'double') {
        // Strong beat - darker and thicker
        ctx.strokeStyle = '#444';
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.moveTo(marker.x, layout.marginTop);
        ctx.lineTo(marker.x, layout.height - layout.marginBottom);
        ctx.stroke();
        
        ctx.beginPath();
        ctx.moveTo(marker.x + 3, layout.marginTop);
        ctx.lineTo(marker.x + 3, layout.height - layout.marginBottom);
        ctx.stroke();
        
        // Add tala mark at top
        ctx.fillStyle = '#444';
        ctx.font = 'bold 14px sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText('||', marker.x + 1.5, layout.marginTop - 10);
      } else {
        // Regular beat - lighter
        ctx.strokeStyle = '#999';
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.moveTo(marker.x, layout.marginTop);
        ctx.lineTo(marker.x, layout.height - layout.marginBottom);
        ctx.stroke();
        
        // Add tala mark at top
        ctx.fillStyle = '#666';
        ctx.font = '12px sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText('|', marker.x, layout.marginTop - 10);
      }
    });
  };

  const drawSwaras = (ctx: CanvasRenderingContext2D, layout: StaffLayout, phrase: { swaras: string[] | string }, talaPattern: string, gati: number) => {
    const tokens = Array.isArray(phrase.swaras) ? phrase.swaras : phrase.swaras.split(/\s+/);
    const startX = layout.marginLeft;
    const endX = layout.width - layout.marginRight;
    const totalWidth = endX - startX;
    
    console.log('Drawing swaras for phrase:', phrase.swaras);
    console.log('Tokens:', tokens);
    
    // First, expand compound tokens and calculate total note count
    const expandedNotes: { swara: ParsedSwara; originalTokenIndex: number; indexInToken: number; isFirstInBeat: boolean }[] = [];
    let beatStarted = true;
    let lastActualSwara: ParsedSwara | null = null;
    
    // Parse tala pattern
    const talaBeats = parseTalaPattern(talaPattern);
    let talaIndex = 0;
    
    // Track token positions for tala indicators
    const tokenTalaIndicators: { tokenIndex: number; talaType: string }[] = [];
    
    tokens.forEach((token, tokenIndex) => {
      if (token === '|' || token === '||') {
        beatStarted = true;
        return;
      }
      
      // Assign tala indicator to this token (beat)
      if (talaIndex < talaBeats.length) {
        tokenTalaIndicators.push({
          tokenIndex,
          talaType: talaBeats[talaIndex]
        });
        talaIndex = (talaIndex + 1) % talaBeats.length;
      }
      
      if (token === ',') {
        // For comma, use the previous swara but mark as sustain
        if (lastActualSwara) {
          expandedNotes.push({
            swara: { ...lastActualSwara, isSustain: true },
            originalTokenIndex: tokenIndex,
            indexInToken: 0,
            isFirstInBeat: false
          });
        }
        return;
      }
      
      const parsedSwaras = VNAParser.parseSwaraLine(token);
      console.log(`Token "${token}" parsed to:`, parsedSwaras);
      parsedSwaras.forEach((swara, swaraIndex) => {
        // Update last actual swara if this is not a rest or sustain
        if (!swara.isRest && !swara.isSustain) {
          lastActualSwara = swara;
        }
        
        // Handle sustain markers within compound tokens
        if (swara.isSustain && lastActualSwara) {
          expandedNotes.push({
            swara: { ...lastActualSwara, isSustain: true },
            originalTokenIndex: tokenIndex,
            indexInToken: swaraIndex,
            isFirstInBeat: beatStarted && swaraIndex === 0
          });
        } else {
          expandedNotes.push({
            swara,
            originalTokenIndex: tokenIndex,
            indexInToken: swaraIndex,
            isFirstInBeat: beatStarted && swaraIndex === 0
          });
        }
        
        if (swaraIndex === 0 && !swara.isRest && !swara.isSustain) {
          beatStarted = false;
        }
      });
    });
    
    console.log('Total expanded notes:', expandedNotes.length);
    console.log('Expanded notes:', expandedNotes);
    console.log('Token tala indicators:', tokenTalaIndicators);

    // Calculate spacing based on total tokens (including beat markers)
    const tokenSpacing = totalWidth / tokens.length;

    ctx.font = '16px sans-serif';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    
    let prevNotePosition: { x: number, y: number } | null = null;
    
    expandedNotes.forEach((note, noteIndex) => {
      const { swara, originalTokenIndex, indexInToken, isFirstInBeat } = note;
      
      // Calculate position within the token space
      const tokenX = startX + (originalTokenIndex + 0.5) * tokenSpacing;
      const tokenWidth = tokenSpacing * 0.8; // Leave some space between tokens
      
      // If multiple notes in token, distribute them within the token space
      const notesInThisToken = expandedNotes.filter(n => n.originalTokenIndex === originalTokenIndex).length;
      const noteOffset = notesInThisToken > 1 
        ? (indexInToken - (notesInThisToken - 1) / 2) * (tokenWidth / notesInThisToken)
        : 0;
      
      const x = tokenX + noteOffset;
      const staffPos = getStaffYPosition(layout, swara);
      
      if (!swara.isRest) {
        if (swara.isSustain && prevNotePosition) {
          // Draw sustain line from previous note
          ctx.strokeStyle = '#0066cc';
          ctx.lineWidth = 3;
          ctx.beginPath();
          ctx.moveTo(prevNotePosition.x + 5, prevNotePosition.y);
          ctx.lineTo(x - 5, staffPos);
          ctx.stroke();
          
          // Draw a small circle at the end of sustain
          ctx.fillStyle = '#0066cc';
          ctx.beginPath();
          ctx.arc(x, staffPos, 3, 0, Math.PI * 2);
          ctx.fill();
          
          // Update previous position for next sustain
          prevNotePosition = { x, y: staffPos };
        } else {
          // Regular note
          // Color first note in beat differently
          ctx.fillStyle = isFirstInBeat ? '#0066cc' : '#000';
          
          // Draw swara notation
          let notation = swara.swara;
          if (swara.variant) notation += swara.variant;
          if (swara.octave === '..') notation = notation + '..';
          else if (swara.octave === '.') notation = notation + '.';
          else if (swara.octave === "'") notation = notation + "'";
          else if (swara.octave === "''") notation = notation + "''";
          
          ctx.fillText(notation, x, staffPos - 20);
          
          // Draw dot on staff
          ctx.beginPath();
          ctx.arc(x, staffPos, isFirstInBeat ? 5 : 4, 0, Math.PI * 2);
          ctx.fill();
          
          // Draw ledger lines for this specific note if outside main staff
          const noteStaffIndex = layout.staffPositions.findIndex(pos => pos.y === staffPos);
          if (noteStaffIndex !== -1 && (noteStaffIndex < 12 || noteStaffIndex > 18)) {
            ctx.strokeStyle = '#999';
            ctx.lineWidth = 1;
            ctx.beginPath();
            ctx.moveTo(x - 15, staffPos);
            ctx.lineTo(x + 15, staffPos);
            ctx.stroke();
          }
          
          // Store position for potential sustain
          prevNotePosition = { x, y: staffPos };
        }
      } else {
        // Rest - reset previous note position
        prevNotePosition = null;
      }
    });
    
    // Draw tala indicators for each token (beat)
    tokenTalaIndicators.forEach(({ tokenIndex, talaType }) => {
      // Find the first note of this token to get its position
      const firstNoteOfToken = expandedNotes.find(n => n.originalTokenIndex === tokenIndex);
      if (!firstNoteOfToken) return;
      
      // Calculate the x position of the first note in the token
      const tokenX = startX + (tokenIndex + 0.5) * tokenSpacing;
      const tokenWidth = tokenSpacing * 0.8;
      const notesInThisToken = expandedNotes.filter(n => n.originalTokenIndex === tokenIndex).length;
      const firstNoteOffset = notesInThisToken > 1 
        ? (0 - (notesInThisToken - 1) / 2) * (tokenWidth / notesInThisToken)
        : 0;
      const x = tokenX + firstNoteOffset;
      
      // Get the y position of the first note
      const staffPos = getStaffYPosition(layout, firstNoteOfToken.swara);
      const indicatorY = staffPos + 30; // Position below the note's dot
      
      ctx.save();
      ctx.font = '14px sans-serif';
      ctx.textAlign = 'center';
      
      // Draw white background circle for visibility
      ctx.fillStyle = 'white';
      ctx.beginPath();
      ctx.arc(x, indicatorY, 12, 0, Math.PI * 2);
      ctx.fill();
      
      if (talaType === '+') {
        ctx.fillStyle = '#dc3545';
        ctx.font = 'bold 16px sans-serif';
        ctx.fillText('+', x, indicatorY + 5);
      } else if (talaType === '0') {
        ctx.strokeStyle = '#0066cc';
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.arc(x, indicatorY, 6, 0, Math.PI, true);
        ctx.stroke();
      } else {
        ctx.fillStyle = '#f39c12';
        ctx.font = 'bold 14px sans-serif';
        ctx.fillText(talaType, x, indicatorY + 4);
      }
      
      ctx.restore();
    });
  };

  const drawSahitya = (ctx: CanvasRenderingContext2D, layout: StaffLayout, phrase: { sahitya: string[] | string, swaras: string[] | string }) => {
    const swaraTokens = Array.isArray(phrase.swaras) ? phrase.swaras : phrase.swaras.split(/\s+/);
    const sahityaTokens = Array.isArray(phrase.sahitya) ? phrase.sahitya : phrase.sahitya.split(/\s+/);
    const startX = layout.marginLeft;
    const endX = layout.width - layout.marginRight;
    const totalWidth = endX - startX;
    const tokenSpacing = totalWidth / swaraTokens.length;

    ctx.font = '14px monospace';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'top';

    // Process each token pair according to VNA spec
    let sahityaIndex = 0;
    let beatStarted = true;
    
    swaraTokens.forEach((swaraToken, tokenIndex) => {
      if (swaraToken === '|' || swaraToken === '||') {
        beatStarted = true;
        return; // Skip beat markers
      }
      
      if (sahityaIndex < sahityaTokens.length) {
        const sahityaToken = sahityaTokens[sahityaIndex];
        const tokenX = startX + (tokenIndex + 0.5) * tokenSpacing;
        const tokenWidth = tokenSpacing * 0.8;
        
        // Parse the swara token to get individual notes
        const swaraNotes = VNAParser.parseSwaraLine(swaraToken);
        const sahityaChars = Array.from(sahityaToken);
        
        // Calculate x positions for each note (same logic as drawSwaras)
        const notesInThisToken = swaraNotes.length;
        
        // Remove backtick marks from sahitya before displaying
        const cleanSahityaToken = sahityaToken.replace(/`/g, '');
        const cleanSahityaChars = Array.from(cleanSahityaToken);
        
        if (swaraNotes.length === cleanSahityaChars.length && swaraNotes.length > 0) {
          // Align each sahitya character with its corresponding note position
          cleanSahityaChars.forEach((char, idx) => {
            const noteOffset = notesInThisToken > 1 
              ? (idx - (notesInThisToken - 1) / 2) * (tokenWidth / notesInThisToken)
              : 0;
            
            const x = tokenX + noteOffset;
            ctx.fillStyle = beatStarted && idx === 0 ? '#0066cc' : '#444';
            ctx.fillText(char, x, layout.height - layout.marginBottom + 10);
          });
        } else {
          // Fallback: center the whole token
          ctx.fillStyle = beatStarted ? '#0066cc' : '#444';
          ctx.fillText(cleanSahityaToken, tokenX, layout.height - layout.marginBottom + 10);
        }
        
        if (swaraToken !== ',' && swaraToken !== '-') {
          beatStarted = false;
        }
        sahityaIndex++;
      }
    });
  };

  const getStaffYPosition = (layout: StaffLayout, swara: ParsedSwara): number => {
    // Find the matching staff position
    let searchSwara = swara.swara;
    if (swara.octave === '.') {
      searchSwara = swara.swara + '.';
    } else if (swara.octave === "'") {
      searchSwara = swara.swara + "'";
    } else if (swara.octave === "''") {
      searchSwara = swara.swara + "''";
    } else if (swara.octave === "..") {
      searchSwara = swara.swara + "..";
    }
    
    const position = layout.staffPositions.findIndex(pos => pos.swara === searchSwara);
    if (position !== -1) {
      return layout.staffPositions[position].y;
    }
    
    // Default to middle octave S if not found
    return layout.staffPositions[18].y;
  };

  const drawTalaIndicators = (ctx: CanvasRenderingContext2D, layout: StaffLayout, talaPattern: string) => {
    // This function is now called from drawSwaras to align with actual notes
    // Show tala pattern label at the start
    ctx.font = 'italic 11px sans-serif';
    ctx.fillStyle = '#666';
    ctx.textAlign = 'left';
    ctx.fillText(`Tala: ${talaPattern}`, layout.marginLeft, layout.marginTop - 35);
  };
  

  const parseTalaPattern = (pattern: string): string[] => {
    const beats: string[] = [];
    let i = 0;
    
    while (i < pattern.length) {
      const char = pattern[i];
      if (char === '+') {
        beats.push('+');
        i++;
      } else if (char === '0') {
        beats.push('0');
        i++;
      } else if (char >= '2' && char <= '9') {
        // Direct finger count number
        beats.push(char);
        i++;
      } else {
        i++; // Skip invalid characters
      }
    }
    
    return beats;
  };

  const drawGatiDivisions = (ctx: CanvasRenderingContext2D, layout: StaffLayout, phrase: { swaras: string[] | string }, gati: number) => {
    const tokens = Array.isArray(phrase.swaras) ? phrase.swaras : phrase.swaras.split(/\s+/);
    
    // Calculate positions for each beat
    for (let i = 0; i < layout.beatMarkers.length - 1; i++) {
      const startMarker = layout.beatMarkers[i];
      const endMarker = layout.beatMarkers[i + 1];
      const beatWidth = endMarker.x - startMarker.x;
      
      // Draw vertical lines for gati divisions within each beat
      ctx.strokeStyle = 'rgba(0, 0, 0, 0.15)';
      ctx.lineWidth = 1;
      ctx.setLineDash([2, 2]); // Dashed line
      
      for (let j = 1; j < gati; j++) {
        const x = startMarker.x + (beatWidth / gati) * j;
        ctx.beginPath();
        ctx.moveTo(x, layout.marginTop);
        ctx.lineTo(x, layout.height - layout.marginBottom);
        ctx.stroke();
      }
      
      // Draw gati label for this beat
      ctx.setLineDash([]); // Reset to solid lines
      ctx.font = 'italic 11px sans-serif';
      ctx.fillStyle = '#666';
      ctx.textAlign = 'center';
      const centerX = (startMarker.x + endMarker.x) / 2;
      ctx.fillText(`gati: ${gati}`, centerX, layout.marginTop - 35);
    }
    
    ctx.setLineDash([]); // Reset to solid lines
  };

  return (
    <canvas
      ref={canvasRef}
      width={width}
      height={height}
      style={{ border: '1px solid #ddd', borderRadius: '4px' }}
    />
  );
};
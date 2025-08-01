import React, { useState, useCallback } from 'react';
import { VNADocument, VNASection } from '../types/vna';
import { Gamaka } from '../types/gamaka';
import { StaffLayout } from '../types/canvas';
import { StaffCanvas } from './StaffCanvas';
import { CurveEditor } from './CurveEditor';
import { VNAParser } from '../lib/vna-parser-wasm';

interface VNAEditorProps {
  vnaContent?: string;
}

interface PhraseEditorProps {
  section: VNASection;
  sectionIndex: number;
  phraseIndex: number;
  gamakas: Gamaka[];
  onGamakaAdd: (gamaka: Gamaka) => void;
  onGamakaUpdate: (gamaka: Gamaka) => void;
  documentGati?: number;
  documentTala?: string;
}

const PhraseEditor: React.FC<PhraseEditorProps> = ({
  section,
  sectionIndex,
  phraseIndex,
  gamakas,
  onGamakaAdd,
  onGamakaUpdate,
  documentGati,
  documentTala
}) => {
  const [staffLayout, setStaffLayout] = useState<StaffLayout | null>(null);

  const handleLayoutReady = useCallback((layout: StaffLayout) => {
    setStaffLayout(layout);
  }, []);

  const relevantGamakas = gamakas.filter(
    g => g.sectionName === section.name && g.phraseIndex === phraseIndex
  );

  return (
    <div className="phrase-editor">
      <div className="canvas-container">
        <StaffCanvas
          section={section}
          phraseIndex={phraseIndex}
          width={800}
          height={400}
          onLayoutReady={handleLayoutReady}
          documentGati={documentGati}
          documentTala={documentTala}
        />
        {staffLayout && (
          <CurveEditor
            layout={staffLayout}
            gamakas={relevantGamakas}
            onGamakaAdd={onGamakaAdd}
            onGamakaUpdate={onGamakaUpdate}
            sectionName={section.name}
            phraseIndex={phraseIndex}
          />
        )}
      </div>
    </div>
  );
};

export const VNAEditor: React.FC<VNAEditorProps> = ({ vnaContent }) => {
  const [vnaDocument, setVnaDocument] = useState<VNADocument | null>(null);
  const [gamakas, setGamakas] = useState<Gamaka[]>([]);
  const [error, setError] = useState<string | null>(null);

  React.useEffect(() => {
    if (vnaContent) {
      VNAParser.parse(vnaContent)
        .then(doc => {
          setVnaDocument(doc);
          setError(null);
        })
        .catch(err => {
          setError(err instanceof Error ? err.message : 'Failed to parse VNA file');
        });
    }
  }, [vnaContent]);

  const handleGamakaAdd = useCallback((gamaka: Gamaka) => {
    setGamakas(prev => [...prev, gamaka]);
  }, []);

  const handleGamakaUpdate = useCallback((updatedGamaka: Gamaka) => {
    setGamakas(prev => 
      prev.map(g => g.id === updatedGamaka.id ? updatedGamaka : g)
    );
  }, []);

  if (error) {
    return (
      <div className="error-container">
        <h3>Error Loading VNA File</h3>
        <p>{error}</p>
      </div>
    );
  }

  if (!vnaDocument) {
    return (
      <div className="loading-container">
        <p>No VNA file loaded. Please select a file to edit.</p>
      </div>
    );
  }

  return (
    <div className="vna-editor">
      <div className="editor-header">
        <h2>{vnaDocument.metadata.title}</h2>
        <div className="metadata">
          <span>Raga: {vnaDocument.metadata.raga}</span>
          <span>Tala: {vnaDocument.metadata.tala}</span>
          {vnaDocument.metadata.tempo && <span>Tempo: {vnaDocument.metadata.tempo} BPM</span>}
          {vnaDocument.metadata.composer && <span>Composer: {vnaDocument.metadata.composer}</span>}
        </div>
      </div>

      <div className="sections-container">
        {vnaDocument.sections.map((section, sectionIdx) => (
          <div key={sectionIdx} className="section">
            <h3 className="section-title">[{section.name}]</h3>
            {section.comments && section.comments.map((comment, idx) => (
              <p key={idx} className="section-comment"># {comment}</p>
            ))}
            <div className="phrases-container">
              {section.phrases.map((phrase, phraseIdx) => (
                <PhraseEditor
                  key={`${sectionIdx}-${phraseIdx}`}
                  section={section}
                  sectionIndex={sectionIdx}
                  phraseIndex={phraseIdx}
                  gamakas={gamakas}
                  onGamakaAdd={handleGamakaAdd}
                  onGamakaUpdate={handleGamakaUpdate}
                  documentGati={vnaDocument.metadata.gati}
                  documentTala={vnaDocument.metadata.tala}
                />
              ))}
            </div>
          </div>
        ))}
      </div>

      <div className="controls">
        <button onClick={() => console.log('Export PDF')}>Export PDF</button>
        <button onClick={() => console.log('Save Curves')}>Save Curves</button>
      </div>
    </div>
  );
};
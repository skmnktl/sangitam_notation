// Adapter to make WASM parser data compatible with existing components
import { VNADocument, VNAPhrase } from '../types/vna';

export function normalizeVNADocument(doc: any): VNADocument {
  return {
    ...doc,
    sections: doc.sections.map((section: any) => ({
      ...section,
      phrases: section.phrases.map((phrase: any) => normalizePhrase(phrase))
    }))
  };
}

function normalizePhrase(phrase: any): VNAPhrase {
  // Convert arrays to space-separated strings if needed
  const swaras = Array.isArray(phrase.swaras) 
    ? phrase.swaras.join(' ')
    : phrase.swaras;
    
  const sahitya = Array.isArray(phrase.sahitya)
    ? phrase.sahitya.join(' ')
    : phrase.sahitya;
    
  return {
    ...phrase,
    swaras,
    sahitya
  };
}
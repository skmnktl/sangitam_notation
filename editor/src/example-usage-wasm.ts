import { 
  parseVna, 
  validateVna, 
  formatVna, 
  getMetadataFields,
  getSectionNames,
  getSwaraTokens,
  initializeWasm
} from './wasm-parser';

// Initialize WASM module on startup
initializeWasm().then(() => {
  console.log('VNA WASM parser initialized');
}).catch(error => {
  console.error('Failed to initialize WASM:', error);
});

// Example 1: Parse and validate on every keystroke (debounced)
async function onDocumentChange(content: string) {
  try {
    // Parse the document
    const document = await parseVna(content);
    
    // Get validation issues
    const issues = await validateVna(content);
    
    // Update editor diagnostics
    updateDiagnostics(issues);
    
    // Update document structure for outline view
    updateOutline(document);
    
  } catch (error) {
    // Handle parsing errors
    console.error('Parse error:', error);
  }
}

// Example 2: Provide autocomplete suggestions
async function getCompletions(line: string, position: number) {
  const suggestions = [];
  
  // If at start of line, suggest section headers
  if (line.trim() === '') {
    const sections = await getSectionNames();
    sections.forEach(section => {
      suggestions.push({
        label: `[${section}]`,
        kind: 'Section',
        insertText: `[${section}]`
      });
    });
  }
  
  // If in metadata section, suggest metadata fields
  if (position < 10) { // rough check for metadata area
    const fields = await getMetadataFields();
    fields.forEach(([name, required]) => {
      suggestions.push({
        label: name,
        kind: 'Property',
        insertText: `${name}: "$1"`,
        detail: required ? 'required' : 'optional'
      });
    });
  }
  
  // Suggest swara tokens
  const swaraTokens = await getSwaraTokens();
  swaraTokens.forEach(swara => {
    suggestions.push({
      label: swara,
      kind: 'Swara',
      insertText: swara
    });
  });
  
  return suggestions;
}

// Example 3: Format document
async function formatDocument(content: string): Promise<string> {
  try {
    return await formatVna(content);
  } catch (error) {
    // Return original if formatting fails
    console.error('Format error:', error);
    return content;
  }
}

// Example 4: Hover information
async function getHoverInfo(content: string, line: number, column: number) {
  const lines = content.split('\n');
  const currentLine = lines[line] || '';
  const word = getWordAt(currentLine, column);
  
  // Provide hover info for swaras
  const swaraInfo: Record<string, string> = {
    'S': 'Shadja - The tonic note',
    'R': 'Rishabha - The second note',
    'G': 'Gandhara - The third note',
    'M': 'Madhyama - The fourth note',
    'P': 'Panchama - The fifth note',
    'D': 'Dhaivata - The sixth note',
    'N': 'Nishada - The seventh note',
  };
  
  if (word in swaraInfo) {
    return {
      contents: swaraInfo[word]
    };
  }
  
  // Provide hover info for sections
  const sectionInfo: Record<string, string> = {
    'pallavi': 'The main theme or refrain of the composition',
    'anupallavi': 'The second section that provides contrast',
    'charanam': 'Verse section with different lyrics',
    'cittasvaras': 'Intricate swara passages',
  };
  
  const sectionMatch = currentLine.match(/\[(\w+)\]/);
  if (sectionMatch && sectionMatch[1] in sectionInfo) {
    return {
      contents: sectionInfo[sectionMatch[1]]
    };
  }
  
  return null;
}

// Helper functions (would be implemented elsewhere)
function updateDiagnostics(issues: any[]) {
  // Update editor diagnostics
  console.log('Validation issues:', issues);
}

function updateOutline(document: any) {
  // Update document outline
  console.log('Document structure:', document);
}

function getWordAt(line: string, column: number): string {
  // Simple implementation - get word at position
  const before = line.substring(0, column);
  const after = line.substring(column);
  
  const beforeMatch = before.match(/(\w+)$/);
  const afterMatch = after.match(/^(\w+)/);
  
  const beforePart = beforeMatch ? beforeMatch[1] : '';
  const afterPart = afterMatch ? afterMatch[1] : '';
  
  return beforePart + afterPart;
}

// Example 5: Real-time validation with debouncing
let validationTimeout: NodeJS.Timeout | null = null;

function validateWithDebounce(content: string) {
  if (validationTimeout) {
    clearTimeout(validationTimeout);
  }
  
  validationTimeout = setTimeout(async () => {
    await onDocumentChange(content);
  }, 500); // 500ms debounce
}

// Export for use in extension
export {
  onDocumentChange,
  getCompletions,
  formatDocument,
  getHoverInfo,
  validateWithDebounce
};
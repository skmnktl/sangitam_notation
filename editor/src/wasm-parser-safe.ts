import init, { VnaParser } from './pkg/vna.js';

let wasmInitialized = false;
let parserInstance: VnaParser | null = null;

/**
 * Initialize the WASM module and create parser instance
 */
export async function initializeWasm(): Promise<void> {
  if (!wasmInitialized) {
    try {
      await init();
      parserInstance = new VnaParser();
      wasmInitialized = true;
    } catch (error) {
      console.error('Failed to initialize WASM:', error);
      throw error;
    }
  }
}

/**
 * Get the parser instance, initializing if necessary
 */
export async function getParser(): Promise<VnaParser> {
  if (!parserInstance) {
    await initializeWasm();
  }
  return parserInstance!;
}

/**
 * Parse VNA content with error handling
 */
export async function parseVna(content: string): Promise<any> {
  try {
    const parser = await getParser();
    
    // Check content size
    if (content.length > 100000) { // 100KB limit
      throw new Error('VNA file too large');
    }
    
    // Try to parse
    const result = parser.parse(content);
    return result;
  } catch (error: any) {
    console.error('Parse error:', error);
    
    // If WASM error, try to reinitialize
    if (error.message?.includes('memory') || error.message?.includes('bounds')) {
      wasmInitialized = false;
      parserInstance = null;
      
      // Try once more
      try {
        await initializeWasm();
        const parser = await getParser();
        return parser.parse(content);
      } catch (retryError) {
        console.error('Retry failed:', retryError);
        throw new Error('Parser memory error. Please refresh the page.');
      }
    }
    
    throw error;
  }
}

/**
 * Validate VNA content
 */
export async function validateVna(content: string): Promise<any[]> {
  try {
    const parser = await getParser();
    return parser.validate(content);
  } catch (error) {
    console.error('Validation error:', error);
    return [];
  }
}

/**
 * Format VNA content
 */
export async function formatVna(content: string): Promise<string> {
  try {
    const parser = await getParser();
    return parser.format(content);
  } catch (error) {
    console.error('Format error:', error);
    return content; // Return original on error
  }
}

/**
 * Get metadata fields
 */
export async function getMetadataFields(): Promise<Array<[string, boolean]>> {
  const parser = await getParser();
  return parser.get_metadata_fields();
}

/**
 * Get valid section names
 */
export async function getSectionNames(): Promise<string[]> {
  const parser = await getParser();
  return parser.get_section_names();
}

/**
 * Get swara tokens
 */
export async function getSwaraTokens(): Promise<string[]> {
  const parser = await getParser();
  return parser.get_swara_tokens();
}

/**
 * Parse a single line
 */
export async function parseLine(line: string): Promise<string[]> {
  const parser = await getParser();
  return parser.parse_line(line);
}
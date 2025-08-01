import init, { VnaParser } from './pkg/vna.js';

let wasmInitialized = false;
let parserInstance: VnaParser | null = null;

/**
 * Initialize the WASM module and create parser instance
 */
export async function initializeWasm(): Promise<void> {
  if (!wasmInitialized) {
    await init();
    parserInstance = new VnaParser();
    wasmInitialized = true;
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
 * Parse VNA content
 */
export async function parseVna(content: string): Promise<any> {
  const parser = await getParser();
  return parser.parse(content);
}

/**
 * Validate VNA content
 */
export async function validateVna(content: string): Promise<any[]> {
  const parser = await getParser();
  return parser.validate(content);
}

/**
 * Format VNA content
 */
export async function formatVna(content: string): Promise<string> {
  const parser = await getParser();
  return parser.format(content);
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
import { useState, useEffect } from 'react'
import './App.css'
import './styles/editor.css'
import { VNAEditor } from './components/VNAEditor'
import { initializeWasm } from './wasm-parser'

function App() {
  const [vnaContent, setVnaContent] = useState<string>('');
  const [fileInput, setFileInput] = useState<HTMLInputElement | null>(null);
  const [wasmReady, setWasmReady] = useState(false);
  const [loading, setLoading] = useState(true);
  const [availableFiles] = useState([
    'ninnukori_mohanam.vna'
  ]);
  const [selectedFile, setSelectedFile] = useState('ninnukori_mohanam.vna');

  const handleFileLoad = (event: Event) => {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (e) => {
        const content = e.target?.result as string;
        setVnaContent(content);
      };
      reader.readAsText(file);
    }
  };

  useEffect(() => {
    // Initialize WASM and load default VNA file
    Promise.all([
      initializeWasm(),
      fetch('/data/ninnukori_mohanam.vna').then(res => res.text())
    ]).then(([_, content]) => {
      setWasmReady(true);
      setVnaContent(content);
      setLoading(false);
      console.log('VNA WASM parser initialized and file loaded');
    }).catch(error => {
      console.error('Failed to initialize:', error);
      setLoading(false);
    });
  }, []);

  useEffect(() => {
    if (fileInput) {
      fileInput.addEventListener('change', handleFileLoad);
      return () => fileInput.removeEventListener('change', handleFileLoad);
    }
  }, [fileInput]);

  return (
    <div className="app">
      <div className="file-controls">
        <input
          ref={setFileInput}
          type="file"
          accept=".vna"
          style={{ display: 'none' }}
          id="file-input"
        />
        <label htmlFor="file-input" className="file-label">
          Load VNA File
        </label>
        <select 
          value={selectedFile}
          onChange={(e) => {
            const fileName = e.target.value;
            setSelectedFile(fileName);
            fetch(`/data/${fileName}`)
              .then(res => res.text())
              .then(content => {
                setVnaContent(content);
                console.log(`Loaded ${fileName}`);
              })
              .catch(err => console.error('Failed to load file:', err));
          }}
        >
          {availableFiles.map(file => (
            <option key={file} value={file}>{file}</option>
          ))}
        </select>
      </div>
      {loading ? (
        <div>Loading VNA parser and file...</div>
      ) : wasmReady && vnaContent ? (
        <VNAEditor vnaContent={vnaContent} />
      ) : (
        <div>Error loading VNA file</div>
      )}
    </div>
  )
}

export default App
// LaTeX PDF Generator for Gamaka Notation
import { exec } from 'child_process';
import { promises as fs } from 'fs';
import path from 'path';

export class GamakaLatexGenerator {
    constructor() {
        this.outputDir = './pdf-output';
        
        // Define gamaka marker styles
        this.gamakaStyles = {
            kampita: {
                color: 'blue',
                startMarker: 'circle',
                endMarker: 'circle',
                startFill: 'white',
                endFill: 'black'
            },
            jaru: {
                color: 'red',
                startMarker: 'square',
                endMarker: 'arrow',
                startFill: 'white'
            },
            andolita: {
                color: 'green!60!black',
                startMarker: 'diamond',
                endMarker: 'diamond',
                startFill: 'white',
                endFill: 'black'
            },
            tribhinna: {
                color: 'purple',
                startMarker: 'triangle',
                endMarker: 'arrow',
                startFill: 'white',
                intermediateMarkers: true
            },
            default: {
                color: 'black',
                startMarker: 'circle',
                endMarker: 'arrow',
                startFill: 'white'
            }
        };
    }

    // Generate Bezier curve control points for smooth gamaka
    generateBezierControlPoints(notes) {
        const points = [];
        for (let i = 0; i < notes.length - 1; i++) {
            const note1 = notes[i];
            const note2 = notes[i + 1];
            
            // Calculate control points based on note properties
            const dx = note2.position.x - note1.position.x;
            const dy = note2.position.y - note1.position.y;
            
            // Adjust control points based on gamaka type
            const cp1 = {
                x: note1.position.x + dx * 0.3,
                y: note1.position.y + dy * 0.5 + (note1.gamaka?.amplitude || 0)
            };
            const cp2 = {
                x: note2.position.x - dx * 0.3,
                y: note2.position.y - dy * 0.5 + (note2.gamaka?.amplitude || 0)
            };
            
            points.push({
                start: note1.position,
                cp1,
                cp2,
                end: note2.position,
                style: note1.gamaka?.style || 'default'
            });
        }
        return points;
    }

    // Convert note data to LaTeX TikZ commands
    generateTikzCommands(noteGroup) {
        const commands = [];
        const bezierPoints = this.generateBezierControlPoints(noteGroup.notes);
        
        // Define gamaka curve styles with markers
        commands.push('% Define gamaka styles');
        commands.push(this.generateGamakaStyleDefinitions());
        commands.push('');
        
        // Staff lines
        commands.push('% Staff lines');
        for (let i = 0; i < 5; i++) {
            commands.push(`\\draw[gray,thin] (0,${i * 0.5}) -- (10,${i * 0.5});`);
        }
        
        // Notes
        commands.push('\\n% Notes');
        noteGroup.notes.forEach((note, idx) => {
            commands.push(`\\coordinate (n${idx}) at (${note.position.x},${note.position.y});`);
            commands.push(`\\fill (n${idx}) circle (0.1);`);
            commands.push(`\\node[below] at (n${idx}) {${note.swara}};`);
        });
        
        // Gamaka curves with markers
        commands.push('\\n% Gamaka curves with markers');
        bezierPoints.forEach((curve, idx) => {
            const styleInfo = this.gamakaStyles[curve.style] || this.gamakaStyles.default;
            const styleName = `${curve.style} curve`;
            
            commands.push(
                `\\draw[${styleName}] (${curve.start.x},${curve.start.y}) .. controls ` +
                `(${curve.cp1.x},${curve.cp1.y}) and (${curve.cp2.x},${curve.cp2.y}) .. ` +
                `(${curve.end.x},${curve.end.y});`
            );
        });
        
        return commands;
    }
    
    generateGamakaStyleDefinitions() {
        const styles = [];
        
        for (const [name, config] of Object.entries(this.gamakaStyles)) {
            const decorations = [];
            
            // Start marker
            if (config.startMarker) {
                const markerNode = this.getMarkerNode(config.startMarker, config.startFill || 'white');
                decorations.push(`mark=at position 0 with {${markerNode}}`);
            }
            
            // End marker
            if (config.endMarker) {
                const endNode = config.endMarker === 'arrow' 
                    ? '\\arrow{stealth}' 
                    : this.getMarkerNode(config.endMarker, config.endFill || config.color);
                decorations.push(`mark=at position 1 with {${endNode}}`);
            }
            
            // Intermediate markers for tribhinna
            if (config.intermediateMarkers) {
                decorations.push('mark=at position 0.33 with {\\fill circle (2pt);}');
                decorations.push('mark=at position 0.66 with {\\fill circle (2pt);}');
            }
            
            styles.push(
                `\\tikzset{${name} curve/.style={` +
                `thick, ${config.color}, ` +
                `decoration={markings, ${decorations.join(', ')}}, ` +
                `postaction={decorate}}}`
            );
        }
        
        return styles.join('\\n');
    }
    
    getMarkerNode(type, fill) {
        switch (type) {
            case 'circle':
                return `\\node[gamaka marker circle, fill=${fill}] {};`;
            case 'square':
                return `\\node[gamaka marker square, fill=${fill}] {};`;
            case 'diamond':
                return `\\node[gamaka marker diamond, fill=${fill}] {};`;
            case 'triangle':
                return `\\node[gamaka marker triangle, fill=${fill}] {};`;
            default:
                return `\\node[gamaka marker circle, fill=${fill}] {};`;
        }
    }

    getGamakaColor(style) {
        const colorMap = {
            'kampita': 'blue',
            'jaru': 'red',
            'andolita': 'green!60!black',
            'tribhinna': 'purple',
            'default': 'black'
        };
        return colorMap[style] || 'black';
    }

    // Generate complete LaTeX document
    async generateLatexDocument(composition) {
        const template = `\\documentclass[a4paper,12pt]{article}
\\usepackage{tikz}
\\usepackage{geometry}
\\usepackage{amssymb}
\\geometry{margin=2cm}
\\usetikzlibrary{calc,positioning,arrows.meta,decorations.markings}

% Define marker shapes
\\tikzset{
    gamaka marker circle/.style={circle, draw=black, inner sep=2pt},
    gamaka marker square/.style={rectangle, draw=black, inner sep=2pt},
    gamaka marker diamond/.style={diamond, draw=black, inner sep=2pt},
    gamaka marker triangle/.style={regular polygon, regular polygon sides=3, draw=black, inner sep=2pt}
}

\\title{${composition.title || 'Gamaka Notation'}}
\\author{${composition.composer || ''}}
\\date{${composition.raga || ''} - ${composition.tala || ''}}

\\begin{document}
\\maketitle

`;

        let content = template;
        
        // Process each section
        for (const section of composition.sections) {
            content += `\\section*{${section.name}}\\n`;
            content += '\\begin{tikzpicture}[scale=1.5]\\n';
            
            const tikzCommands = this.generateTikzCommands(section);
            content += tikzCommands.join('\\n') + '\\n';
            
            content += '\\end{tikzpicture}\\n\\n';
        }
        
        content += '\\end{document}';
        
        return content;
    }

    // Compile LaTeX to PDF
    async compileToPDF(latexContent, outputName) {
        const texFile = path.join(this.outputDir, `${outputName}.tex`);
        const pdfFile = path.join(this.outputDir, `${outputName}.pdf`);
        
        // Ensure output directory exists
        await fs.mkdir(this.outputDir, { recursive: true });
        
        // Write LaTeX file
        await fs.writeFile(texFile, latexContent);
        
        // Compile with pdflatex
        return new Promise((resolve, reject) => {
            exec(
                `pdflatex -output-directory="${this.outputDir}" "${texFile}"`,
                (error, stdout, stderr) => {
                    if (error) {
                        reject(error);
                        return;
                    }
                    resolve(pdfFile);
                }
            );
        });
    }
}

// Example usage
export async function generateGamakaPDF(editorData) {
    const generator = new GamakaLatexGenerator();
    
    try {
        const latexDoc = await generator.generateLatexDocument(editorData);
        const pdfPath = await generator.compileToPDF(latexDoc, editorData.filename);
        return pdfPath;
    } catch (error) {
        console.error('PDF generation failed:', error);
        throw error;
    }
}
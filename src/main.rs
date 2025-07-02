use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod types;
mod parser;
mod validator;
mod formatter;
mod pdf;
mod lsp;

use crate::types::*;

#[derive(Parser)]
#[command(name = "vna")]
#[command(about = "Veena Notation Archive - Tools for .vna files")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lint .vna files for syntax and musical errors
    Lint {
        /// Files to lint (supports globs)
        files: Vec<PathBuf>,
        /// Auto-fix formatting issues
        #[arg(long)]
        fix: bool,
        /// Watch for changes
        #[arg(short, long)]
        watch: bool,
    },
    /// Validate .vna file structure and musical correctness
    Validate {
        /// File to validate
        file: PathBuf,
    },
    /// Format .vna files with consistent spacing and alignment
    Format {
        /// Files to format (supports globs)
        files: Vec<PathBuf>,
        /// Check if files are formatted (exit 1 if not)
        #[arg(short, long)]
        check: bool,
    },
    /// Generate PDF with frequency grids from .vna file
    Pdf {
        /// VNA file to convert
        file: PathBuf,
        /// Output PDF file
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Height of frequency grids in pixels
        #[arg(long, default_value = "60")]
        grid_height: u32,
        /// Page size (a4, letter)
        #[arg(long, default_value = "a4")]
        page_size: String,
    },
    /// Show information about a .vna file
    Info {
        /// VNA file to analyze
        file: PathBuf,
    },
    /// Start LSP server for editor integration
    Lsp,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Lint { files, fix, watch } => {
            if watch {
                todo!("Watch mode not implemented yet");
            }
            
            let files = if files.is_empty() {
                glob::glob("*.vna")?.collect::<Result<Vec<_>, _>>()?
            } else {
                files
            };

            println!("{}", "🎵 Linting VNA files...".blue().bold());
            println!();

            let mut has_errors = false;
            for file in files {
                match lint_file(&file, fix) {
                    Ok(had_issues) => {
                        if had_issues {
                            has_errors = true;
                        }
                    }
                    Err(e) => {
                        println!("{} {}: {}", "❌".red(), file.display(), e);
                        has_errors = true;
                    }
                }
            }

            if has_errors {
                std::process::exit(1);
            } else {
                println!("{}", "🎉 All files passed linting!".green().bold());
            }
        }
        
        Commands::Validate { file } => {
            match validate_file(&file) {
                Ok(_) => println!("{}", "✅ File is valid!".green()),
                Err(e) => {
                    println!("{} {}", "❌ Error:".red(), e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Format { files, check } => {
            let files = if files.is_empty() {
                glob::glob("*.vna")?.collect::<Result<Vec<_>, _>>()?
            } else {
                files
            };

            let mut needs_formatting = false;
            for file in files {
                match format_file(&file, check) {
                    Ok(was_formatted) => {
                        if was_formatted {
                            needs_formatting = true;
                        }
                    }
                    Err(e) => {
                        println!("{} {}: {}", "❌".red(), file.display(), e);
                    }
                }
            }

            if check && needs_formatting {
                std::process::exit(1);
            }
        }

        Commands::Pdf { file, output, grid_height, page_size } => {
            let output = output.unwrap_or_else(|| {
                file.with_extension("pdf")
            });

            println!("{} {}...", "🎵 Generating PDF from".blue(), file.display());
            
            match generate_pdf(&file, &output, grid_height, &page_size) {
                Ok(_) => {
                    println!("{} {}", "✅ PDF generated:".green(), output.display());
                }
                Err(e) => {
                    println!("{} {}", "❌ Error:".red(), e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Info { file } => {
            match show_info(&file) {
                Ok(_) => {}
                Err(e) => {
                    println!("{} {}", "❌ Error:".red(), e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Lsp => {
            println!("{}", "🚀 Starting VNA Language Server...".blue().bold());
            tokio::runtime::Runtime::new()?.block_on(lsp::VnaLanguageServer::run())?;
        }
    }

    Ok(())
}

fn lint_file(file: &PathBuf, fix: bool) -> anyhow::Result<bool> {
    let content = std::fs::read_to_string(file)?;
    let document = parser::parse(&content)?;
    let issues = validator::validate(&document)?;

    println!("{} {}", "📄".cyan(), file.display());

    if issues.is_empty() {
        println!("  {}", "✅ No issues found".green());
        return Ok(false);
    }

    let mut has_errors = false;
    for issue in &issues {
        let (icon, color): (&str, fn(&str) -> ColoredString) = match issue.severity {
            Severity::Error => ("❌", |s| s.red()),
            Severity::Warning => ("⚠️", |s| s.yellow()),
            Severity::Info => ("ℹ️", |s| s.blue()),
        };
        
        println!("  {} Line {}: {}", icon, issue.line, color(&issue.message));
        
        if issue.severity == Severity::Error {
            has_errors = true;
        }
    }

    if fix {
        let formatted = formatter::format(&document)?;
        std::fs::write(file, formatted)?;
        println!("  {}", "🔧 Auto-fixed formatting".green());
    }

    println!();
    Ok(has_errors)
}

fn validate_file(file: &PathBuf) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(file)?;
    let document = parser::parse(&content)?;
    let issues = validator::validate(&document)?;

    println!("{} {}...", "Validating".cyan(), file.display());

    if issues.is_empty() {
        println!("{}", "✅ File is valid!".green());
        println!(
            "📊 {} sections, {} raga, {} tala",
            document.sections.len(),
            document.metadata.raga,
            document.metadata.tala
        );
    } else {
        for issue in issues {
            let prefix = match issue.severity {
                Severity::Error => "ERROR".red(),
                Severity::Warning => "WARNING".yellow(),
                Severity::Info => "INFO".blue(),
            };
            println!("{}: {} (line {})", prefix, issue.message, issue.line);
        }
    }

    Ok(())
}

fn format_file(file: &PathBuf, check_only: bool) -> anyhow::Result<bool> {
    let content = std::fs::read_to_string(file)?;
    let document = parser::parse(&content)?;
    let formatted = formatter::format(&document)?;

    if content != formatted {
        if check_only {
            println!("{} {} is not formatted", "❌".yellow(), file.display());
        } else {
            std::fs::write(file, formatted)?;
            println!("{} Formatted {}", "✅".green(), file.display());
        }
        Ok(true)
    } else {
        println!("{} {} is already formatted", "✅".bright_black(), file.display());
        Ok(false)
    }
}

fn generate_pdf(input: &PathBuf, output: &PathBuf, grid_height: u32, page_size: &str) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(input)?;
    let document = parser::parse(&content)?;
    let pdf_bytes = pdf::generate(&document, grid_height, page_size)?;
    std::fs::write(output, pdf_bytes)?;
    Ok(())
}

fn show_info(file: &PathBuf) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(file)?;
    let document = parser::parse(&content)?;

    println!("{} {}", "📄".cyan(), file.display());
    println!("Title: {}", document.metadata.title);
    println!("Raga: {}", document.metadata.raga);
    println!("Tala: {}", document.metadata.tala);
    println!("Tempo: {} BPM", document.metadata.tempo.unwrap_or(60));
    
    if let Some(composer) = &document.metadata.composer {
        println!("Composer: {}", composer);
    }
    if let Some(language) = &document.metadata.language {
        println!("Language: {}", language);
    }
    if let Some(nadaka) = document.metadata.nadaka {
        println!("Nadaka: {} beats per unit", nadaka);
    }
    if let Some(line_length) = document.metadata.line_length {
        println!("Line length: {} elements", line_length);
    }

    println!("\n{}", "📊 Structure:".cyan());
    for section in &document.sections {
        println!("  {}: {} phrases", section.name, section.phrases.len());
    }

    Ok(())
}
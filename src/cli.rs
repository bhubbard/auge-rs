use crate::cleaner::Cleaner;
use crate::engine::VisionAnalyzer;
use crate::error::AugeError;
use crate::formatter::ResultFormatter;
use crate::input::ImageSource;
use crate::types::{AnalysisMode, OutputFormat};
use clap::Parser;
use std::io::IsTerminal;

#[derive(Parser, Debug)]
#[command(
    name = "auge",
    about = "Apple's on-device Vision framework from the command line in Rust",
    version
)]
pub struct Cli {
    /// Optical Character Recognition
    #[arg(long = "ocr")]
    pub ocr: bool,

    /// Image classification
    #[arg(long = "classify")]
    pub classify: bool,

    /// Barcode and QR code detection
    #[arg(long = "barcode")]
    pub barcode: bool,

    /// Face rectangle detection
    #[arg(long = "faces")]
    pub faces: bool,

    /// Human rectangle detection
    #[arg(long = "humans")]
    pub humans: bool,

    /// Rectangle detection
    #[arg(long = "rectangles")]
    pub rectangles: bool,

    /// Horizon angle detection
    #[arg(long = "horizon")]
    pub horizon: bool,

    /// Animal detection (cats and dogs)
    #[arg(long = "animals")]
    pub animals: bool,

    /// Output format (plain, md, json, ndjson)
    #[arg(short = 'o', long = "output")]
    pub output: Option<OutputFormat>,

    /// Plain text output
    #[arg(long = "plain")]
    pub plain: bool,

    /// Markdown output
    #[arg(long = "md")]
    pub md: bool,

    /// JSON output
    #[arg(long = "json")]
    pub json: bool,

    /// NDJSON output
    #[arg(long = "ndjson")]
    pub ndjson: bool,

    /// Compact JSON output
    #[arg(long = "compact")]
    pub compact: bool,

    /// Clean OCR text (fix hyphenation, smart quotes, layout splits)
    #[arg(long = "clean")]
    pub clean: bool,

    /// Read image from clipboard
    #[arg(long = "clipboard")]
    pub clipboard: bool,

    /// Suppress progress output
    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,

    /// Image or document files to analyze
    pub files: Vec<String>,
}

impl Cli {
    pub fn resolved_format(&self) -> OutputFormat {
        if self.plain {
            OutputFormat::Plain
        } else if self.md {
            OutputFormat::Md
        } else if self.json {
            OutputFormat::Json
        } else if self.ndjson {
            OutputFormat::Ndjson
        } else {
            self.output.unwrap_or(OutputFormat::Plain)
        }
    }

    pub fn resolved_mode(&self) -> Result<AnalysisMode, AugeError> {
        if self.ocr {
            Ok(AnalysisMode::Ocr)
        } else if self.classify {
            Ok(AnalysisMode::Classify)
        } else if self.barcode {
            Ok(AnalysisMode::Barcode)
        } else if self.faces {
            Ok(AnalysisMode::Faces)
        } else if self.humans {
            Ok(AnalysisMode::Humans)
        } else if self.rectangles {
            Ok(AnalysisMode::Rectangles)
        } else if self.horizon {
            Ok(AnalysisMode::Horizon)
        } else if self.animals {
            Ok(AnalysisMode::Animals)
        } else {
            // Default to OCR if no explicit mode provided
            Ok(AnalysisMode::Ocr)
        }
    }
}

pub fn run_cli(cli: Cli) -> Result<(), AugeError> {
    let mode = cli.resolved_mode()?;
    let format = cli.resolved_format();
    let analyzer = VisionAnalyzer::new();

    // 1. Determine image inputs
    let sources: Vec<ImageSource> = if cli.clipboard {
        vec![ImageSource::from_path_or_stdin(None, true)?]
    } else if !cli.files.is_empty() {
        let mut list = Vec::new();
        for f in &cli.files {
            list.push(ImageSource::from_path_or_stdin(Some(f), false)?);
        }
        list
    } else {
        if std::io::stdin().is_terminal() {
            return Err(AugeError::Usage(
                "no image provided: specify file path(s), pass --clipboard, or pipe an image on stdin".into(),
            ));
        }
        vec![ImageSource::from_path_or_stdin(None, false)?]
    };

    for source in sources {
        let path = source.path();
        let display_name = source.display_name();

        let mut payload = analyzer.analyze(mode, path)?;

        // If --clean is requested and mode is OCR, clean the text
        if cli.clean && mode == AnalysisMode::Ocr {
            if let Some(text) = payload.get("text").and_then(|v| v.as_str()) {
                let cleaned = Cleaner::clean_text(text);
                if let Some(obj) = payload.as_object_mut() {
                    obj.insert("text".to_string(), serde_json::Value::String(cleaned));
                }
            }
        }

        let formatted = ResultFormatter::format(mode, &display_name, format, &payload, cli.compact);
        if !formatted.is_empty() {
            println!("{formatted}");
        }
    }

    Ok(())
}

use auge_rs::cleaner::{CleanChunker, Cleaner};
use auge_rs::engine::VisionAnalyzer;
use auge_rs::error::AugeError;
use auge_rs::formatter::ResultFormatter;
use auge_rs::input::ImageSource;
use auge_rs::types::{AnalysisMode, OutputFormat};
use std::path::Path;

// ======================== AUGE ERROR TESTS ========================

#[test]
fn test_auge_error_classify() {
    assert!(matches!(
        AugeError::classify("no such file or directory"),
        AugeError::FileNotFound(_)
    ));
    assert_eq!(
        AugeError::classify("the image could not be decoded"),
        AugeError::InvalidImage
    );
    assert!(matches!(
        AugeError::classify("unsupported image format"),
        AugeError::UnsupportedFormat(_)
    ));
    assert_eq!(
        AugeError::classify("vision framework not available"),
        AugeError::VisionUnavailable
    );
    assert!(matches!(
        AugeError::classify("something went wrong"),
        AugeError::Unknown(_)
    ));
}

#[test]
fn test_auge_error_cli_labels() {
    assert_eq!(AugeError::FileNotFound("/x".into()).cli_label(), "[file not found]");
    assert_eq!(AugeError::InvalidImage.cli_label(), "[invalid image]");
    assert_eq!(AugeError::UnsupportedFormat("tiff".into()).cli_label(), "[unsupported format]");
    assert_eq!(AugeError::VisionUnavailable.cli_label(), "[vision unavailable]");
    assert_eq!(AugeError::NoTextFound.cli_label(), "[no text found]");
    assert_eq!(AugeError::NoResults.cli_label(), "[no results]");
    assert_eq!(AugeError::ClipboardEmpty.cli_label(), "[clipboard empty]");
    assert_eq!(AugeError::NetworkBlocked("http://x".into()).cli_label(), "[network blocked]");
    assert_eq!(AugeError::Unknown("x".into()).cli_label(), "[error]");
}

#[test]
fn test_auge_error_exit_codes() {
    assert_eq!(AugeError::FileNotFound("/x".into()).exit_code(), 1);
    assert_eq!(AugeError::InvalidImage.exit_code(), 1);
    assert_eq!(AugeError::UnsupportedFormat("x".into()).exit_code(), 1);
    assert_eq!(AugeError::VisionUnavailable.exit_code(), 5);
    assert_eq!(AugeError::NoTextFound.exit_code(), 0);
    assert_eq!(AugeError::NoResults.exit_code(), 0);
    assert_eq!(AugeError::ClipboardEmpty.exit_code(), 1);
    assert_eq!(AugeError::NetworkBlocked("http://x".into()).exit_code(), 2);
    assert_eq!(AugeError::Unknown("x".into()).exit_code(), 1);
}

// ======================== IMAGE SOURCE TESTS ========================

#[test]
fn test_supported_extensions() {
    assert!(ImageSource::is_supported_extension("png"));
    assert!(ImageSource::is_supported_extension("jpg"));
    assert!(ImageSource::is_supported_extension("jpeg"));
    assert!(ImageSource::is_supported_extension("tiff"));
    assert!(ImageSource::is_supported_extension("tif"));
    assert!(ImageSource::is_supported_extension("bmp"));
    assert!(ImageSource::is_supported_extension("gif"));
    assert!(ImageSource::is_supported_extension("heic"));
    assert!(ImageSource::is_supported_extension("heif"));
    assert!(ImageSource::is_supported_extension("pdf"));

    assert!(!ImageSource::is_supported_extension("webp"));
    assert!(!ImageSource::is_supported_extension("txt"));
    assert!(!ImageSource::is_supported_extension("svg"));

    // Case-insensitivity
    assert!(ImageSource::is_supported_extension("PNG"));
    assert!(ImageSource::is_supported_extension("Jpg"));
    assert!(ImageSource::is_supported_extension("HEIC"));
}

#[test]
fn test_extension_from_path() {
    assert_eq!(ImageSource::extension_from("/tmp/photo.png"), Some("png".to_string()));
    assert_eq!(ImageSource::extension_from("image.JPEG"), Some("jpeg".to_string()));
    assert_eq!(ImageSource::extension_from("/a/b/c.TiFf"), Some("tiff".to_string()));
    assert_eq!(ImageSource::extension_from("/tmp/noext"), None);
    assert_eq!(ImageSource::extension_from("/tmp/.gitignore"), None);
    assert_eq!(ImageSource::extension_from("/tmp/file.backup.png"), Some("png".to_string()));
    assert_eq!(ImageSource::extension_from("/tmp/my photo.jpg"), Some("jpg".to_string()));
}

// ======================== CLEAN CHUNKER TESTS ========================

#[test]
fn test_clean_chunker_empty_and_short() {
    assert_eq!(CleanChunker::chunk("", 5000), Vec::<String>::new());
    assert_eq!(CleanChunker::chunk("hello", 5000), vec!["hello".to_string()]);
}

#[test]
fn test_clean_chunker_oversized_text_splits() {
    let text = "a".repeat(6000);
    let chunks = CleanChunker::chunk(&text, 1000);
    assert_eq!(chunks.len(), 6);
    for c in chunks {
        assert!(c.chars().count() <= 1000);
    }
}

#[test]
fn test_clean_chunker_paragraph_boundaries() {
    let p1 = "x".repeat(500);
    let p2 = "y".repeat(500);
    let p3 = "z".repeat(500);
    let combined = format!("{p1}\n\n{p2}\n\n{p3}");
    let chunks = CleanChunker::chunk(&combined, 1100);
    assert!(chunks.len() >= 1 && chunks.len() <= 3);
    for c in chunks {
        assert!(c.chars().count() <= 1100);
    }
}

// ======================== CLEANER TESTS ========================

#[test]
fn test_cleaner_transformations() {
    let raw = "This is a multi-\nline docu-\nment with “quotes” and ‘apostrophes’.";
    let cleaned = Cleaner::clean_text(raw);
    assert!(cleaned.contains("multiline"));
    assert!(cleaned.contains("document"));
    assert!(cleaned.contains("\"quotes\""));
    assert!(cleaned.contains("'apostrophes'"));
}

// ======================== FORMATTER TESTS ========================

#[test]
fn test_formatter_json_envelope() {
    let payload = serde_json::json!({
        "text": "Hello world"
    });
    let output = ResultFormatter::format(
        AnalysisMode::Ocr,
        "sample.png",
        OutputFormat::Json,
        &payload,
        true,
    );
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(parsed["mode"], "ocr");
    assert_eq!(parsed["file"], "sample.png");
    assert_eq!(parsed["metadata"]["schema"], "2");
    assert_eq!(parsed["metadata"]["on_device"], true);
}

#[test]
fn test_formatter_markdown_classifications_and_barcodes() {
    let payload = serde_json::json!({
        "classifications": [
            { "label": "cat", "confidence": 0.95 },
            { "label": "dog", "confidence": 0.05 }
        ],
        "barcodes": [
            { "payload": "https://example.com", "symbology": "QR" }
        ]
    });

    let md_classify = ResultFormatter::format(
        AnalysisMode::Classify,
        "test.jpg",
        OutputFormat::Md,
        &payload,
        false,
    );
    assert!(md_classify.contains("- **cat** — 95%"));

    let md_barcode = ResultFormatter::format(
        AnalysisMode::Barcode,
        "test.jpg",
        OutputFormat::Md,
        &payload,
        false,
    );
    assert!(md_barcode.contains("- `QR`: https://example.com"));
}

// ======================== APPLE VISION LIVE INFERENCE ========================

#[test]
fn test_vision_analyzer_on_image() {
    let test_img = Path::new("/Users/bhubbard/PROJECTS/higgsfield-cli/demo.png");
    if test_img.exists() {
        let analyzer = VisionAnalyzer::new();
        let result = analyzer.analyze(AnalysisMode::Classify, test_img).unwrap();
        let classifications = result["classifications"].as_array().unwrap();
        assert!(!classifications.is_empty());
    }
}

use auge_rs::cleaner::Cleaner;
use auge_rs::engine::VisionAnalyzer;
use auge_rs::formatter::ResultFormatter;
use auge_rs::types::{AnalysisMode, OutputFormat};
use std::path::Path;

#[test]
fn test_cleaner_transformations() {
    let raw = "This is a multi-\nline docu-\nment with “quotes” and ‘apostrophes’.";
    let cleaned = Cleaner::clean_text(raw);
    assert!(cleaned.contains("multiline"));
    assert!(cleaned.contains("document"));
    assert!(cleaned.contains("\"quotes\""));
    assert!(cleaned.contains("'apostrophes'"));
}

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
fn test_vision_analyzer_on_image() {
    let test_img = Path::new("/Users/bhubbard/PROJECTS/higgsfield-cli/demo.png");
    if test_img.exists() {
        let analyzer = VisionAnalyzer::new();
        let result = analyzer.analyze(AnalysisMode::Classify, test_img).unwrap();
        let classifications = result["classifications"].as_array().unwrap();
        assert!(!classifications.is_empty());
    }
}

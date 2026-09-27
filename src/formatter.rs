use crate::types::{AnalysisMode, OutputFormat};

pub struct ResultFormatter;

impl ResultFormatter {
    pub fn format(
        mode: AnalysisMode,
        file: &str,
        format: OutputFormat,
        payload: &serde_json::Value,
        compact: bool,
    ) -> String {
        match format {
            OutputFormat::Plain => Self::format_plain(mode, payload),
            OutputFormat::Md => Self::format_markdown(mode, payload),
            OutputFormat::Json => {
                let envelope = serde_json::json!({
                    "mode": mode.as_str(),
                    "file": file,
                    "results": payload,
                    "metadata": {
                        "on_device": true,
                        "version": env!("CARGO_PKG_VERSION"),
                        "schema": "2"
                    }
                });
                if compact {
                    serde_json::to_string(&envelope).unwrap_or_default()
                } else {
                    serde_json::to_string_pretty(&envelope).unwrap_or_default()
                }
            }
            OutputFormat::Ndjson => {
                let envelope = serde_json::json!({
                    "mode": mode.as_str(),
                    "file": file,
                    "results": payload,
                    "metadata": {
                        "on_device": true,
                        "version": env!("CARGO_PKG_VERSION"),
                        "schema": "2"
                    }
                });
                serde_json::to_string(&envelope).unwrap_or_default()
            }
        }
    }

    fn format_plain(mode: AnalysisMode, payload: &serde_json::Value) -> String {
        match mode {
            AnalysisMode::Ocr => {
                if let Some(text) = payload.get("text").and_then(|v| v.as_str()) {
                    text.to_string()
                } else {
                    String::new()
                }
            }
            AnalysisMode::Classify => {
                if let Some(items) = payload.get("classifications").and_then(|v| v.as_array()) {
                    let mut lines = Vec::new();
                    for item in items {
                        let label = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
                        let conf = item.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let pct = (conf * 100.0).round() as i64;
                        lines.push(format!("{label}: {pct}%"));
                    }
                    lines.join("\n")
                } else {
                    String::new()
                }
            }
            AnalysisMode::Barcode => {
                if let Some(items) = payload.get("barcodes").and_then(|v| v.as_array()) {
                    let mut lines = Vec::new();
                    for item in items {
                        let payload_str = item.get("payload").and_then(|v| v.as_str()).unwrap_or("");
                        let symbology = item.get("symbology").and_then(|v| v.as_str()).unwrap_or("");
                        lines.push(format!("[{symbology}] {payload_str}"));
                    }
                    lines.join("\n")
                } else {
                    String::new()
                }
            }
            AnalysisMode::Faces => {
                let count = payload.get("faces").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
                let noun = if count == 1 { "face" } else { "faces" };
                format!("{count} {noun} detected")
            }
            AnalysisMode::Humans => {
                let count = payload.get("humans").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
                let noun = if count == 1 { "human" } else { "humans" };
                format!("{count} {noun} detected")
            }
            AnalysisMode::Rectangles => {
                let count = payload.get("rectangles").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
                let noun = if count == 1 { "rectangle" } else { "rectangles" };
                format!("{count} {noun} detected")
            }
            AnalysisMode::Horizon => {
                let deg = payload.get("angle_degrees").and_then(|v| v.as_f64()).unwrap_or(0.0);
                format!("horizon angle: {deg:.2}°")
            }
            AnalysisMode::Animals => {
                if let Some(items) = payload.get("animals").and_then(|v| v.as_array()) {
                    let mut lines = Vec::new();
                    for item in items {
                        let label = item.get("label").and_then(|v| v.as_str()).unwrap_or("animal");
                        let conf = item.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let pct = (conf * 100.0).round() as i64;
                        lines.push(format!("{label}: {pct}%"));
                    }
                    lines.join("\n")
                } else {
                    String::new()
                }
            }
            _ => serde_json::to_string(payload).unwrap_or_default(),
        }
    }

    fn format_markdown(mode: AnalysisMode, payload: &serde_json::Value) -> String {
        match mode {
            AnalysisMode::Ocr => {
                payload.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string()
            }
            AnalysisMode::Classify => {
                if let Some(items) = payload.get("classifications").and_then(|v| v.as_array()) {
                    let mut lines = Vec::new();
                    for item in items {
                        let label = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
                        let conf = item.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let pct = (conf * 100.0).round() as i64;
                        lines.push(format!("- **{label}** — {pct}%"));
                    }
                    lines.join("\n")
                } else {
                    String::new()
                }
            }
            AnalysisMode::Barcode => {
                if let Some(items) = payload.get("barcodes").and_then(|v| v.as_array()) {
                    let mut lines = Vec::new();
                    for item in items {
                        let payload_str = item.get("payload").and_then(|v| v.as_str()).unwrap_or("");
                        let symbology = item.get("symbology").and_then(|v| v.as_str()).unwrap_or("");
                        lines.push(format!("- `{symbology}`: {payload_str}"));
                    }
                    lines.join("\n")
                } else {
                    String::new()
                }
            }
            _ => Self::format_plain(mode, payload),
        }
    }
}

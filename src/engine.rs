use crate::error::AugeError;
use crate::types::AnalysisMode;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct VisionAnalyzer {
    bridge_path: Option<PathBuf>,
}

impl VisionAnalyzer {
    pub fn new() -> Self {
        let mut path = None;

        if let Some(bin_str) = option_env!("APPLE_VISION_BRIDGE_BIN") {
            let p = PathBuf::from(bin_str);
            if p.exists() {
                path = Some(p);
            }
        }

        if path.is_none() {
            let p = PathBuf::from("bridge/apple_vision_bridge");
            if p.exists() {
                path = Some(p);
            }
        }

        if path.is_none() {
            if let Ok(exe) = std::env::current_exe() {
                if let Some(dir) = exe.parent() {
                    let p = dir.join("apple_vision_bridge");
                    if p.exists() {
                        path = Some(p);
                    }
                }
            }
        }

        Self { bridge_path: path }
    }

    pub fn analyze(
        &self,
        mode: AnalysisMode,
        image_path: &Path,
    ) -> Result<serde_json::Value, AugeError> {
        let bridge = self.bridge_path.as_ref().ok_or(AugeError::VisionUnavailable)?;

        let output = Command::new(bridge)
            .args([mode.as_str(), &image_path.to_string_lossy()])
            .output()
            .map_err(|e| AugeError::Unknown(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            if stderr.contains("could not load image") {
                return Err(AugeError::InvalidImage);
            }
            if stderr.contains("Unsupported mode") {
                return Err(AugeError::Usage(format!("Unsupported mode '{}'", mode.as_str())));
            }
            return Err(AugeError::Unknown(stderr));
        }

        let json_val: serde_json::Value = serde_json::from_slice(&output.stdout)
            .map_err(|e| AugeError::Unknown(format!("Invalid bridge JSON response: {e}")))?;

        Ok(json_val)
    }
}

impl Default for VisionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

use crate::error::AugeError;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::NamedTempFile;

#[derive(Debug)]
pub enum ImageSource {
    FilePath(PathBuf),
    TempFile(NamedTempFile),
}

impl ImageSource {
    pub const SUPPORTED_EXTENSIONS: &[&str] = &[
        "png", "jpg", "jpeg", "tiff", "tif", "bmp", "gif", "heic", "heif", "pdf",
    ];

    pub fn is_supported_extension(ext: &str) -> bool {
        let lower = ext.trim().to_lowercase();
        Self::SUPPORTED_EXTENSIONS.contains(&lower.as_str())
    }

    pub fn extension_from(path: &str) -> Option<String> {
        let p = Path::new(path);
        let file_name = p.file_name()?.to_str()?;

        // If file_name starts with a dot and has no other dot, it's a dotfile with no extension
        if file_name.starts_with('.') && !file_name[1..].contains('.') {
            return None;
        }

        p.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
    }

    pub fn validate_path(path: &str) -> Result<PathBuf, AugeError> {
        if path.is_empty() {
            return Err(AugeError::FileNotFound(path.to_string()));
        }

        let p = PathBuf::from(path);
        if !p.exists() || p.is_dir() {
            return Err(AugeError::FileNotFound(path.to_string()));
        }

        if let Some(ext) = Self::extension_from(path) {
            if !Self::is_supported_extension(&ext) {
                return Err(AugeError::UnsupportedFormat(ext));
            }
        }

        Ok(p)
    }

    pub fn path(&self) -> &Path {
        match self {
            ImageSource::FilePath(p) => p.as_path(),
            ImageSource::TempFile(t) => t.path(),
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            ImageSource::FilePath(p) => p.to_string_lossy().to_string(),
            ImageSource::TempFile(_) => "<stdin>".to_string(),
        }
    }

    pub fn from_path_or_stdin(
        path: Option<&str>,
        use_clipboard: bool,
    ) -> Result<Self, AugeError> {
        if use_clipboard {
            return Self::from_clipboard();
        }

        if let Some(p) = path {
            let path_buf = Self::validate_path(p)?;
            return Ok(ImageSource::FilePath(path_buf));
        }

        Self::from_stdin()
    }

    fn from_stdin() -> Result<Self, AugeError> {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;

        if buffer.is_empty() {
            return Err(AugeError::InvalidImage);
        }

        let mut temp = NamedTempFile::new()?;
        temp.write_all(&buffer)?;
        temp.flush()?;

        Ok(ImageSource::TempFile(temp))
    }

    fn from_clipboard() -> Result<Self, AugeError> {
        let output = Command::new("osascript")
            .args([
                "-e",
                "try\nset theData to the clipboard as «class PNGf»\nreturn 1\non error\nreturn 0\nend try",
            ])
            .output()
            .map_err(|e| AugeError::Unknown(e.to_string()))?;

        let has_png = String::from_utf8_lossy(&output.stdout).trim() == "1";
        if !has_png {
            return Err(AugeError::ClipboardEmpty);
        }

        let temp = NamedTempFile::new()?;
        let dump = Command::new("osascript")
            .args([
                "-e",
                &format!(
                    "set theData to the clipboard as «class PNGf»\nset fp to open for access POSIX file \"{}\" with write permission\nwrite theData to fp\nclose access fp",
                    temp.path().to_string_lossy()
                ),
            ])
            .output()
            .map_err(|e| AugeError::Unknown(e.to_string()))?;

        if !dump.status.success() {
            return Err(AugeError::ClipboardEmpty);
        }

        Ok(ImageSource::TempFile(temp))
    }
}

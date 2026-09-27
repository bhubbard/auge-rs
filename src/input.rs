use crate::error::AugeError;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::NamedTempFile;

pub enum ImageSource {
    FilePath(PathBuf),
    TempFile(NamedTempFile),
}

impl ImageSource {
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
            let path_buf = PathBuf::from(p);
            if !path_buf.exists() {
                return Err(AugeError::FileNotFound(p.to_string()));
            }
            return Ok(ImageSource::FilePath(path_buf));
        }

        // Stdin input
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
        // macOS pbpaste / osascript clipboard image extraction
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

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum AugeError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("The file is not a valid image or could not be decoded.")]
    InvalidImage,

    #[error("Unsupported image format: {0}. Use PNG, JPEG, TIFF, BMP, GIF, HEIC, or PDF.")]
    UnsupportedFormat(String),

    #[error("Apple Vision framework is not available on this system.")]
    VisionUnavailable,

    #[error("No text was detected in the image.")]
    NoTextFound,

    #[error("No results were detected in the image.")]
    NoResults,

    #[error("Clipboard does not contain a PNG, JPEG, HEIC, TIFF image, or a file URL.")]
    ClipboardEmpty,

    #[error("Network call blocked (auge is on-device only): {0}")]
    NetworkBlocked(String),

    #[error("Usage error: {0}")]
    Usage(String),

    #[error("{0}")]
    Unknown(String),

    #[error("IO error: {0}")]
    Io(String),
}

impl AugeError {
    pub fn classify(desc: &str) -> Self {
        let lower = desc.to_lowercase();
        if lower.contains("no such file")
            || lower.contains("file not found")
            || lower.contains("doesn't exist")
        {
            return AugeError::FileNotFound(desc.to_string());
        }
        if lower.contains("could not be decoded")
            || lower.contains("invalid image")
            || lower.contains("corrupt")
        {
            return AugeError::InvalidImage;
        }
        if lower.contains("unsupported") && (lower.contains("format") || lower.contains("image")) {
            return AugeError::UnsupportedFormat(desc.to_string());
        }
        if lower.contains("vision")
            && (lower.contains("not available") || lower.contains("unavailable"))
        {
            return AugeError::VisionUnavailable;
        }
        AugeError::Unknown(desc.to_string())
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            AugeError::FileNotFound(_) => 1,
            AugeError::InvalidImage => 1,
            AugeError::UnsupportedFormat(_) => 1,
            AugeError::VisionUnavailable => 5,
            AugeError::NoTextFound => 0,
            AugeError::NoResults => 0,
            AugeError::ClipboardEmpty => 1,
            AugeError::NetworkBlocked(_) => 2,
            AugeError::Usage(_) => 2,
            AugeError::Unknown(_) => 1,
            AugeError::Io(_) => 1,
        }
    }

    pub fn cli_label(&self) -> &'static str {
        match self {
            AugeError::FileNotFound(_) => "[file not found]",
            AugeError::InvalidImage => "[invalid image]",
            AugeError::UnsupportedFormat(_) => "[unsupported format]",
            AugeError::VisionUnavailable => "[vision unavailable]",
            AugeError::NoTextFound => "[no text found]",
            AugeError::NoResults => "[no results]",
            AugeError::ClipboardEmpty => "[clipboard empty]",
            AugeError::NetworkBlocked(_) => "[network blocked]",
            AugeError::Usage(_) => "[usage error]",
            AugeError::Unknown(_) | AugeError::Io(_) => "[error]",
        }
    }

    pub fn user_message(&self) -> String {
        self.to_string()
    }
}

impl From<std::io::Error> for AugeError {
    fn from(err: std::io::Error) -> Self {
        AugeError::Io(err.to_string())
    }
}

use regex::Regex;
use std::sync::LazyLock;

static HYPHENATION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\w+)-\n(\w+)").unwrap()
});

static MULTI_SPACE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[ \t]+").unwrap()
});

pub struct Cleaner;

impl Cleaner {
    pub fn clean_text(text: &str) -> String {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return String::new();
        }

        // 1. Join end-of-line hyphenation: "docu-\nment" -> "document"
        let dehyphenated = HYPHENATION_REGEX.replace_all(trimmed, "$1$2");

        // 2. Normalize smart quotes and unicode dashes
        let normalized = dehyphenated
            .replace('“', "\"")
            .replace('”', "\"")
            .replace('‘', "'")
            .replace('’', "'")
            .replace('—', " - ")
            .replace('–', " - ");

        // 3. Normalize multiple spaces
        let cleaned = MULTI_SPACE_REGEX.replace_all(&normalized, " ");

        cleaned.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_hyphenation() {
        let input = "This is a docu-\nment with some text.";
        let output = Cleaner::clean_text(input);
        assert_eq!(output, "This is a document with some text.");
    }

    #[test]
    fn test_clean_smart_quotes() {
        let input = "“Hello world” and ‘single quotes’";
        let output = Cleaner::clean_text(input);
        assert_eq!(output, "\"Hello world\" and 'single quotes'");
    }
}

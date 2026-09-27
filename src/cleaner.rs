use regex::Regex;
use std::sync::LazyLock;

static HYPHENATION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\w+)-\n(\w+)").unwrap()
});

static MULTI_SPACE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[ \t]+").unwrap()
});

pub struct CleanChunker;

impl CleanChunker {
    pub const DEFAULT_MAX_CHARACTERS: usize = 5000;

    pub fn chunk(text: &str, max_characters: usize) -> Vec<String> {
        let max = if max_characters == 0 {
            return if text.is_empty() { Vec::new() } else { vec![text.to_string()] };
        } else {
            max_characters
        };

        if text.is_empty() {
            return Vec::new();
        }

        if text.chars().count() <= max {
            return vec![text.to_string()];
        }

        let paragraphs: Vec<&str> = text.split("\n\n").collect();
        let mut chunks = Vec::new();
        let mut current_chunk = Vec::new();
        let mut current_len = 0;

        for p in paragraphs {
            let p_len = p.chars().count();

            // If a single paragraph is larger than max, hard-split it
            if p_len > max {
                if !current_chunk.is_empty() {
                    chunks.push(current_chunk.join("\n\n"));
                    current_chunk.clear();
                    current_len = 0;
                }
                let mut start = 0;
                let chars: Vec<char> = p.chars().collect();
                while start < chars.len() {
                    let end = (start + max).min(chars.len());
                    chunks.push(chars[start..end].iter().collect());
                    start = end;
                }
                continue;
            }

            let separator_len = if current_chunk.is_empty() { 0 } else { 2 };
            if current_len + separator_len + p_len <= max {
                current_chunk.push(p);
                current_len += separator_len + p_len;
            } else {
                chunks.push(current_chunk.join("\n\n"));
                current_chunk.clear();
                current_chunk.push(p);
                current_len = p_len;
            }
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk.join("\n\n"));
        }

        chunks
    }
}

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

    #[test]
    fn test_clean_chunker_empty() {
        assert_eq!(CleanChunker::chunk("", 5000), Vec::<String>::new());
    }

    #[test]
    fn test_clean_chunker_short() {
        assert_eq!(CleanChunker::chunk("hello", 5000), vec!["hello"]);
    }
}

use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WordContext {
    Code,
    Comment,
    Attribute,
    Path,
    Literal,
}

pub struct TypoAnalyzer {
    dictionary: HashSet<String>,
    code_words: HashSet<String>
}

impl TypoAnalyzer {
    pub fn new() -> Self {
        let common_words = include_str!("../../resources/words.txt")
            .lines()
            .map(String::from)
            .collect();

        let code_words = include_str!("../../resources/code_words.txt")
            .lines()
            .filter(|line| !line.starts_with("//") && !line.trim().is_empty())
            .map(String::from)
            .collect();

        Self {
            dictionary: common_words,
            code_words
        }
    }

    fn should_ignore_word(word: &str) -> bool {
        if word.len() <= 2 {
            return true;
        }

        if (word.starts_with('\'') && word.ends_with('\'')) && word.len() == 3 {
            return true;
        }

        if word.chars().any(|c| c.is_numeric()) {
            return true;
        }

        if word.contains('/') || word.contains('\\') || 
           word.contains("::") || word.contains("://") {
            return true;
        }

        if word.contains("->") || word.contains("=>") ||
           word.starts_with('<') || word.ends_with('>') ||
           word.starts_with('(') || word.ends_with(')') ||
           word.starts_with('[') || word.ends_with(']') ||
           word.starts_with('{') || word.ends_with('}') {
            return true;
        }

        if word.contains('_') {
            if word.chars().all(|c| c.is_ascii_lowercase() || c == '_') ||
               word.chars().all(|c| c.is_ascii_uppercase() || c == '_') {
                return true;
            }
        }

        if word.chars().next().map_or(false, |c| c.is_ascii_uppercase()) &&
           !word.contains(' ') &&
           word.chars().all(|c| c.is_alphanumeric()) {
            return false;
        }

        if word.starts_with('_') || word.starts_with("r#") {
            return true;
        }

        if word.starts_with("0x") || word.starts_with("0b") ||
           word.starts_with("0o") || word.ends_with("px") ||
           word.ends_with("em") || word.ends_with("rem") {
            return true;
        }

        if word.contains('@') || word.contains('.') {
            return true;
        }

        if word.starts_with('#') || word.starts_with('<') ||
           word.starts_with('`') || word.starts_with('*') {
            return true;
        }

        false
    }

    fn get_word_context(line: &str, word_start: usize) -> WordContext {
        let line_before = &line[..word_start];
        let trimmed = line_before.trim_start();
        
        if trimmed.starts_with('#') || line.contains("derive") || line.contains("cfg") {
            return WordContext::Attribute;
        }

        if trimmed.starts_with("//") || trimmed.starts_with("/*") ||
           (line_before.contains("/*") && !line_before.contains("*/")) {
            return WordContext::Comment;
        }

        if line_before.contains("use ") || line_before.contains("mod ") ||
           line_before.contains("crate::") || line_before.contains("super::") ||
           line_before.contains("self::") || line.contains("::") {
            return WordContext::Path;
        }

        let mut in_string = false;
        let mut in_char = false;
        let mut escaped = false;

        for c in line_before.chars() {
            match c {
                '"' if !escaped => in_string = !in_string,
                '\'' if !escaped => in_char = !in_char,
                '\\' => escaped = !escaped,
                _ => escaped = false,
            }
        }

        if in_string || in_char {
            WordContext::Literal
        } else {
            WordContext::Code
        }
    }

    fn clean_word(word: &str, context: &WordContext) -> String {
        match context {
            WordContext::Path => String::new(),
            WordContext::Attribute => String::new(),
            WordContext::Literal => {
                let cleaned = word.trim_matches(|c| c == '"' || c == '\'');
                if cleaned.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
                    cleaned.to_lowercase()
                } else {
                    String::new()
                }
            },
            WordContext::Comment => {
                if word.chars().all(|c| c.is_alphabetic()) {
                    word.to_lowercase()
                } else {
                    String::new()
                }
            },
            WordContext::Code => {
                if word.chars().all(|c| c.is_alphabetic()) {
                    word.to_lowercase()
                } else {
                    String::new()
                }
            }
        }
    }

    pub fn check_word(&self, word: &str, context: &WordContext) -> bool {
        if Self::should_ignore_word(word) {
            return true;
        }

        let cleaned = Self::clean_word(word, context);
        if cleaned.is_empty() {
            return true;
        }

        match context {
            WordContext::Comment | WordContext::Literal => {
                cleaned.split_whitespace()
                    .all(|word| self.dictionary.contains(word) || self.code_words.contains(word))
            },
            _ => self.dictionary.contains(&cleaned) || self.code_words.contains(&cleaned)
        }
    }

    pub fn scan_file(&self, path: &Path) -> Result<Vec<(String, usize)>, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let mut typos = Vec::new();
        let mut seen_words = HashSet::new();

        for (line_num, line) in content.lines().enumerate() {
            let mut word_start = 0;
            let mut in_comment = false;

            let trimmed = line.trim_start();
            if trimmed.starts_with("//") || trimmed.starts_with("/*") {
                in_comment = true;
            }

            for word in line.split_whitespace() {
                word_start = line[word_start..].find(word).map_or(0, |pos| pos + word_start);
                let mut context = Self::get_word_context(line, word_start);
                
                if in_comment {
                    context = WordContext::Comment;
                }

                let word_key = (word.to_string(), context.clone());
                if !seen_words.contains(&word_key) {
                    let cleaned = Self::clean_word(word, &context);
                    if !cleaned.is_empty() && !self.check_word(word, &context) {
                        typos.push((word.to_string(), line_num + 1));
                        seen_words.insert(word_key);
                    }
                }

                word_start += word.len();
            }
        }

        Ok(typos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_test_file(content: &str) -> std::io::Result<(std::path::PathBuf, impl Drop)> {
        let path = std::env::temp_dir().join(format!("test_{}.txt", rand::random::<u32>()));
        fs::write(&path, content)?;
        
        struct Cleanup(std::path::PathBuf);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                let _ = fs::remove_file(&self.0);
            }
        }
        
        Ok((path.clone(), Cleanup(path)))
    }

    #[test]
    fn test_check_word() {
        let analyzer = TypoAnalyzer::new();
        
        assert!(analyzer.check_word("the", &WordContext::Code));
        assert!(analyzer.check_word("THE", &WordContext::Code));
        assert!(!analyzer.check_word("thex", &WordContext::Code));

        assert!(analyzer.check_word("struct", &WordContext::Code));
        assert!(analyzer.check_word("impl", &WordContext::Code));
        assert!(analyzer.check_word("fn", &WordContext::Code));

        assert!(analyzer.check_word("snake_case_ident", &WordContext::Code));
        assert!(analyzer.check_word("SCREAMING_SNAKE_CASE", &WordContext::Code));
        assert!(analyzer.check_word("_private_ident", &WordContext::Code));

        assert!(analyzer.check_word("std::string::String", &WordContext::Path));
        assert!(analyzer.check_word("Result<T, E>", &WordContext::Code));
        assert!(analyzer.check_word("x -> y", &WordContext::Code));

        assert!(analyzer.check_word("\"string literal\"", &WordContext::Literal));
        assert!(analyzer.check_word("'c'", &WordContext::Literal));
        assert!(analyzer.check_word("42", &WordContext::Literal));

        assert!(analyzer.check_word("#[derive(Debug)]", &WordContext::Attribute));
        assert!(analyzer.check_word("#[cfg(test)]", &WordContext::Attribute));

        assert!(analyzer.check_word("0xFF", &WordContext::Code));
        assert!(analyzer.check_word("100px", &WordContext::Code));
        assert!(analyzer.check_word("2em", &WordContext::Code));
        assert!(analyzer.check_word("user@example.com", &WordContext::Code));
        assert!(analyzer.check_word("example.com", &WordContext::Code));
        assert!(analyzer.check_word("# Heading", &WordContext::Code));
        assert!(analyzer.check_word("<div>", &WordContext::Code));
    }

    #[test]
    fn test_scan_file() -> std::io::Result<()> {
        let analyzer = TypoAnalyzer::new();
        let content = r#"
            fn main() {
                let message = "This is a sampel message";
                println!("{}", message);
            }
        "#;
        let (path, _cleanup) = create_test_file(content)?;

        let typos = analyzer.scan_file(&path)?;
        assert_eq!(typos.len(), 1);
        assert!(typos.iter().any(|(word, line)| word == "sampel" && *line == 3));
        
        Ok(())
    }

    #[test]
    fn test_context_awareness() -> std::io::Result<()> {
        let analyzer = TypoAnalyzer::new();
        let content = r#"
Sampel
c
42
#FF0000
100px
user@example.com
HashMap Debug Clone MyStruct
"#;
        let (path, _cleanup) = create_test_file(content)?;

        let typos = analyzer.scan_file(&path)?;
        
        assert!(typos.iter().any(|(word, _)| word == "Sampel"));

        assert!(!typos.iter().any(|(word, _)| word == "HashMap"));
        assert!(!typos.iter().any(|(word, _)| word == "Debug"));
        assert!(!typos.iter().any(|(word, _)| word == "Clone"));
        assert!(!typos.iter().any(|(word, _)| word == "MyStruct"));
        assert!(!typos.iter().any(|(word, _)| word == "42"));
        assert!(!typos.iter().any(|(word, _)| word == "#FF0000"));
        assert!(!typos.iter().any(|(word, _)| word == "100px"));
        assert!(!typos.iter().any(|(word, _)| word == "user@example.com"));
        
        Ok(())
    }
}

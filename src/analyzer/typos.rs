use std::collections::HashSet;
use std::path::Path;

pub struct TypoAnalyzer {
    dictionary: HashSet<String>
}

impl TypoAnalyzer {
    pub fn new() -> Self {
        let common_words = include_str!("../../resources/words.txt")
            .lines()
            .map(String::from)
            .collect();

        Self {
            dictionary: common_words
        }
    }

    fn clean_word(word: &str) -> String {
        word.trim_matches(|c: char| !c.is_alphabetic())
            .to_lowercase()
    }

    pub fn check_word(&self, word: &str) -> bool {
        let cleaned = Self::clean_word(word);
        if cleaned.is_empty() || cleaned.len() <= 2 {
            return true;
        }
        self.dictionary.contains(&cleaned)
    }

    pub fn scan_file(&self, path: &Path) -> Result<Vec<(String, usize)>, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let typos: Vec<(String, usize)> = content
            .lines()
            .enumerate()
            .flat_map(|(line_num, line)| {
                line.split_whitespace()
                    .filter(|word| !self.check_word(word))
                    .map(move |word| (word.to_string(), line_num + 1))
            })
            .collect();

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
        assert!(analyzer.check_word("the"));
        assert!(analyzer.check_word("THE"));
        assert!(!analyzer.check_word("thex"));
    }

    #[test]
    fn test_scan_file() -> std::io::Result<()> {
        let analyzer = TypoAnalyzer::new();
        let content = "The quick brwn fox\njumped ovr the lazy dog";
        let (path, _cleanup) = create_test_file(content)?;

        let typos = analyzer.scan_file(&path)?;
        assert_eq!(typos.len(), 2);
        assert_eq!(typos[0].0, "brwn");
        assert_eq!(typos[0].1, 1);
        assert_eq!(typos[1].0, "ovr");
        assert_eq!(typos[1].1, 2);
        
        Ok(())
    }

    #[test]
    fn test_case_insensitive() {
        let analyzer = TypoAnalyzer::new();
        assert!(analyzer.check_word("The"));
        assert!(analyzer.check_word("THE"));
        assert!(analyzer.check_word("the"));
    }

    #[test]
    fn test_punctuation() {
        let analyzer = TypoAnalyzer::new();
        assert!(analyzer.check_word("the."));
        assert!(analyzer.check_word("\"the\""));
        assert!(analyzer.check_word("(the)"));
    }
}

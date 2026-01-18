use regex::Regex;
use crate::learning::Token;

pub struct JapaneseTokenizer {
    kanji_regex: Regex,
    kana_regex: Regex,
}

impl Default for JapaneseTokenizer {
    fn default() -> Self {
        Self {
            kanji_regex: Regex::new(r"[\u4e00-\u9faf]+").unwrap(),
            kana_regex: Regex::new(r"[\u3040-\u30ff]+").unwrap(),
        }
    }
}

impl JapaneseTokenizer {
    pub fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current_pos = 0;
        
        while current_pos < text.len() {
            let slice = &text[current_pos..];
            
            // Try matching Kanji block
            if let Some(m) = self.kanji_regex.find(slice) {
                if m.start() == 0 {
                    tokens.push(Token {
                        text: m.as_str().to_string(),
                        part_of_speech: "KANJI".to_string(),
                        lemma: m.as_str().to_string(),
                    });
                    current_pos += m.end();
                    continue;
                }
            }
            
            // Try matching Kana block
            if let Some(m) = self.kana_regex.find(slice) {
                if m.start() == 0 {
                    tokens.push(Token {
                        text: m.as_str().to_string(),
                        part_of_speech: "KANA".to_string(),
                        lemma: m.as_str().to_string(),
                    });
                    current_pos += m.end();
                    continue;
                }
            }
            
            // Default to single character if no block matched
            let ch = slice.chars().next().unwrap();
            tokens.push(Token {
                text: ch.to_string(),
                part_of_speech: "OTHER".to_string(),
                lemma: ch.to_string(),
            });
            current_pos += ch.len_utf8();
        }
        
        tokens
    }
}

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NON_ALPHANUM_RE: Regex = Regex::new(r"[^a-z0-9]+").unwrap();
    static ref SPACES_RE: Regex = Regex::new(r"\s+").unwrap();
    static ref VALID_TOKEN_RE: Regex = Regex::new(r"^[a-z0-9]+$").unwrap();
}

pub type Token<'a> = &'a str;

pub trait Tokenizer: Send + Sync {
    fn tokenize<'a>(&self, text: &'a mut String) -> Vec<Token<'a>>;
}

pub struct RougeTokenizer;

/// Original tokenize function from ROUGE by Chin-Yew Lin
impl Tokenizer for RougeTokenizer {
    fn tokenize<'a>(&self, text: &'a mut String) -> Vec<&'a str> {
        // Convert everything to lowercase in-place
        text.make_ascii_lowercase();

        // Replace any non-alpha-numeric characters with spaces in-place
        *text = NON_ALPHANUM_RE.replace_all(text, " ").to_string();

        SPACES_RE
            .split(text)
            .filter(|token| VALID_TOKEN_RE.is_match(token))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TOKENIZER: RougeTokenizer = RougeTokenizer;
    }

    #[test]
    fn basic_test() {
        let mut text = String::from("Hello123");
        assert_eq!(TOKENIZER.tokenize(&mut text), vec!["hello123"]);
    }

    #[test]
    fn special_characters() {
        let mut text = String::from("Hello!@#$%^&*123");
        assert_eq!(TOKENIZER.tokenize(&mut text), vec!["hello", "123"]);
    }

    #[test]
    fn only_spaces() {
        let mut text = String::from("     ");
        assert_eq!(TOKENIZER.tokenize(&mut text), Vec::<&str>::new());
    }

    #[test]
    fn empty_string() {
        let mut text = String::new();
        assert_eq!(TOKENIZER.tokenize(&mut text), Vec::<&str>::new());
    }

    #[test]
    fn numeric_only() {
        let mut text = String::from("123456");
        assert_eq!(TOKENIZER.tokenize(&mut text), vec!["123456"]);
    }

    #[test]
    fn alphabets_only() {
        let mut text = String::from("abcdef");
        assert_eq!(TOKENIZER.tokenize(&mut text), vec!["abcdef"]);
    }

    #[test]
    fn mixed_case() {
        let mut text = String::from("HelloWorld");
        assert_eq!(TOKENIZER.tokenize(&mut text), vec!["helloworld"]);
    }

    #[test]
    fn continuous_non_alphanumeric() {
        let mut text = String::from("Hello***World123");
        assert_eq!(TOKENIZER.tokenize(&mut text), vec!["hello", "world123"]);
    }

    #[test]
    fn leading_and_trailing_spaces() {
        let mut text = String::from("  hello world 123  ");
        assert_eq!(TOKENIZER.tokenize(&mut text), vec!["hello", "world", "123"]);
    }

    #[test]
    fn unicode_characters() {
        let mut text = String::from("こんにちは123");
        assert_eq!(TOKENIZER.tokenize(&mut text), vec!["123"]); // Assuming non-ASCII characters are not considered alphanumeric
    }
}

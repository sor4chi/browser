use crate::{attribute::Attribute, tag::Tag};
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    StartTag(Tag, Vec<Attribute>),
    EndTag(Tag),
    Text(String),
}

struct Tokenizer<'a> {
    input: Peekable<std::str::Chars<'a>>,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            position: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.position += 1;
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn parse_text(&mut self) -> Token {
        let mut text = String::new();
        while let Some(current_char) = self.peek_char() {
            if *current_char == '<' {
                break;
            }
            text.push(self.next_char().unwrap());
        }
        Token::Text(text)
    }

    fn parse_tag(&mut self) -> Token {
        let a = self.next_char(); // <
        if self.peek_char() == Some(&'/') {
            // end tag
            self.next_char(); // /
            let tag = self.parse_tag_name();
            self.next_char(); // >
            Token::EndTag(tag)
        } else {
            // start tag
            let tag = self.parse_tag_name();
            if self.peek_char() == Some(&'>') {
                self.next_char(); // >
                return Token::StartTag(tag, vec![]);
            }
            self.next_char(); // space after tag name
            let attributes = self.parse_attributes();
            self.next_char(); // >
            Token::StartTag(tag, attributes)
        }
    }

    fn parse_tag_name(&mut self) -> Tag {
        let mut tag_name = String::new();
        while let Some(current_char) = self.peek_char() {
            if *current_char == ' ' || *current_char == '>' {
                break;
            }
            tag_name.push(self.next_char().unwrap());
        }
        Tag::new(&tag_name)
    }

    fn parse_attributes(&mut self) -> Vec<Attribute> {
        let mut attributes = vec![];
        while let Some(current_char) = self.peek_char() {
            if *current_char == '>' {
                break;
            }
            if *current_char == ' ' {
                self.next_char();
                continue;
            }
            attributes.push(self.parse_attribute());
        }
        attributes
    }

    fn parse_attribute(&mut self) -> Attribute {
        let name = self.parse_attribute_name();
        self.next_char();
        let value = self.parse_attribute_value();
        Attribute::new(&name, &value)
    }

    fn parse_attribute_name(&mut self) -> String {
        let mut name = String::new();
        while let Some(current_char) = self.peek_char() {
            if *current_char == '=' {
                break;
            }
            name.push(self.next_char().unwrap());
        }
        name
    }

    fn parse_attribute_value(&mut self) -> String {
        self.next_char(); // "
        let mut value = String::new();
        while let Some(current_char) = self.peek_char() {
            if *current_char == '"' {
                self.next_char(); // "
                break;
            }
            value.push(self.next_char().unwrap());
        }
        value
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let next_char = self.peek_char();
        next_char?;
        match *next_char.unwrap() {
            '<' => Some(self.parse_tag()),
            _ => Some(self.parse_text()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_parse_text() {
        let input = r#"Hello, world!<"#; // < is not included in the text
        let expected = Token::Text("Hello, world!".to_string());
        let mut tokenizer = Tokenizer::new(input);
        let actual = tokenizer.parse_text();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_parse_start_tag() {
        let input = r#"<html>"#;
        let expected = Token::StartTag(Tag::Html, vec![]);
        let mut tokenizer = Tokenizer::new(input);
        let actual = tokenizer.parse_tag();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_parse_end_tag() {
        let input = r#"</html>"#;
        let expected = Token::EndTag(Tag::Html);
        let mut tokenizer = Tokenizer::new(input);
        let actual = tokenizer.parse_tag();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_parse_attribute_name() {
        let input = r#"class="#;
        let expected = "class".to_string();
        let mut tokenizer = Tokenizer::new(input);
        let actual = tokenizer.parse_attribute_name();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_parse_attribute_value() {
        let input = r#""test""#;
        let expected = "test".to_string();
        let mut tokenizer = Tokenizer::new(input);
        let actual = tokenizer.parse_attribute_value();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_parse_attribute() {
        let input = r#"class="test""#;
        let expected = Attribute::Class("test".to_string());
        let mut tokenizer = Tokenizer::new(input);
        let actual = tokenizer.parse_attribute();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_parse_attributes() {
        let input = r#"class="test" id="test""#;
        let expected = vec![
            Attribute::Class("test".to_string()),
            Attribute::Id("test".to_string()),
        ];
        let mut tokenizer = Tokenizer::new(input);
        let actual = tokenizer.parse_attributes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_parse_tag_name() {
        let input = r#"html"#;
        let expected = Tag::Html;
        let mut tokenizer = Tokenizer::new(input);
        let actual = tokenizer.parse_tag_name();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_parse_tag() {
        let input = r#"<html class="test" id="test">"#;
        let expected = Token::StartTag(
            Tag::Html,
            vec![
                Attribute::Class("test".to_string()),
                Attribute::Id("test".to_string()),
            ],
        );
        let mut tokenizer = Tokenizer::new(input);
        let actual = tokenizer.parse_tag();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_next_token() {
        let input = r#"Hello, world!<html class="test" id="test">"#;
        let expected = vec![
            Token::Text("Hello, world!".to_string()),
            Token::StartTag(
                Tag::Html,
                vec![
                    Attribute::Class("test".to_string()),
                    Attribute::Id("test".to_string()),
                ],
            ),
        ];
        let mut tokenizer = Tokenizer::new(input);
        let actual = vec![
            tokenizer.next_token().unwrap(),
            tokenizer.next_token().unwrap(),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_simple_html() {
        let input = r#"<html><head><title>Test Page</title></head><body><h1>Hello, world!</h1></body></html>"#;
        let expected = vec![
            Token::StartTag(Tag::Html, vec![]),
            Token::StartTag(Tag::Head, vec![]),
            Token::StartTag(Tag::Title, vec![]),
            Token::Text("Test Page".to_string()),
            Token::EndTag(Tag::Title),
            Token::EndTag(Tag::Head),
            Token::StartTag(Tag::Body, vec![]),
            Token::StartTag(Tag::H1, vec![]),
            Token::Text("Hello, world!".to_string()),
            Token::EndTag(Tag::H1),
            Token::EndTag(Tag::Body),
            Token::EndTag(Tag::Html),
        ];
        let mut tokenizer = Tokenizer::new(input);
        let mut actual = vec![];
        while let Some(token) = tokenizer.next_token() {
            actual.push(token);
        }
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_simple_html_with_attributes() {
        let input = r#"<html><head><title class="title">Test Page</title></head><body><h1 class="header">Hello, world!</h1></body></html>"#;
        let expected = vec![
            Token::StartTag(Tag::Html, vec![]),
            Token::StartTag(Tag::Head, vec![]),
            Token::StartTag(Tag::Title, vec![Attribute::Class("title".to_string())]),
            Token::Text("Test Page".to_string()),
            Token::EndTag(Tag::Title),
            Token::EndTag(Tag::Head),
            Token::StartTag(Tag::Body, vec![]),
            Token::StartTag(Tag::H1, vec![Attribute::Class("header".to_string())]),
            Token::Text("Hello, world!".to_string()),
            Token::EndTag(Tag::H1),
            Token::EndTag(Tag::Body),
            Token::EndTag(Tag::Html),
        ];
        let mut tokenizer = Tokenizer::new(input);
        let mut actual = vec![];
        while let Some(token) = tokenizer.next_token() {
            actual.push(token);
        }
        assert_eq!(expected, actual);
    }
}

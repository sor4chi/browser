use crate::tag::Tag;

#[derive(Debug, PartialEq)]
pub enum Token {
    StartTag(Tag, Vec<(String, String)>),
    EndTag(Tag),
    Text(String),
}

struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.position >= self.input.len() {
            return None;
        }

        let next_opening = self.input[self.position..].find('<');
        if let Some(next_opening) = next_opening {
            if next_opening > 0 {
                let text = &self.input[self.position..self.position + next_opening];
                self.position += next_opening;
                return Some(Token::Text(text.to_string()));
            }

            self.position += next_opening + 1;
            if self.input[self.position..].starts_with('/') {
                self.position += 1;
                let next_closing = self.input[self.position..].find('>');
                if let Some(next_closing) = next_closing {
                    let tag = &self.input[self.position..self.position + next_closing];
                    self.position += next_closing + 1;
                    return Some(Token::EndTag(Tag::from_str(tag)));
                }
            } else {
                let next_closing = self.input[self.position..].find('>');
                if let Some(next_closing) = next_closing {
                    let tag = &self.input[self.position..self.position + next_closing];
                    self.position += next_closing + 1;
                    return Some(Token::StartTag(Tag::from_str(tag), vec![]));
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

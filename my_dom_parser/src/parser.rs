use crate::attribute::Attribute;
use crate::tag::Tag;
use crate::tokenizer::{Token, Tokenizer};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Node {
    Element(ElementData),
    Text(String),
}

#[derive(PartialEq, Debug)]
pub struct ElementData {
    pub tag_name: Tag,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl Parser<'_> {
    pub fn new(input: &str) -> Parser {
        Parser {
            tokenizer: Tokenizer::new(input),
        }
    }

    pub fn parse_nodes(&mut self) -> Result<Vec<Node>, String> {
        panic!("Not implemented yet");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_parse_nodes() {
        let input = r#"<html><head><title>Test Page</title></head><body><h1>Hello, world!</h1></body></html>"#;
        let expected = vec![Node::Element(ElementData {
            tag_name: Tag::Html,
            attributes: HashMap::new(),
            children: vec![
                Node::Element(ElementData {
                    tag_name: Tag::Head,
                    attributes: HashMap::new(),
                    children: vec![Node::Element(ElementData {
                        tag_name: Tag::Title,
                        attributes: HashMap::new(),
                        children: vec![Node::Text("Test Page".to_string())],
                    })],
                }),
                Node::Element(ElementData {
                    tag_name: Tag::Body,
                    attributes: HashMap::new(),
                    children: vec![Node::Element(ElementData {
                        tag_name: Tag::H1,
                        attributes: HashMap::new(),
                        children: vec![Node::Text("Hello, world!".to_string())],
                    })],
                }),
            ],
        })];
        let mut parser = Parser::new(input);
        let actual = parser.parse_nodes().expect("Failed to parse nodes");
        assert_eq!(expected, actual);
    }
}

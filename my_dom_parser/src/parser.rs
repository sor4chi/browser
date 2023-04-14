#[derive(Debug, PartialEq)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: Vec<(String, String)>,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Element(ElementData, Vec<Node>),
    Text(String),
}

pub fn parse(input: &str) -> Node {
    let mut nodes = vec![];

    let mut remaining = input;
    while let Some(next_opening) = remaining.find('<') {
        if next_opening > 0 {
            let text = &remaining[..next_opening];
            nodes.push(Node::Text(text.to_string()));
        }

        remaining = &remaining[next_opening..];
        if let Some(next_closing) = remaining.find('>') {
            let tag = &remaining[1..next_closing];
            nodes.push(Node::Element(
                ElementData {
                    tag_name: tag.to_string(),
                    attributes: vec![],
                },
                vec![],
            ));
            remaining = &remaining[next_closing + 1..];
        }
    }

    Node::Element(
        ElementData {
            tag_name: "html".to_string(),
            attributes: vec![],
        },
        nodes,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_html() {
        let input = r#"<html><head><title>Test Page</title></head><body><h1>Hello, world!</h1></body></html>"#;
        let expected = Node::Element(
            ElementData {
                tag_name: "html".to_string(),
                attributes: vec![],
            },
            vec![
                Node::Element(
                    ElementData {
                        tag_name: "head".to_string(),
                        attributes: vec![],
                    },
                    vec![Node::Element(
                        ElementData {
                            tag_name: "title".to_string(),
                            attributes: vec![],
                        },
                        vec![Node::Text("Test Page".to_string())],
                    )],
                ),
                Node::Element(
                    ElementData {
                        tag_name: "body".to_string(),
                        attributes: vec![],
                    },
                    vec![Node::Element(
                        ElementData {
                            tag_name: "h1".to_string(),
                            attributes: vec![],
                        },
                        vec![Node::Text("Hello, world!".to_string())],
                    )],
                ),
            ],
        );

        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_simple_html_with_text() {
        let input = r#"<html><head><title>Test Page</title></head><body><h1>Hello, world!</h1><p>Some text</p></body></html>"#;
        let expected = Node::Element(
            ElementData {
                tag_name: "html".to_string(),
                attributes: vec![],
            },
            vec![
                Node::Element(
                    ElementData {
                        tag_name: "head".to_string(),
                        attributes: vec![],
                    },
                    vec![Node::Element(
                        ElementData {
                            tag_name: "title".to_string(),
                            attributes: vec![],
                        },
                        vec![Node::Text("Test Page".to_string())],
                    )],
                ),
                Node::Element(
                    ElementData {
                        tag_name: "body".to_string(),
                        attributes: vec![],
                    },
                    vec![
                        Node::Element(
                            ElementData {
                                tag_name: "h1".to_string(),
                                attributes: vec![],
                            },
                            vec![Node::Text("Hello, world!".to_string())],
                        ),
                        Node::Element(
                            ElementData {
                                tag_name: "p".to_string(),
                                attributes: vec![],
                            },
                            vec![Node::Text("Some text".to_string())],
                        ),
                    ],
                ),
            ],
        );

        let result = parse(input);
        assert_eq!(result, expected);
    }
}

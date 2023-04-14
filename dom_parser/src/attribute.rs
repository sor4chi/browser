#[derive(PartialEq, Debug)]
pub enum Attribute {
    Class(String),
    Id(String),
    Unknown,
}

impl Attribute {
    pub fn new(attribute: &str, value: &str) -> Self {
        match attribute {
            "class" => Self::Class(value.to_string()),
            "id" => Self::Id(value.to_string()),
            _ => Self::Unknown,
        }
    }
}

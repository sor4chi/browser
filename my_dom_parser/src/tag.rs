#[derive(PartialEq, Debug)]
pub enum Tag {
    Html,
    Head,
    Title,
    Body,
    H1,
    P,
    Unknown,
}

impl Tag {
    pub fn from_str(tag: &str) -> Self {
        match tag {
            "html" => Self::Html,
            "head" => Self::Head,
            "title" => Self::Title,
            "body" => Self::Body,
            "h1" => Self::H1,
            "p" => Self::P,
            _ => Self::Unknown,
        }
    }
}

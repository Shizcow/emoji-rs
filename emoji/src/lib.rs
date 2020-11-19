#[derive(Debug, PartialEq)]
pub enum Status {
    Component,
    FullyQualified,
    MinimallyQualified,
    Unqualified,
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	use Status::*;
	write!(f, "{}", match self {
	    Component => "Component",
	    FullyQualified => "FullyQualified",
	    MinimallyQualified => "MinimallyQualified",
	    Unqualified => "Unqualified",
	})
    }
}

#[derive(Debug, PartialEq)]
pub struct Emoji {
    pub codepoint: &'static str,
    pub status: Status,
    pub glyph: &'static str,
    pub introduction_version: f32,
    pub name: &'static str,
    pub annotations: &'static [&'static str],
    pub variants: &'static [Emoji],
}

include!("emoji_data.rs");

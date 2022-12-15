use crossterm::style;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    None,
    Number,
    Match,
    String,
}

impl Type {
    pub fn to_color(&self) -> style::Color {
        match self {
            Self::Number => style::Color::Red,
            Self::Match => style::Color::Blue,
            Self::String => style::Color::Green,
            _ => style::Color::White,
        }
    }
}

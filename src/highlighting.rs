use crossterm::style;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
}

impl Type {
    pub fn to_color(&self) -> style::Color {
        match self {
            Self::Number => style::Color::Red,
            Self::Match => style::Color::Blue,
            Self::String => style::Color::Green,
            Self::Character => style::Color::Green,
            Self::Comment => style::Color::Grey,
            _ => style::Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }
}

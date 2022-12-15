use crossterm::style;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
    MultilineComment,
    PrimaryKeywords,
    SecondaryKeywords,
}

impl Type {
    pub fn to_color(self) -> style::Color {
        match self {
            Self::Number => style::Color::Red,
            Self::Match => style::Color::Blue,
            Self::String => style::Color::Green,
            Self::Character => style::Color::Green,
            Self::Comment => style::Color::Grey,
            Self::MultilineComment => style::Color::Grey,
            Self::PrimaryKeywords => style::Color::Yellow,
            Self::SecondaryKeywords => style::Color::Cyan,
            _ => style::Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }
}

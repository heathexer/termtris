use tui::{
    style::{Color, Style},
    text::Span,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoardColor {
    Empty = 0,
    Ghost,
    LightBlue,
    DarkBlue,
    Red,
    Purple,
    Orange,
    Green,
    Yellow,
}

pub fn cell_to_span(content: &str, color: BoardColor) -> Span {
    Span::styled(
        content,
        Style::default().fg(match color {
            BoardColor::Ghost => Color::LightYellow,
            BoardColor::LightBlue => Color::Cyan,
            BoardColor::DarkBlue => Color::Blue,
            BoardColor::Red => Color::Red,
            BoardColor::Purple => Color::Magenta,
            BoardColor::Orange => Color::LightRed,
            BoardColor::Green => Color::Green,
            BoardColor::Yellow => Color::Yellow,
            BoardColor::Empty => Color::Black,
        }),
    )
}

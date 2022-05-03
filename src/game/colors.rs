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
    let color = match color {
        BoardColor::Ghost => Color::Rgb(128, 128, 128),
        BoardColor::LightBlue => Color::Rgb(60, 160, 160),
        BoardColor::DarkBlue => Color::Rgb(60, 60, 200),
        BoardColor::Red => Color::Rgb(160, 60, 60),
        BoardColor::Purple => Color::Rgb(180, 60, 180),
        BoardColor::Orange => Color::Rgb(203, 80, 60),
        BoardColor::Green => Color::Rgb(60, 160, 60),
        BoardColor::Yellow => Color::Rgb(200, 150, 60),
        BoardColor::Empty => {
            return Span::raw(content);
        }
    };

    Span::styled(content, Style::default().fg(color).bg(color))
}

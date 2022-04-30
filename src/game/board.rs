use super::{colors::BoardColor, Game};

use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

#[derive(Clone)]
pub struct Board(pub [[BoardColor; Game::WIDTH]; Game::HEIGHT]);

impl<'a> Into<Paragraph<'a>> for Board {
    fn into(self) -> Paragraph<'a> {
        let mut text = Vec::new();
        self.0.iter().enumerate().for_each(|(i, row)| {
            if i < Game::HEIGHT - Game::DISPLAY_HEIGHT {
                let mut text_row = Vec::<Span>::new();

                row.iter().for_each(|cell| {
                    text_row.push(match *cell {
                        BoardColor::Empty => Span::raw("  "),
                        color => Span::styled(
                            "██",
                            Style::default().fg(match color {
                                BoardColor::LightBlue => Color::Cyan,
                                BoardColor::DarkBlue => Color::Blue,
                                BoardColor::Red => Color::Red,
                                BoardColor::Purple => Color::Magenta,
                                BoardColor::Orange => Color::LightRed,
                                BoardColor::Green => Color::Green,
                                BoardColor::Yellow => Color::Yellow,
                                BoardColor::Empty => panic!(),
                            }),
                        ),
                    });
                });

                text.push(Spans::from(text_row));
            }
        });

        text.reverse();
        Paragraph::new(text)
    }
}

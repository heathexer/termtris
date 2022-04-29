use super::colors::BoardColor;

use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

#[derive(Clone)]
pub struct Board(pub [[BoardColor; 10]; 16]);

impl<'a> Into<Paragraph<'a>> for Board {
    fn into(self) -> Paragraph<'a> {
        let mut text = Vec::new();
        self.0.iter().for_each(|row| {
            // let mut text_row = String::new();
            let mut text_row = Vec::<Span>::new();

            row.iter().for_each(|cell| {
                // text_row += if *cell { "██" } else { "[]" };
                text_row.push(match *cell {
                    BoardColor::Empty => Span::raw("[]"),
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
        });

        Paragraph::new(text)
    }
}

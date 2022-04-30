use super::{
    colors::{self, BoardColor},
    Game,
};

use tui::{
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
                        BoardColor::Empty => colors::cell_to_span("  ", BoardColor::Empty),
                        color => colors::cell_to_span("██", color),
                    });
                });

                text.push(Spans::from(text_row));
            }
        });

        text.reverse();
        Paragraph::new(text)
    }
}

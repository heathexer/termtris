use super::{
    colors::{self, BoardColor},
    Game,
};

use tui::{
    text::{Span, Spans},
    widgets::Paragraph,
};

// Struct to store the state of the game board with an impl to turn it into a Paragrpah for rendering
#[derive(Clone)]
pub struct Board(pub [[BoardColor; Game::WIDTH]; Game::HEIGHT]);

impl Board {
    pub fn reset(&mut self) {
        self.0 = [[BoardColor::Empty; Game::WIDTH]; Game::HEIGHT];
    }
}

impl<'a> From<Board> for Paragraph<'a> {
    fn from(board: Board) -> Paragraph<'a> {
        let mut text = Vec::new();

        board.0.iter().enumerate().for_each(|(i, row)| {
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

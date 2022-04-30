use rand::seq::SliceRandom;
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

#[derive(Clone, Eq, PartialEq)]
pub struct Piece {
    pub shapes: [[[u8; 4]; 4]; 4],
    pub color: BoardColor,
}

// pub enum Piece {
//     O,
//     L,
// }

use super::colors::BoardColor;

impl Piece {
    pub const O: Piece = Piece {
        shapes: [
            [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        ],
        color: BoardColor::Yellow,
    };
    pub const I: Piece = Piece {
        shapes: [
            [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]],
            [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
            [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
        ],
        color: BoardColor::LightBlue,
    };
    pub const S: Piece = Piece {
        shapes: [
            [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
            [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
            [[1, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        ],
        color: BoardColor::Red,
    };
    pub const Z: Piece = Piece {
        shapes: [
            [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 0, 1, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
            [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
            [[0, 1, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]],
        ],
        color: BoardColor::Green,
    };
    pub const T: Piece = Piece {
        shapes: [
            [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
            [[0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        ],
        color: BoardColor::Purple,
    };
    pub const L: Piece = Piece {
        shapes: [
            [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
            [[0, 0, 0, 0], [1, 1, 1, 0], [1, 0, 0, 0], [0, 0, 0, 0]],
            [[1, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        ],
        color: BoardColor::Orange,
    };
    pub const J: Piece = Piece {
        shapes: [
            [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
            [[0, 0, 0, 0], [1, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
            [[0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
        ],
        color: BoardColor::DarkBlue,
    };

    pub const ALL: [&'static Piece; 7] = [
        &Self::O,
        &Self::I,
        &Self::S,
        &Self::Z,
        &Self::T,
        &Self::L,
        &Self::J,
    ];

    pub fn random_bag<'a>() -> [&'a Piece; 7] {
        let mut permutation = Piece::ALL.clone();
        permutation.shuffle(&mut rand::thread_rng());

        permutation
    }
}

impl<'a> Into<Paragraph<'a>> for &Piece {
    fn into(self) -> Paragraph<'a> {
        let mut text = Vec::new();
        text.push(Spans::from(Span::raw("")));

        self.shapes[1].iter().for_each(|row| {
            let mut text_row = Vec::<Span>::new();

            row.iter().enumerate().for_each(|(i, cell)| {
                // Only draw the middle two columns, always covers every square of pieces in orientation 1
                if 0 < i && i < 3 {
                    text_row.push(if *cell != 0 {
                        Span::styled(
                            "██",
                            Style::default().fg(match self.color {
                                BoardColor::LightBlue => Color::Cyan,
                                BoardColor::DarkBlue => Color::Blue,
                                BoardColor::Red => Color::Red,
                                BoardColor::Purple => Color::Magenta,
                                BoardColor::Orange => Color::LightRed,
                                BoardColor::Green => Color::Green,
                                BoardColor::Yellow => Color::Yellow,
                                BoardColor::Empty => panic!(),
                            }),
                        )
                    } else {
                        Span::raw("  ")
                    });
                }
            });

            text.push(Spans::from(text_row));
        });

        Paragraph::new(text)
    }
}

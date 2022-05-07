use rand::seq::SliceRandom;
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};

#[derive(Clone, Eq, PartialEq)]
pub struct Piece {
    pub shapes: [[[u8; 4]; 4]; 4],
    pub color: BoardColor,
    pub kicks: [[[(isize, isize); 4]; 4]; 4],
}

use super::colors::{self, BoardColor};

impl Piece {
    // Kick data from here https://tetris.fandom.com/wiki/SRS#Wall_Kicks
    const O_KICKS: [[[(isize, isize); 4]; 4]; 4] = [[[(0, 0); 4]; 4]; 4];

    const I_KICKS: [[[(isize, isize); 4]; 4]; 4] = [
        [
            [(0, 0), (0, 0), (0, 0), (0, 0)],
            [(-2, 0), (1, 0), (-2, -1), (1, 2)],
            [(0, 0), (0, 0), (0, 0), (0, 0)],
            [(-1, 0), (2, 0), (-1, 2), (2, -1)],
        ],
        [
            [(2, 0), (-1, 0), (2, 1), (-1, -2)],
            [(0, 0), (0, 0), (0, 0), (0, 0)],
            [(-1, 0), (2, 0), (-1, 2), (2, -1)],
            [(0, 0), (0, 0), (0, 0), (0, 0)],
        ],
        [
            [(0, 0), (0, 0), (0, 0), (0, 0)],
            [(1, 0), (-2, 0), (1, -2), (-2, 1)],
            [(0, 0), (0, 0), (0, 0), (0, 0)],
            [(2, 0), (-1, 0), (2, 1), (-1, -2)],
        ],
        [
            [(1, 0), (-2, 0), (1, -2), (-2, 1)],
            [(0, 0), (0, 0), (0, 0), (0, 0)],
            [(-2, 0), (1, 0), (-2, -1), (1, 2)],
            [(0, 0), (0, 0), (0, 0), (0, 0)],
        ],
    ];

    const NORMAL_KICKS: [[[(isize, isize); 4]; 4]; 4] = [
        [
            [(0, 0), (0, 0), (0, 0), (0, 0)],
            [(-1, 0), (-1, 1), (0, -2), (-1, -2)],
            [(0, 0), (0, 0), (0, 0), (0, 0)],
            [(1, 0), (1, 1), (0, -2), (1, -2)],
        ],
        [
            [(1, 0), (1, -1), (0, 2), (1, 2)],
            [(0, 0), (0, 0), (0, 0), (0, 0)],
            [(1, 0), (1, -1), (0, 2), (1, 2)],
            [(0, 0), (0, 0), (0, 0), (0, 0)],
        ],
        [
            [(0, 0), (0, 0), (0, 0), (0, 0)],
            [(-1, 0), (-1, 1), (0, -2), (-1, -2)],
            [(0, 0), (0, 0), (0, 0), (0, 0)],
            [(1, 0), (1, 1), (0, -2), (1, -2)],
        ],
        [
            [(-1, 0), (-1, -1), (0, 2), (-1, 2)],
            [(0, 0), (0, 0), (0, 0), (0, 0)],
            [(-1, 0), (-1, -1), (0, 2), (-1, 2)],
            [(0, 0), (0, 0), (0, 0), (0, 0)],
        ],
    ];

    // Piece shape data from here https://tetris.fandom.com/wiki/SRS?file=SRS-pieces.png
    pub const O: Piece = Piece {
        shapes: [
            [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        ],
        color: BoardColor::Yellow,
        kicks: Self::O_KICKS,
    };
    pub const I: Piece = Piece {
        shapes: [
            [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]],
            [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
            [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
        ],
        color: BoardColor::LightBlue,
        kicks: Self::I_KICKS,
    };
    pub const S: Piece = Piece {
        shapes: [
            [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
            [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
            [[1, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        ],
        color: BoardColor::Red,
        kicks: Self::NORMAL_KICKS,
    };
    pub const Z: Piece = Piece {
        shapes: [
            [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 0, 1, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
            [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
            [[0, 1, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]],
        ],
        color: BoardColor::Green,
        kicks: Self::NORMAL_KICKS,
    };
    pub const T: Piece = Piece {
        shapes: [
            [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
            [[0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        ],
        color: BoardColor::Purple,
        kicks: Self::NORMAL_KICKS,
    };
    pub const L: Piece = Piece {
        shapes: [
            [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
            [[0, 0, 0, 0], [1, 1, 1, 0], [1, 0, 0, 0], [0, 0, 0, 0]],
            [[1, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        ],
        color: BoardColor::Orange,
        kicks: Self::NORMAL_KICKS,
    };
    pub const J: Piece = Piece {
        shapes: [
            [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 1, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
            [[0, 0, 0, 0], [1, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
            [[0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
        ],
        color: BoardColor::DarkBlue,
        kicks: Self::NORMAL_KICKS,
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
        // Ensures an even distribution of pieces by giving one of each type per 7
        let mut permutation = Piece::ALL.clone();
        permutation.shuffle(&mut rand::thread_rng());

        permutation

        // [&Piece::I; 7]
    }
}

impl<'a> Into<Vec<Spans<'a>>> for &Piece {
    fn into(self) -> Vec<Spans<'a>> {
        let mut text = Vec::new();
        text.push(Spans::from(Span::styled(
            "",
            Style::default().fg(Color::Gray),
        )));

        self.shapes[0].iter().take(2).for_each(|row| {
            let mut text_row = Vec::<Span>::new();

            match self {
                &Piece::O | &Piece::I => {}
                _ => {
                    // Add a half block of space to center odd width blocks
                    text_row.push(Span::raw(" "));
                }
            }

            row.iter().for_each(|cell| {
                text_row.push(if *cell != 0 {
                    colors::cell_to_span("██", self.color)
                } else {
                    Span::raw("  ")
                });
            });

            text.push(Spans::from(text_row));
        });

        text
    }
}

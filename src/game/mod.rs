pub mod board;
pub mod colors;
pub mod piece;

use tui::widgets::Paragraph;

use self::{board::Board, colors::BoardColor, piece::Piece};

pub struct Game {
    board: Board,
    cur_piece: Piece,
    next_piece: Piece,
    hold_piece: Piece,
    cur_rotation: u8,
    piece_offset: (usize, usize),
}

impl<'a> Game {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 16;

    pub fn new() -> Self {
        let board = Board([[BoardColor::Empty; Game::WIDTH]; Game::HEIGHT]);
        let cur_piece = Piece::I;
        let next_piece = Piece::I;
        let hold_piece = Piece::J;
        let cur_rotation = 0_u8;
        let piece_offset = (3_usize, 3_usize);
        Game {
            board,
            cur_piece,
            next_piece,
            hold_piece,
            cur_rotation,
            piece_offset,
        }
    }

    pub fn get_board_paragraph(&self) -> Paragraph<'a> {
        let mut board_copy = self.board.clone();

        for (i, row) in self.cur_piece.shapes[self.cur_rotation as usize]
            .iter()
            .enumerate()
        {
            let row_idx = self.piece_offset.0 + i;
            for (j, cell) in row.iter().enumerate() {
                let col_idx = self.piece_offset.1 + j;
                if *cell != 0 && row_idx < Game::HEIGHT && col_idx < Game::WIDTH {
                    board_copy.0[row_idx][col_idx] = self.cur_piece.color;
                }
            }
        }

        board_copy.into()
    }

    pub fn move_left(&mut self) {
        let new_offset = (
            self.piece_offset.0 as isize,
            self.piece_offset.1 as isize - 1,
        );
        if self.try_move(new_offset, self.cur_rotation) {
            self.piece_offset.1 = new_offset.1 as usize;
        }
    }

    pub fn move_right(&mut self) {
        let new_offset = (
            self.piece_offset.0 as isize,
            self.piece_offset.1 as isize + 1,
        );
        if self.try_move(new_offset, self.cur_rotation) {
            self.piece_offset.1 = new_offset.1 as usize;
        }
    }

    pub fn move_down(&mut self) {
        let new_offset = (
            self.piece_offset.0 as isize + 1,
            self.piece_offset.1 as isize,
        );
        if self.try_move(new_offset, self.cur_rotation) {
            self.piece_offset.0 = new_offset.0 as usize;
        } else {
            self.lock_piece();
        }
    }

    pub fn rotate_left(&mut self) {
        let new_rotation = (self.cur_rotation + 3) % 4;
        if self.try_move(
            (self.piece_offset.0 as isize, self.piece_offset.1 as isize),
            new_rotation,
        ) {
            self.cur_rotation = new_rotation;
        }
    }

    pub fn rotate_right(&mut self) {
        let new_rotation = (self.cur_rotation + 1) % 4;
        if self.try_move(
            (self.piece_offset.0 as isize, self.piece_offset.1 as isize),
            new_rotation,
        ) {
            self.cur_rotation = new_rotation;
        }
    }

    fn try_move(&self, offset: (isize, isize), rotation: u8) -> bool {
        for (i, row) in self.cur_piece.shapes[rotation as usize].iter().enumerate() {
            let row_idx = offset.0 + i as isize;
            for (j, cell) in row.iter().enumerate() {
                let col_idx = offset.1 + j as isize;
                if *cell != 0
                    && (row_idx < 0
                        || row_idx >= Game::HEIGHT as isize
                        || col_idx < 0
                        || col_idx >= Game::WIDTH as isize
                        || self.board.0[row_idx as usize][col_idx as usize] != BoardColor::Empty)
                {
                    return false;
                }
            }
        }
        true
    }

    fn lock_piece(&mut self) {
        for (i, row) in self.cur_piece.shapes[self.cur_rotation as usize]
            .iter()
            .enumerate()
        {
            let row_idx = self.piece_offset.0 + i;
            for (j, cell) in row.iter().enumerate() {
                let col_idx = self.piece_offset.1 + j;
                if *cell != 0 {
                    self.board.0[row_idx][col_idx] = self.cur_piece.color;
                }
            }
        }

        self.cur_piece = self.next_piece;
        self.next_piece = Piece::random();
        self.piece_offset = (0, 4);
        self.cur_rotation = 0;
    }
}

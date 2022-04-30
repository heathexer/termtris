pub mod board;
pub mod colors;
pub mod piece;

use tui::{
    text::{Span, Spans},
    widgets::Paragraph,
};

use self::{board::Board, colors::BoardColor, piece::Piece};

pub struct Game<'a> {
    board: Board,
    piece_bag: Vec<&'a Piece>,
    cur_piece: &'a Piece,
    next_piece: &'a Piece,
    hold_piece: Option<&'a Piece>,
    can_hold: bool,
    cur_rotation: u8,
    piece_offset: (isize, isize),
}

impl<'a> Game<'a> {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 40;
    pub const DISPLAY_HEIGHT: usize = 20;

    pub fn new() -> Self {
        let board = Board([[BoardColor::Empty; Game::WIDTH]; Game::HEIGHT]);
        let mut piece_bag = Vec::from(Piece::random_bag());
        let cur_piece = piece_bag.pop().unwrap();
        let next_piece = piece_bag.pop().unwrap();
        let hold_piece = None;
        let can_hold = true;
        let cur_rotation = 0_u8;
        let piece_offset = (Game::DISPLAY_HEIGHT as isize + 2, 4_isize);

        Game {
            board,
            cur_piece,
            next_piece,
            hold_piece,
            can_hold,
            cur_rotation,
            piece_offset,
            piece_bag,
        }
    }

    pub fn get_board_paragraph(&self) -> Paragraph<'a> {
        let mut board_copy = self.board.clone();

        for (i, row) in self.cur_piece.shapes[self.cur_rotation as usize]
            .iter()
            .enumerate()
        {
            let row_idx = self.piece_offset.0 - i as isize;
            for (j, cell) in row.iter().enumerate() {
                let col_idx = self.piece_offset.1 + j as isize;
                if *cell != 0
                    && row_idx < Game::HEIGHT as isize
                    && row_idx >= 0
                    && col_idx < Game::WIDTH as isize
                    && col_idx >= 0
                {
                    board_copy.0[row_idx as usize][col_idx as usize] = self.cur_piece.color;
                }
            }
        }

        board_copy.into()
    }

    pub fn next_piece_paragraph(&self) -> Paragraph<'a> {
        self.next_piece.into()
    }

    pub fn hold_piece_paragraph(&self) -> Paragraph<'a> {
        if let Some(piece) = self.hold_piece {
            piece.into()
        } else {
            Paragraph::new(Spans::from(Span::raw("")))
        }
    }

    pub fn move_left(&mut self) {
        let new_offset = (
            self.piece_offset.0 as isize,
            self.piece_offset.1 as isize - 1,
        );
        if self.try_move(new_offset, self.cur_rotation) {
            self.piece_offset.1 = new_offset.1;
        }
    }

    pub fn move_right(&mut self) {
        let new_offset = (
            self.piece_offset.0 as isize,
            self.piece_offset.1 as isize + 1,
        );
        if self.try_move(new_offset, self.cur_rotation) {
            self.piece_offset.1 = new_offset.1;
        }
    }

    pub fn move_down(&mut self) {
        let new_offset = (self.piece_offset.0 - 1, self.piece_offset.1);
        if self.try_move(new_offset, self.cur_rotation) {
            self.piece_offset.0 = new_offset.0;
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

    pub fn hard_drop(&mut self) {
        let mut new_offset = self.piece_offset;

        while self.try_move((new_offset.0 - 1, new_offset.1), self.cur_rotation) {
            new_offset.0 -= 1;
        }

        self.piece_offset = new_offset;
        self.lock_piece();
    }

    pub fn hold(&mut self) {
        if !self.can_hold {
            return;
        }

        if let Some(piece) = self.hold_piece {
            self.hold_piece = Some(self.cur_piece);
            self.cur_piece = piece;
        } else {
            self.hold_piece = Some(self.cur_piece);
            self.cur_piece = self.next_piece;

            if let Some(next_piece) = self.piece_bag.pop() {
                self.next_piece = next_piece;
            } else {
                self.piece_bag.append(&mut Vec::from(Piece::random_bag()));
                self.next_piece = self.piece_bag.pop().unwrap();
            }
        }

        self.can_hold = false;
        self.piece_offset = (21, 4);
        self.cur_rotation = 0;
    }

    fn is_line_full(row: &[BoardColor; Game::WIDTH]) -> bool {
        for color in row {
            if *color == BoardColor::Empty {
                return false;
            }
        }
        true
    }

    fn clear_lines(&mut self) {
        let mut new_board = self.board.clone();

        for i in 0..Game::HEIGHT {
            while Self::is_line_full(&new_board.0[i]) {
                new_board.0[i] = [BoardColor::Empty; Game::WIDTH];
                new_board.0.copy_within((i + 1).., i);
            }
        }

        self.board = new_board;
    }

    fn try_move(&self, offset: (isize, isize), rotation: u8) -> bool {
        for (i, row) in self.cur_piece.shapes[rotation as usize].iter().enumerate() {
            let row_idx = offset.0 - i as isize;
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
            let row_idx = self.piece_offset.0 - i as isize;
            for (j, cell) in row.iter().enumerate() {
                let col_idx = self.piece_offset.1 + j as isize;
                if *cell != 0 {
                    self.board.0[row_idx as usize][col_idx as usize] = self.cur_piece.color;
                }
            }
        }

        self.clear_lines();

        self.cur_piece = self.next_piece;

        if let Some(next_piece) = self.piece_bag.pop() {
            self.next_piece = next_piece;
        } else {
            self.piece_bag.append(&mut Vec::from(Piece::random_bag()));
            self.next_piece = self.piece_bag.pop().unwrap();
        }

        self.can_hold = true;
        self.piece_offset = (21, 4);
        self.cur_rotation = 0;
    }
}

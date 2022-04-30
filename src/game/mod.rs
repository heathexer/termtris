mod board;
mod colors;
mod piece;
mod score;

use tui::{
    text::{Span, Spans},
    widgets::Paragraph,
};

use self::{
    board::Board,
    colors::BoardColor,
    piece::Piece,
    score::{Score, ScoreEvent},
};

pub struct Game<'a> {
    board: Board,
    score: Score,
    piece_bag: Vec<&'a Piece>,
    cur_piece: &'a Piece,
    next_piece: &'a Piece,
    hold_piece: Option<&'a Piece>,
    can_hold: bool,
    cur_rotation: u8,
    piece_offset: (isize, isize),
    ghost_offset: (isize, isize),
}

impl<'a> Game<'a> {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 40;
    pub const DISPLAY_HEIGHT: usize = 20;

    pub fn new() -> Self {
        let board = Board([[BoardColor::Empty; Game::WIDTH]; Game::HEIGHT]);
        let score = Score::new();
        let mut piece_bag = Vec::from(Piece::random_bag());
        let cur_piece = piece_bag.pop().unwrap();
        let next_piece = piece_bag.pop().unwrap();
        let hold_piece = None;
        let can_hold = true;
        let cur_rotation = 0_u8;
        let piece_offset = (0, 0);
        let ghost_offset = (0, 0);

        let mut game = Game {
            board,
            score,
            cur_piece,
            next_piece,
            hold_piece,
            can_hold,
            cur_rotation,
            piece_offset,
            ghost_offset,
            piece_bag,
        };

        game.reset_piece(false);
        game
    }

    pub fn get_board_paragraph(&self) -> Paragraph<'a> {
        let mut board_copy = self.board.clone();

        for (i, row) in self.cur_piece.shapes[self.cur_rotation as usize]
            .iter()
            .enumerate()
        {
            let row_idx = self.piece_offset.0 - i as isize;
            let ghost_row_idx = self.ghost_offset.0 - i as isize;
            for (j, cell) in row.iter().enumerate() {
                let col_idx = self.piece_offset.1 + j as isize;
                let ghost_col_idx = self.ghost_offset.1 + j as isize;
                if *cell != 0
                    && row_idx < Game::HEIGHT as isize
                    && row_idx >= 0
                    && col_idx < Game::WIDTH as isize
                    && col_idx >= 0
                {
                    board_copy.0[ghost_row_idx as usize][ghost_col_idx as usize] =
                        BoardColor::Ghost;
                    board_copy.0[row_idx as usize][col_idx as usize] = self.cur_piece.color;
                }
            }
        }

        board_copy.into()
    }

    pub fn get_score_paragraph(&self) -> Paragraph<'a> {
        Paragraph::new(Spans::from(Span::raw(format!(
            "Score: {}",
            self.score.score
        ))))
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
        self.update_ghost_position();
    }

    pub fn move_right(&mut self) {
        let new_offset = (
            self.piece_offset.0 as isize,
            self.piece_offset.1 as isize + 1,
        );
        if self.try_move(new_offset, self.cur_rotation) {
            self.piece_offset.1 = new_offset.1;
        }
        self.update_ghost_position();
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

        self.try_rotate_with_kick(new_rotation);

        self.update_ghost_position();
    }

    pub fn rotate_right(&mut self) {
        let new_rotation = (self.cur_rotation + 1) % 4;

        self.try_rotate_with_kick(new_rotation);

        self.update_ghost_position();
    }

    pub fn hard_drop(&mut self) {
        self.piece_offset = self.ghost_offset;
        self.lock_piece();
    }

    pub fn hold(&mut self) {
        if !self.can_hold {
            return;
        }

        if let Some(piece) = self.hold_piece {
            self.hold_piece = Some(self.cur_piece);
            self.cur_piece = piece;
            self.reset_piece(false);
        } else {
            self.hold_piece = Some(self.cur_piece);
            self.reset_piece(true);
        }

        self.can_hold = false;
    }

    fn try_rotate_with_kick(&mut self, new_rotation: u8) {
        if self.try_move(self.piece_offset, new_rotation) {
            self.cur_rotation = new_rotation;
            return;
        }

        let mut new_offset;
        for (j, i) in self.cur_piece.kicks[self.cur_rotation as usize][new_rotation as usize] {
            new_offset = (self.piece_offset.0 + i, self.piece_offset.1 + j);
            if self.try_move(new_offset, new_rotation) {
                self.piece_offset = new_offset;
                self.cur_rotation = new_rotation;
                return;
            }
        }
    }

    fn reset_piece(&mut self, use_next_piece: bool) {
        if use_next_piece {
            self.cur_piece = self.next_piece;

            if let Some(next_piece) = self.piece_bag.pop() {
                self.next_piece = next_piece;
            } else {
                self.piece_bag.append(&mut Vec::from(Piece::random_bag()));
                self.next_piece = self.piece_bag.pop().unwrap();
            }
        }

        self.piece_offset = (21, 3);
        self.cur_rotation = 0;

        // Lose condition
        if !self.try_move(self.piece_offset, self.cur_rotation) {
            self.board.reset();
        }

        self.update_ghost_position();
    }

    fn update_ghost_position(&mut self) {
        let mut new_offset = self.piece_offset;

        while self.try_move((new_offset.0 - 1, new_offset.1), self.cur_rotation) {
            new_offset.0 -= 1;
        }

        self.ghost_offset = new_offset;
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
        let mut n_lines: u8 = 0;

        for i in 0..Game::HEIGHT {
            while Self::is_line_full(&new_board.0[i]) {
                new_board.0[i] = [BoardColor::Empty; Game::WIDTH];
                new_board.0.copy_within((i + 1).., i);
                n_lines += 1;
            }
        }

        // Send score event
        match n_lines {
            0 => self.score.do_event(ScoreEvent::EndCombo),
            1 => self.score.do_event(ScoreEvent::Single),
            2 => self.score.do_event(ScoreEvent::Double),
            3 => self.score.do_event(ScoreEvent::Triple),
            4 => self.score.do_event(ScoreEvent::Tetris),
            _ => {}
        }

        self.score.do_event(ScoreEvent::EndTurn);

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

        self.reset_piece(true);

        self.can_hold = true;
    }
}

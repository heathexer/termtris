use self::ScoreEvent::*;
use microkv::MicroKV;
use tui::style::Color;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum ScoreEvent {
    LineClear(Lines),
    TSpin(TSpins),
    SoftDrop(isize),
    HardDrop(isize),
    EndTurn,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Lines {
    None,
    Single,
    Double,
    Triple,
    Tetris,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum TSpins {
    None,
    MiniTSpin,
    TSpin,
}

pub struct Score {
    pub score: u32,
    high_score: u32,
    high_score_db: MicroKV,
    lines: u32,
    level: u8,
    // speed: Duration,
    turn: (TSpins, Lines),
    turn_score: u32,
    last_turn: (TSpins, Lines),
    last_turn_score: u32,
    last_turn_text: String,
    text_color: Color,
}

impl<'a> Score {
    pub fn new() -> Self {
        let high_score_db = MicroKV::open("score.data")
            .expect("Failed to create MicroKV")
            .set_auto_commit(true);
        let high_score = match high_score_db.get("score") {
            Ok(Some(score)) => score,
            _ => 0,
        };
        Score {
            score: 0,
            high_score,
            high_score_db,
            lines: 0,
            level: 1,
            turn_score: 0,
            last_turn_score: 0,
            last_turn_text: String::new(),
            turn: (TSpins::None, Lines::None),
            last_turn: (TSpins::None, Lines::None),
            text_color: Color::Gray,
        }
    }

    pub fn last_turn_score(&self) -> u32 {
        self.last_turn_score
    }

    pub fn last_turn_text(&self) -> &str {
        &self.last_turn_text
    }

    pub fn text_color(&self) -> Color {
        self.text_color
    }

    pub fn high_score(&self) -> u32 {
        self.high_score
    }

    pub fn lines(&self) -> u32 {
        self.lines
    }

    pub fn save_and_reset_score(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
            self.high_score_db.put("score", &self.high_score).unwrap();
        }

        self.score = 0;
    }

    pub fn do_event(&mut self, event: ScoreEvent) {
        match event {
            LineClear(lines) => self.turn.1 = lines,
            TSpin(spin) => self.turn.0 = spin,
            SoftDrop(len) => self.turn_score += len as u32,
            HardDrop(len) => self.turn_score += 2 * len as u32,
            EndTurn => {
                // Do score calculations
                self.last_turn_text = String::new();

                match self.turn {
                    (TSpins::None, Lines::Single) => {
                        // Single
                        self.turn_score += 100 * self.level as u32;
                        self.last_turn_text = "Single".to_string();
                    }
                    (TSpins::MiniTSpin, Lines::None) => {
                        // Mini T-Spin
                        self.turn_score += 100 * self.level as u32;
                        self.last_turn_text = "Mini T-Spin".to_string();
                    }
                    (TSpins::MiniTSpin, Lines::Single) => {
                        // Mini T-Spin Single
                        self.turn_score += 200 * self.level as u32;
                        self.last_turn_text = "Mini T-Spin Single".to_string();
                    }
                    (TSpins::None, Lines::Double) => {
                        // Double
                        self.turn_score += 300 * self.level as u32;
                        self.last_turn_text = "Double".to_string();
                    }
                    (TSpins::MiniTSpin, Lines::Double) => {
                        // Mini T-Spin Double
                        self.turn_score += 400 * self.level as u32;
                        self.last_turn_text = "Mini T-Spin Double".to_string();
                    }
                    (TSpins::TSpin, Lines::None) => {
                        // T-Spin
                        self.turn_score += 400 * self.level as u32;
                        self.last_turn_text = "T-Spin".to_string();
                    }
                    (TSpins::None, Lines::Triple) | (TSpins::MiniTSpin, Lines::Triple) => {
                        // Triple
                        self.turn_score += 500 * self.level as u32;
                        self.last_turn_text = "Triple".to_string();
                    }
                    (TSpins::None, Lines::Tetris)
                    | (TSpins::MiniTSpin, Lines::Tetris)
                    | (TSpins::TSpin, Lines::Tetris) => {
                        // Tetris
                        self.turn_score += 800 * self.level as u32;
                        self.last_turn_text = "Tetris".to_string();
                    }
                    (TSpins::TSpin, Lines::Single) => {
                        // T-Spin Single
                        self.turn_score += 800 * self.level as u32;
                        self.last_turn_text = "T-Spin Single".to_string();
                    }
                    (TSpins::TSpin, Lines::Double) => {
                        // T-Spin Double
                        self.turn_score += 1200 * self.level as u32;
                        self.last_turn_text = "T-Spin Double".to_string();
                    }
                    (TSpins::TSpin, Lines::Triple) => {
                        // T-Spin Triple
                        self.turn_score += 1600 * self.level as u32;
                        self.last_turn_text = "T-Spin Triple".to_string();
                    }
                    (TSpins::None, Lines::None) => {}
                }

                // Combo points
                if self.last_turn != (TSpins::None, Lines::None)
                    && self.turn != (TSpins::None, Lines::None)
                {
                    self.turn_score += 50 * self.level as u32;
                    self.text_color = Color::LightRed;
                } else {
                    self.text_color = Color::Gray;
                }

                // Count lines
                match self.turn.1 {
                    Lines::Single => self.lines += 1,
                    Lines::Double => self.lines += 3,
                    Lines::Triple => self.lines += 5,
                    Lines::Tetris => self.lines += 8,
                    Lines::None => {}
                }

                self.last_turn = self.turn;
                self.turn = (TSpins::None, Lines::None);

                self.score += self.turn_score;
                self.last_turn_score = self.turn_score;
                self.turn_score = 0;
            }
        }
    }
}

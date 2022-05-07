use self::ScoreEvent::*;
use super::level::Level;
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
    pub level: Level,
    high_score: u32,
    high_score_db: MicroKV,
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
            level: Level::new(),
            high_score,
            high_score_db,
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

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn high_score(&self) -> u32 {
        self.high_score
    }

    pub fn lines(&self) -> u32 {
        self.level.lines
    }

    pub fn lines_goal(&self) -> u32 {
        self.level.lines_goal
    }

    pub fn level(&self) -> u8 {
        self.level.level
    }

    pub fn add_lines(&mut self, lines: u32) {
        self.level.add_lines(lines)
    }

    pub fn save_and_reset_score(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
            self.high_score_db.put("score", &self.high_score).unwrap();
        }

        self.score = 0;
        self.level = Level::new();
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

                // Combo points
                if self.last_turn != (TSpins::None, Lines::None)
                    && self.turn != (TSpins::None, Lines::None)
                {
                    self.turn_score += 50 * self.level() as u32;
                    self.text_color = Color::Cyan;
                } else {
                    self.text_color = Color::Gray;
                }

                // Score data taken from here: https://tetris.fandom.com/wiki/Scoring#Guideline_scoring_system
                match self.turn {
                    (TSpins::None, Lines::Single) => {
                        // Single
                        self.turn_score += 100 * self.level() as u32;
                        self.last_turn_text = "Single".to_string();
                    }
                    (TSpins::MiniTSpin, Lines::None) => {
                        // Mini T-Spin
                        self.turn_score += 100 * self.level() as u32;
                        self.last_turn_text = "Mini T-Spin".to_string();
                    }
                    (TSpins::MiniTSpin, Lines::Single) => {
                        // Mini T-Spin Single
                        self.turn_score += 200 * self.level() as u32;
                        self.last_turn_text = "Mini T-Spin Single".to_string();
                    }
                    (TSpins::None, Lines::Double) => {
                        // Double
                        self.turn_score += 300 * self.level() as u32;
                        self.last_turn_text = "Double".to_string();
                    }
                    (TSpins::MiniTSpin, Lines::Double) => {
                        // Mini T-Spin Double
                        if self.last_turn == (TSpins::MiniTSpin, Lines::Double) {
                            // Back to back
                            self.turn_score += 600 * self.level() as u32;
                            self.last_turn_text = "B2B Mini T-Spin Double".to_owned();
                            self.text_color = Color::LightMagenta;
                        } else {
                            // Once
                            self.turn_score += 400 * self.level() as u32;
                            self.last_turn_text = "Mini T-Spin Double".to_string();
                        }
                    }
                    (TSpins::TSpin, Lines::None) => {
                        // T-Spin
                        self.turn_score += 400 * self.level() as u32;
                        self.last_turn_text = "T-Spin".to_string();
                    }
                    (TSpins::None, Lines::Triple) | (TSpins::MiniTSpin, Lines::Triple) => {
                        // Triple
                        self.turn_score += 500 * self.level() as u32;
                        self.last_turn_text = "Triple".to_string();
                    }
                    (_, Lines::Tetris) => {
                        // Tetris
                        if self.last_turn.1 == Lines::Tetris {
                            self.turn_score += 1200 * self.level() as u32;
                            self.last_turn_text = "B2B Tetris".to_string();
                            self.text_color = Color::LightMagenta;
                        } else {
                            self.turn_score += 800 * self.level() as u32;
                            self.last_turn_text = "Tetris".to_string();
                        }
                    }
                    (TSpins::TSpin, Lines::Single) => {
                        // T-Spin Single
                        if self.last_turn == (TSpins::TSpin, Lines::Single) {
                            self.turn_score += 1200 * self.level() as u32;
                            self.last_turn_text = "B2B T-Spin Single".to_string();
                            self.text_color = Color::LightMagenta;
                        } else {
                            self.turn_score += 800 * self.level() as u32;
                            self.last_turn_text = "T-Spin Single".to_string();
                        }
                    }
                    (TSpins::TSpin, Lines::Double) => {
                        // T-Spin Double
                        if self.last_turn == (TSpins::TSpin, Lines::Double) {
                            self.turn_score += 1800 * self.level() as u32;
                            self.last_turn_text = "B2B T-Spin Double".to_string();
                            self.text_color = Color::LightMagenta;
                        } else {
                            self.turn_score += 1200 * self.level() as u32;
                            self.last_turn_text = "T-Spin Double".to_string();
                        }
                    }
                    (TSpins::TSpin, Lines::Triple) => {
                        // T-Spin Triple
                        if self.last_turn == (TSpins::TSpin, Lines::Triple) {
                            self.turn_score += 2400 * self.level() as u32;
                            self.last_turn_text = "B2B T-Spin Triple".to_string();
                            self.text_color = Color::LightMagenta;
                        } else {
                            self.turn_score += 1600 * self.level() as u32;
                            self.last_turn_text = "T-Spin Triple".to_string();
                        }
                    }
                    (TSpins::None, Lines::None) => {
                        self.last_turn_text = "-".to_string();
                    }
                }

                // Count lines (Numbers from here https://tetris.fandom.com/wiki/Tetris_Guideline)
                match self.turn.1 {
                    Lines::Single => self.add_lines(1),
                    Lines::Double => self.add_lines(3),
                    Lines::Triple => self.add_lines(5),
                    Lines::Tetris => self.add_lines(8),
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

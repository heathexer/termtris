use self::ScoreEvent::*;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum ScoreEvent {
    EndTurn,
    EndCombo,
    Single,
    MiniTSpin,
    TSpin,
    Double,
    Triple,
    Tetris,
    SoftDrop(u8),
    HardDrop(u8),
}

pub struct Score {
    pub score: u32,
    lines: u32,
    level: u8,
    // speed: Duration,
    turn_stack: Vec<ScoreEvent>,
    last_turn_stack: Vec<ScoreEvent>,
    last_turn_score: u32,
}

impl<'a> Score {
    pub fn new() -> Self {
        Score {
            score: 0,
            lines: 0,
            level: 1,
            turn_stack: Vec::new(),
            last_turn_stack: Vec::new(),
            last_turn_score: 0,
        }
    }

    pub fn do_event(&mut self, event: ScoreEvent) {
        match event {
            ScoreEvent::EndTurn => {
                // Do score calculations
                let mut turn_score = 0_u32;

                if self.turn_stack.contains(&Single) {
                    turn_score += 100 * self.level as u32;
                }

                if self.turn_stack.contains(&MiniTSpin) {
                    turn_score += 100 * self.level as u32;
                }

                if self.turn_stack.contains(&Double) {
                    turn_score += 300 * self.level as u32;
                }

                if self.turn_stack.contains(&Triple) {
                    turn_score += 500 * self.level as u32;
                }

                if self.turn_stack.contains(&Tetris) {
                    turn_score += 800 * self.level as u32;
                }

                // Combo points
                if !self.last_turn_stack.contains(&EndCombo) && !self.turn_stack.contains(&EndCombo)
                {
                    turn_score += 50 * self.level as u32;
                }

                // Drop points
                self.turn_stack.iter().for_each(|event| match event {
                    SoftDrop(len) => turn_score += *len as u32,
                    HardDrop(len) => turn_score += 2 * *len as u32,
                    _ => {}
                });

                self.score += turn_score;
                self.last_turn_score = turn_score;

                self.last_turn_stack = self.turn_stack.clone();
                self.turn_stack = Vec::new();
            }
            event => self.turn_stack.push(event),
        }
    }
}

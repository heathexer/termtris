use std::time::Duration;

// Struct to store the player level and info associated with it, including constant tick delay data
pub struct Level {
    pub level: u8,
    pub lines: u32,
    pub lines_goal: u32,
}

// I can't think of a reason to have a Default impl for any structs in this project but I'm open to any reasons
#[allow(clippy::new_without_default)]
impl Level {
    // Speed information from here: https://tetris.fandom.com/wiki/Tetris_Worlds (rounded slightly)
    // Allowing zero prefixed literals for better alignment
    #[allow(clippy::zero_prefixed_literal)]
    const SPEEDS: [Duration; 16] = [
        Duration::from_millis(u64::MAX),
        Duration::from_millis(1000),
        Duration::from_millis(0793),
        Duration::from_millis(0618),
        Duration::from_millis(0473),
        Duration::from_millis(0355),
        Duration::from_millis(0262),
        Duration::from_millis(0190),
        Duration::from_millis(0135),
        Duration::from_millis(0094),
        Duration::from_millis(0064),
        Duration::from_millis(0043),
        Duration::from_millis(0028),
        Duration::from_millis(0018),
        Duration::from_millis(0012),
        Duration::from_millis(0007),
    ];

    pub fn new() -> Self {
        let level = 1;
        let lines = 0;
        let lines_goal = level as u32 * 5;
        Level {
            level,
            lines,
            lines_goal,
        }
    }

    pub fn get_tick_delay(&self) -> Duration {
        Self::SPEEDS[self.level as usize]
    }

    pub fn add_lines(&mut self, lines: u32) {
        self.lines += lines;
        if self.lines >= self.lines_goal && self.level < 15 {
            self.level += 1;
            self.lines_goal += self.level as u32 * 5;
        }
    }
}

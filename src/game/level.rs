use std::time::Duration;

pub struct Level {
    pub level: u8,
    pub lines: u32,
    pub lines_goal: u32,
}

impl Level {
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

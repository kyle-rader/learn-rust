#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub enum GameState {
    FrameStart,
    MidFrame,
    GameComplete,
}

pub struct BowlingGame {
    frame_start: bool,
    bonus_factor: (u16, u16),
    bonus_rolls: u8,
    pins_standing: u16,
    frames: usize,
    score: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frame_start: true,
            bonus_factor: (1, 1),
            bonus_rolls: 0,
            pins_standing: 10,
            frames: 0,
            score: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let game_complete = self.frames == 10 && self.bonus_rolls == 0;
        if game_complete {
            return Err(Error::GameComplete);
        }

        if self.frame_start {
            self.pins_standing = 10;
        }

        if pins > self.pins_standing {
            return Err(Error::NotEnoughPinsLeft);
        }

        // Compute running score
        self.score += pins * self.bonus_factor.0;
        self.pins_standing -= pins;

        // handle bonus modifier sliding window for next 2 rolls.
        let strike = self.frame_start && pins == 10;
        let strike_bonus = if strike && self.frames < 9 { 1 } else { 0 }; // no strike bonus on last frame.
        self.bonus_factor = (self.bonus_factor.1 + strike_bonus, 1 + strike_bonus);

        // Are we in bonus rolls?
        if self.bonus_rolls > 0 {
            self.bonus_rolls -= 1;
            self.frame_start = self.pins_standing == 0;
        }
        // are we starting af frame?
        else if self.frame_start {
            self.frame_start = strike;
            if strike {
                self.frames += 1;
                if self.frames == 10 {
                    self.bonus_rolls = 2;
                }
            }
        } else {
            // We're finishing the frame
            self.frames += 1;
            self.frame_start = true;
            if self.pins_standing == 0 {
                // spare!
                if self.frames == 10 {
                    // 1 extra roll for last frame.
                    self.bonus_rolls = 1;
                } else {
                    // Add 1 bonus count to next roll
                    self.bonus_factor.0 += 1;
                }
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if dbg!(self.frames) == 10 && self.bonus_rolls == 0 {
            Some(self.score)
        } else {
            None
        }
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

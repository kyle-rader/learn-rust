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
    was_spare: bool,
    strike_bonus: (u16, u16),
    bonus_rolls: u8,
    pins_standing: u16,
    frames: usize,
    score: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frame_start: true,
            was_spare: false,
            strike_bonus: (1, 1),
            bonus_rolls: 0,
            pins_standing: 10,
            frames: 0,
            score: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let game_complete = dbg!(self.frames) == 10 && dbg!(self.bonus_rolls) == 0;
        if dbg!(game_complete) {
            return Err(Error::GameComplete);
        }

        if self.frame_start {
            self.pins_standing = 10;
        }

        if pins <= self.pins_standing {
            let strike = self.frame_start && pins == 10;
            dbg!(strike);
            // always handle strike bonus modifier
            self.score += pins * self.strike_bonus.0;
            self.pins_standing -= pins;

            let strike_addon = if strike && self.frames < 9 { 1 } else { 0 };
            self.strike_bonus = (self.strike_bonus.1 + strike_addon, 1 + strike_addon);

            if self.bonus_rolls > 0 {
                self.bonus_rolls -= 1;
                if self.pins_standing != 0 {
                    self.frame_start = false;
                }
            } else {
                match self.frame_start {
                    true => {
                        self.frame_start = strike;
                        if strike {
                            self.frames += 1;
                            if self.frames == 10 {
                                self.bonus_rolls = 2;
                            }
                        }

                        if self.was_spare && self.frames != 10 {
                            // bonus points from spare, unless last frame extra roll
                            self.score += pins;
                            self.was_spare = false;
                        }
                    }
                    false => {
                        self.frames += 1;
                        self.frame_start = true;
                        if self.pins_standing == 0 {
                            // spare!
                            if self.frames == 10 {
                                self.bonus_rolls = 1;
                            } else {
                                self.was_spare = true;
                            }
                        }
                    }
                }
            }

            Ok(())
        } else {
            Err(Error::NotEnoughPinsLeft)
        }
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

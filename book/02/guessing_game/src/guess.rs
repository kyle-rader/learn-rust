pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            Err(format!("Guess must be between 1 and 100, got {}.", value))
        } else {
            Ok(Guess { value })
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

// If program crashes durin execution - it panics
// Panics do not return false
// We write test code that checks if it panicked
#[cfg(test)]
mod tests {
    use super::*;

    // Test passes since it panicks
    #[test]
    #[should_panic] // Panic annotation
    fn greater_than_100() {
        Guess::new(200);
    }
}

// test tests::greater_than_100 - should panic ... ok
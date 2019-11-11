use super::*;

/// Trait for rounding values to a number of decimal places
pub trait Round {
    /// Rounds the value to a number of decimal places
    fn round_to(self, places: i32) -> Self;
}

// Round an f32 to a number of decimal places
fn round_to(val: f32, places: i32) -> f32 {
    let mult = 10_f32.powi(places);
    (val * mult).round() / mult
}

impl Round for DeltaE {
    fn round_to(mut self, places: i32) -> Self {
        self.value = round_to(self.value, places);
        self
    }
}

impl Round for LabValue {
    fn round_to(mut self, places: i32) -> LabValue {
        self.l = round_to(self.l, places);
        self.a = round_to(self.a, places);
        self.b = round_to(self.b, places);
        self
    }
}

impl Round for LchValue {
    fn round_to(mut self, places: i32) -> LchValue {
        self.l = round_to(self.l, places);
        self.c = round_to(self.c, places);
        self.h = round_to(self.h, places);
        self
    }
}

impl Round for XyzValue {
    fn round_to(mut self, places: i32) -> XyzValue {
        self.x = round_to(self.x, places);
        self.y = round_to(self.y, places);
        self.z = round_to(self.z, places);
        self
    }
}

#[test]
fn round() {
    let val = 1.234567890;
    let rnd = round::round_to(val, 4);
    assert_eq!(rnd, 1.2346);
    assert_ne!(rnd, val);
}


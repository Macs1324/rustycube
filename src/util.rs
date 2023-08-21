pub mod numbers {
    use std::ops::{Add, Sub};

    pub fn move_toward(source: f32, target: f32, delta: f32) -> f32 {
        let diff = target - source;
        if diff.abs() < delta {
            target
        } else if diff > 0.0 {
            source + delta
        } else {
            source - delta
        }
    }
}

pub mod numbers {
    use std::ops::{Add, Sub};

    pub fn move_toward(source: f32, target: f32, delta: f32) -> f32 {
        if source > target {
            if (source - delta) < target {
                target
            } else {
                source - delta
            }
        } else if source < target {
            if (source + delta) > target {
                target
            } else {
                source + delta
            }
        } else {
            target
        }
    }
}

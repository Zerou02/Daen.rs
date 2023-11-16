use std::time::{SystemTime, UNIX_EPOCH};

pub fn getTime() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("asdad")
        .as_millis();
}

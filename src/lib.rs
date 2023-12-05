pub mod now {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn millis() -> u128 {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!");
        return duration.as_millis();
    }
}

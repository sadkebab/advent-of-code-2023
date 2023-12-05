pub mod now {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn millis() -> u128 {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!");
        return duration.as_millis();
    }
}

pub mod util {
    use std::collections::BTreeMap;

    pub fn print_map(map: BTreeMap<usize, usize>) {
        for (k, value) in map.into_iter() {
            println!("{}: {}", k + 1, value);
        }
    }
}

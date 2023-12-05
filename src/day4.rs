use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: Vec<String>) -> String {
    let mut points_vec = vec![];
    let mut scratchcard_count = input.len();
    let mut points_map: HashMap<usize, usize> = HashMap::new();

    for (index, line) in input.iter().enumerate() {
        let data: Vec<&str> = line.split(":").collect();
        let numbers: Vec<&str> = data[1].split("|").collect();
        let winning_numbers_string = numbers[0].trim();
        let card_numbers_string = numbers[1].trim();
        let mut points = 0usize;
        let mut matches = 0usize;

        let winning_numbers: Vec<&str> = winning_numbers_string.split_whitespace().collect();

        for winning_number in winning_numbers {
            let pattern = format!(r"\b({})\b", winning_number);
            let regex = Regex::new(&pattern).unwrap();

            if regex.is_match(card_numbers_string) {
                points = if points == 0 { 1 } else { points * 2 };
                matches += 1;
            }
        }
        points_vec.push(points);

        if matches > 0 {
            for offset in 1..=matches {
                let key = index + offset;
                let repetitions = match points_map.get(&index) {
                    Some(&n) => n + 1,
                    None => 1usize,
                };

                if let Some(value) = points_map.get(&key) {
                    points_map.insert(key, value + repetitions);
                } else {
                    points_map.insert(key, repetitions);
                }
            }
        }
    }

    let points = points_vec.iter().fold(0, |acc, &x| acc + x);

    for (_, value) in points_map.into_iter() {
        scratchcard_count += value;
    }

    format!("{}, {}", points, scratchcard_count)
}

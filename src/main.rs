use rand::Rng;
use std::{collections::BTreeMap, env, fs};

mod day1;
mod day2;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let mut map: BTreeMap<String, fn(input: Vec<String>) -> String> = BTreeMap::new();
    map.insert("1".to_string(), day1::solve);
    map.insert("2".to_string(), day2::solve);
    println!("## Advent Of Code 2023 by @sadkebab 🎅🏻 ##");

    if args.len() > 1 {
        let day = args[1].clone();
        if let Some(resolver) = map.get(&day) {
            let input = get_input(day.clone());
            let solution = resolver(input);
            let emoji = random_emoji();
            println!("{} Day {} Solution: {}", emoji, day, solution);
        } else {
            println!("🥶 Day argument not valid");
            return;
        }
    } else {
        for (day, resolver) in map.into_iter() {
            let input = get_input(day.clone());
            let solution = resolver(input);
            let emoji = random_emoji();
            println!("{} Day {} Solution: {}", emoji, day, solution);
        }
    }
}

fn get_input(day: String) -> Vec<String> {
    let file_path = format!("./inputs/day-{}.txt", day);
    return match fs::read_to_string(&file_path) {
        Ok(content) => content.split_terminator("\n").map(String::from).collect(),
        Err(_) => {
            panic!("Failed to read {}", file_path);
        }
    };
}

fn random_emoji() -> String {
    let emojis = vec!["🎁", "🎄", "⛄", "🍪", "🥛"];
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..=emojis.len() - 1);
    return emojis[random].to_string();
}

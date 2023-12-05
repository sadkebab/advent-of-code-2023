use advent_of_code_2023::now;
use rand::Rng;
use std::{collections::BTreeMap, env, fs};

mod day1;
mod day2;
mod day3;
mod day4;

pub fn main() {
    let start = now::millis();

    let args: Vec<String> = env::args().collect();
    let mut day_resolver_map: BTreeMap<&str, fn(input: Vec<String>) -> String> = BTreeMap::new();
    day_resolver_map.insert("1", day1::solve);
    day_resolver_map.insert("2", day2::solve);
    day_resolver_map.insert("3", day3::solve);
    day_resolver_map.insert("4", day4::solve);

    println!("## Advent Of Code 2023 by @sadkebab 🎅🏻 ##");
    println!();

    if args.len() > 1 {
        let day = args[1].as_str();
        if let Some(resolver) = day_resolver_map.get(day) {
            let input = get_input(day);
            let solution = resolver(input);
            let emoji = random_emoji();
            println!("{} Day {} Solution: {}", emoji, day, solution);
        } else {
            println!("🥶 Day argument not valid");
            return;
        }
    } else {
        for (day, resolver) in day_resolver_map.into_iter() {
            let input = get_input(day);
            let solution = resolver(input);
            let emoji = random_emoji();
            println!("{} Day {} Solution: {}", emoji, day, solution);
        }
    }

    let end = now::millis();
    println!();
    println!("Executed in {}ms", end - start);
}

fn get_input(day: &str) -> Vec<String> {
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

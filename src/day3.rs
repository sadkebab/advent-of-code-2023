use std::collections::HashMap;

use regex::Regex;

pub fn solve(input: Vec<String>) -> String {
    let number_pattern = r"\d+";
    let number_regex = Regex::new(number_pattern).unwrap();

    let ascii_symbols_pattern = r"[-+*/=<>!@#$%^&*()_+{}|\[\]:;<>,?~`]"; // all ascii symbols excluding .
    let symbols_regex = Regex::new(ascii_symbols_pattern).unwrap();

    let ratio_symbol_pattern = r"\*";
    let ratio_regex = Regex::new(ratio_symbol_pattern).unwrap();

    let mut part_numbers = vec![];
    let mut ratio_vector_map: HashMap<(usize, usize), Vec<String>> = HashMap::new();

    for (row, line) in input.iter().enumerate() {
        for captures in number_regex.captures_iter(line) {
            let capture = captures.get(0).unwrap();
            let number = capture.as_str();
            let start_col = capture.start();
            let end_col = capture.end();

            let mut surrounding = vec![];
            let surr_start = match start_col.checked_sub(1) {
                Some(n) => n,
                None => 0,
            };

            let end_plus_one = end_col;
            let max_end = line.len() - 1;

            let surr_end = if end_plus_one > max_end {
                max_end
            } else {
                end_plus_one
            };

            if let Some(previous) = row.checked_sub(1) {
                let previous_line = &input[previous];
                surrounding.push(&previous_line[surr_start..=surr_end])
            }

            surrounding.push(&line[surr_start..=surr_end]);

            if row + 1 < input.len() {
                let next_line = &input[row + 1];
                surrounding.push(&next_line[surr_start..=surr_end]);
            }

            let surrounding_string = surrounding.join("");
            if symbols_regex.is_match(&surrounding_string) {
                part_numbers.push(number.parse::<u32>().unwrap());
                for (index, surrounding_line) in surrounding.iter().enumerate() {
                    for captures in ratio_regex.captures_iter(&surrounding_line) {
                        let capture = captures.get(0).unwrap();
                        let ratio_start = capture.start();

                        let ratio_row_offset = if row > 0 { 1 } else { 0 };
                        let ratio_row = row + index - ratio_row_offset;
                        let ratio_col = start_col + ratio_start;

                        let ratio_key = (ratio_row, ratio_col);
                        if let Some(ratio_vector) = ratio_vector_map.get_mut(&ratio_key) {
                            ratio_vector.push(number.to_string());
                        } else {
                            ratio_vector_map.insert(ratio_key, vec![number.to_string()]);
                        }
                    }
                }
            }
        }
    }

    let mut ratios = vec![];

    for (_, vector) in ratio_vector_map.into_iter() {
        if vector.len() == 2 {
            let ratio = vector
                .iter()
                .map(|x| x.parse::<u64>().unwrap())
                .fold(1, |acc, x| acc * x);
            ratios.push(ratio)
        }
    }

    let part_numbers_sum = part_numbers.iter().fold(0, |acc, &x| acc + x);
    let ratios_sum = ratios.iter().fold(0, |acc, &x| acc + x);
    format!("{}, {}", part_numbers_sum, ratios_sum)
}

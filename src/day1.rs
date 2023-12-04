pub fn solve(input: Vec<String>) -> String {
    let mut wrong_nums = vec![];
    let mut correct_nums = vec![];

    for line in input {
        wrong_nums.push(parse(line.clone()));
        let correct_line = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("nine", "n9e")
            .replace("eight", "e8t");
        correct_nums.push(parse(correct_line));
    }

    let wrong_sum = wrong_nums.iter().fold(0, |acc, &x| acc + x);
    let correct_sum = correct_nums.iter().fold(0, |acc, &x| acc + x);
    return format!("{}, {}", wrong_sum, correct_sum);
}

fn parse(line: String) -> u32 {
    let chars = line.trim().chars().collect::<Vec<_>>();

    let mut start_pointer = 0;
    let mut end_pointer = chars.len() - 1;

    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    while first_digit == None {
        first_digit = chars[start_pointer].to_digit(10);
        if first_digit == None {
            start_pointer += 1
        }
    }

    while last_digit == None {
        last_digit = chars[end_pointer].to_digit(10);
        if last_digit == None {
            end_pointer -= 1
        }
        if end_pointer < start_pointer {
            return 0;
        }
    }

    let value = first_digit.unwrap() * 10 + last_digit.unwrap();
    return value;
}

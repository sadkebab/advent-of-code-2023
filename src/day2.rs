use std::collections::HashMap;

pub fn solve(input: Vec<String>) -> String {
    let mut ids = vec![];
    let mut cubes = vec![];
    let max_red = 12u32;
    let max_green = 13u32;
    let max_blue = 14u32;

    for line in input {
        let (id, red, green, blue) = parse(line);
        if red <= max_red && green <= max_green && blue <= max_blue {
            ids.push(id);
        }
        cubes.push(red * green * blue);
    }

    let sum = ids.iter().fold(0, |acc, &x| acc + x);
    let power_sum = cubes.iter().fold(0, |acc, &x| acc + x);

    format!("{}, {}", sum, power_sum)
}

fn parse(line: String) -> (u32, u32, u32, u32) {
    let split: Vec<&str> = line.split(":").collect();
    let id = split[0].replace("Game", "").trim().parse::<u32>().unwrap();
    let mut map: HashMap<&str, u32> = HashMap::new();

    map.insert("red", 0);
    map.insert("green", 0);
    map.insert("blue", 0);

    let normalized = split[1].trim().replace(";", ",");
    let sorties = normalized.split(", ");

    for sortie in sorties {
        let parts: Vec<&str> = sortie.split(" ").collect();
        let color = parts[1];
        let parsed = parts[0].parse::<u32>().unwrap();

        map.entry(color).and_modify(|value| {
            if *value < parsed {
                *value = parsed;
            }
        });
    }

    let retval = (
        id,
        map.get("red").cloned().unwrap(),
        map.get("green").cloned().unwrap(),
        map.get("blue").cloned().unwrap(),
    );

    return retval;
}

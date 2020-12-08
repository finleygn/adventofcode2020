use regex::Regex;

fn parse_line(line: &str) -> (usize, usize, &str, &str) {
    let cap = Regex::new(r"^([0-9]*)-([0-9]*)\s(.):\s(.*)$")
        .unwrap()
        .captures(line)
        .unwrap();

    let num1 = cap
        .get(1)
        .map_or("", |m| m.as_str())
        .parse::<usize>()
        .unwrap();

    let num2 = cap
        .get(2)
        .map_or("", |m| m.as_str())
        .parse::<usize>()
        .unwrap();

    let character = cap.get(3).map_or("", |m| m.as_str());
    let password = cap.get(4).map_or("", |m| m.as_str());

    return (num1, num2, character, password);
}

pub fn part1(data: Vec<String>) -> u32 {
    let mut total = 0;

    for item in &data {
        let (min, max, character, password) = parse_line(item);
        let count = password.matches(character).count();

        if count >= min && count <= max {
            total = total + 1;
        }
    }

    return total;
}

pub fn part2(data: Vec<String>) -> u32 {
    let mut total = 0;

    for item in &data {
        let (pos1, pos2, character, password) = parse_line(item);

        let char_1_correct = match password.chars().nth(pos1 - 1) {
            Some(c) => c.to_string() == character,
            None => false,
        };
        let char_2_correct = match password.chars().nth(pos2 - 1) {
            Some(c) => c.to_string() == character,
            None => false,
        };

        if char_1_correct != char_2_correct {
            total = total + 1;
        }
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn d2part1test() {
        let data = util::lines_from_file("./data/day2.txt");
        assert_eq!(part1(data), 580);
    }

    #[test]
    fn d2part2test() {
        let data = util::lines_from_file("./data/day2.txt");
        assert_eq!(part2(data), 611);
    }
}

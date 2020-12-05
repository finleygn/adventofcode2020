#[allow(dead_code)]
mod util {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
        path::Path,
    };

    pub fn grid_from_file(filename: impl AsRef<Path>) -> Vec<Vec<char>> {
        let file = File::open(filename).expect("no such file");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| {
                l.expect("Could not parse line")
                    .chars()
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>()
    }

    pub fn lines_from_file_as_u32(filename: impl AsRef<Path>) -> Vec<u32> {
        let file = File::open(filename).expect("no such file");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| {
                l.expect("Could not parse line")
                    .parse::<u32>()
                    .expect("Could not convert to int")
            })
            .collect()
    }

    pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
        let file = File::open(filename).expect("no such file");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("Could not convert to int"))
            .collect()
    }
}

#[allow(dead_code)]
// Probably should've done this https://en.wikipedia.org/wiki/3SUM
mod day1 {
    pub fn part1(data: Vec<u32>) -> Result<u32, ()> {
        for i in 0..data.len() {
            for ii in 0..data.len() {
                let inum = data[i];
                let iinum = data[ii];
                if inum + iinum == 2020 {
                    return Ok(inum * iinum);
                }
            }
        }
        Err(())
    }

    // 👁👄👁
    pub fn part2(data: Vec<u32>) -> Result<u32, ()> {
        for i in 0..data.len() {
            for ii in 0..data.len() {
                for iii in 0..data.len() {
                    let inum = data[i];
                    let iinum = data[ii];
                    let iiinum = data[iii];

                    if inum + iinum + iiinum == 2020 {
                        return Ok(inum * iinum * iiinum);
                    }
                }
            }
        }
        Err(())
    }

    #[cfg(test)]
    mod tests {
        use super::{super::util, *};
        #[test]
        fn d1part1test() {
            let data = util::lines_from_file_as_u32("./data/day1.txt");
            assert_eq!(1007104, part1(data).unwrap())
        }

        #[test]
        fn d1part2test() {
            let data = util::lines_from_file_as_u32("./data/day1.txt");
            assert_eq!(18847752, part2(data).unwrap())
        }
    }
}

#[allow(dead_code)]
mod day2 {
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
        use super::{super::util, *};

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
}

#[allow(dead_code)]
mod day3 {
    pub struct Pos {
        x: usize,
        y: usize,
    }

    pub fn traverse(map: &Vec<Vec<char>>, inc: Pos) -> u32 {
        let mut total = 0;
        let mut pos = Pos { x: 0, y: 0 };
        while pos.y < map.len() {
            if map[pos.y][pos.x % map[pos.y].len()] == '#' {
                total += 1;
            }

            pos.x += inc.x;
            pos.y += inc.y;
        }

        total
    }

    pub fn part1(map: Vec<Vec<char>>) -> u32 {
        traverse(&map, Pos { x: 3, y: 1 })
    }

    pub fn part2(map: Vec<Vec<char>>) -> u32 {
        let rounds = vec![
            traverse(&map, Pos { x: 1, y: 1 }),
            traverse(&map, Pos { x: 3, y: 1 }),
            traverse(&map, Pos { x: 5, y: 1 }),
            traverse(&map, Pos { x: 7, y: 1 }),
            traverse(&map, Pos { x: 1, y: 2 }),
        ];

        rounds.into_iter().product()
    }

    #[cfg(test)]
    mod tests {
        use super::{super::util, *};

        #[test]
        fn d3part1test() {
            let data = util::grid_from_file("./data/day3.txt");
            assert_eq!(part1(data), 247);
        }

        #[test]
        fn d3part2test() {
            let data = util::grid_from_file("./data/day3.txt");
            assert_eq!(part2(data), 2983070376);
        }
    }
}

#[allow(dead_code)]
mod day4 {
    use regex::Regex;
    use std::collections::HashMap;

    type Passport = HashMap<String, String>;

    fn parseport(parseports: Vec<String>) -> Vec<Passport> {
        let mut passports: Vec<Passport> = vec![];
        let mut index = 0;

        for row in parseports {
            if row != "" {
                if passports.len() == 0 || passports.len() - 1 < index {
                    passports.push(HashMap::new());
                }

                let items: Vec<&str> = row.split(" ").collect();
                for item in items {
                    let data: Vec<&str> = item.split(":").collect();
                    passports[index].insert(String::from(data[0]), String::from(data[1]));
                }
            } else {
                index += 1
            }
        }

        passports
    }

    mod validator {
        pub fn is_number(item: &str) -> bool {
            match item.parse::<usize>() {
                Ok(_) => true,
                Err(_) => false,
            }
        }

        pub fn parse_unit(item: &str, unit: &str) -> Result<usize, ()> {
            if item.ends_with(unit) {
                return Ok(item.split(unit).collect::<Vec<&str>>()[0]
                    .parse::<usize>()
                    .unwrap());
            }
            Err(())
        }
    }

    pub fn part1(data: Vec<String>) -> u32 {
        let passports = parseport(data);
        let targets = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let mut total = 0;

        for passport in passports {
            let mut missing = false;

            for target in &targets {
                if let None = passport.get(&target.to_string()) {
                    missing = true;
                }
            }
            if !missing {
                total += 1;
            }
        }

        total
    }

    pub fn part2(data: Vec<String>) -> u32 {
        let passports = parseport(data);
        let targets = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let mut total = 0;

        for passport in passports {
            let mut valid = true;

            for target in &targets {
                match passport.get(&target.to_string()) {
                    None => {
                        valid = false;
                    }
                    Some(value) => match &target[..] {
                        "byr" => {
                            let num_value = value.parse::<usize>().unwrap();
                            if num_value < 1920 || num_value > 2002 {
                                valid = false;
                            }
                        }
                        "iyr" => {
                            let num_value = value.parse::<usize>().unwrap();
                            if num_value < 2010 || num_value > 2020 {
                                valid = false;
                            }
                        }
                        "eyr" => {
                            let num_value = value.parse::<usize>().unwrap();
                            if num_value < 2020 || num_value > 2030 {
                                valid = false;
                            }
                        }
                        "hgt" => {
                            let parsed_cm = validator::parse_unit(value, "cm");
                            let parsed_inc = validator::parse_unit(value, "in");

                            if let Ok(cm) = parsed_cm {
                                if cm < 150 || cm > 193 {
                                    valid = false;
                                }
                            } else if let Ok(inc) = parsed_inc {
                                if inc < 59 || inc > 76 {
                                    valid = false;
                                }
                            } else {
                                valid = false;
                            }
                        }
                        "hcl" => {
                            let cap = Regex::new(r"^#[0-9a-f]{6}$").unwrap();

                            if !cap.is_match(value) {
                                valid = false;
                            }
                        }
                        "ecl" => match &value[..] {
                            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
                            _ => {
                                valid = false;
                            }
                        },
                        "pid" => {
                            if !validator::is_number(value) || value.len() != 9 {
                                valid = false
                            }
                        }
                        _ => {
                            println!("unhandled {}", value);
                        }
                    },
                }
            }

            if valid {
                total += 1;
            }
        }
        total
    }

    #[cfg(test)]
    mod tests {
        use super::{super::util, *};

        #[test]
        fn d4part1test() {
            let data = util::lines_from_file("./data/day4.txt");
            assert_eq!(210, part1(data));
        }

        #[test]
        fn d4part2test() {
            let data = util::lines_from_file("./data/day4.txt");
            assert_eq!(131, part2(data));
        }
    }
}

#[allow(dead_code)]
// Probably should've done this https://en.wikipedia.org/wiki/3SUM
mod day5 {
    pub fn parse_instructions(characters: &[char], target: &char) -> Vec<bool> {
        characters.into_iter().map(|i| i == target).collect()
    }

    pub fn reduce(instructions: Vec<bool>, length: usize) -> usize {
        let mut span = length;
        let mut location = 0;

        for instruction in instructions {
            if instruction {
                span = span / 2;
                location = span + location;
            } else {
                span = span / 2;
            }
        }

        location
    }

    pub fn get_seat_ids(data: Vec<Vec<char>>) -> Vec<usize> {
        let mut seats = vec![];

        for pass in &data {
            let row = &pass.get(0..7).unwrap();
            let col = &pass.get(7..10).unwrap();

            let row_instructions = parse_instructions(&row, &'B'); // Position increases with B
            let col_instructions = parse_instructions(&col, &'R'); // Position increases with R

            let row_location = reduce(row_instructions, 128);
            let col_location = reduce(col_instructions, 8);

            seats.push(row_location * 8 + col_location);
        }

        seats
    }

    pub fn part1(data: Vec<Vec<char>>) -> usize {
        let seats = get_seat_ids(data);
        seats.into_iter().max().unwrap()
    }

    pub fn part2(data: Vec<Vec<char>>) -> Option<usize> {
        let seats = get_seat_ids(data);

        for seat in 0..seats.len() {
            if !seats.contains(&seat) && seats.contains(&(seat + 1)) && seats.contains(&(seat - 1))
            {
                return Some(seat);
            }
        }

        None
    }

    #[cfg(test)]
    mod tests {
        use super::{super::util, *};
        #[test]
        fn d5part1test() {
            let data = util::grid_from_file("./data/day5.txt");
            assert_eq!(816, part1(data));
        }

        #[test]
        fn d5part2test() {
            let data = util::grid_from_file("./data/day5.txt");
            assert_eq!(539, part2(data).unwrap());
        }
    }
}

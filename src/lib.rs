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

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum Field {
        BYR,
        IYR,
        EYR,
        HGT,
        HCL,
        ECL,
        PID,
        CID,
    }

    pub struct Passport {
        pub map: HashMap<Field, String>,
    }

    impl Field {
        pub fn from(string: &str) -> Result<Field, ()> {
            match string {
                "byr" => Ok(Field::BYR),
                "iyr" => Ok(Field::IYR),
                "eyr" => Ok(Field::EYR),
                "hgt" => Ok(Field::HGT),
                "hcl" => Ok(Field::HCL),
                "ecl" => Ok(Field::ECL),
                "pid" => Ok(Field::PID),
                "cid" => Ok(Field::CID),
                _ => Err(()),
            }
        }
    }

    impl Passport {
        pub fn new() -> Passport {
            Passport {
                map: HashMap::new(),
            }
        }

        pub fn parse(parseports: Vec<String>) -> Vec<Passport> {
            let mut passports: Vec<Passport> = vec![];
            let mut index = 0;

            for row in parseports {
                if row != "" {
                    if passports.len() == 0 || passports.len() - 1 < index {
                        passports.push(Passport::new());
                    }

                    let items: Vec<&str> = row.split(" ").collect();
                    for item in items {
                        let data: Vec<&str> = item.split(":").collect();
                        passports[index].add(data[0], data[1]);
                    }
                } else {
                    index += 1
                }
            }

            passports
        }

        pub fn add(&mut self, key: &str, value: &str) {
            self.map
                .insert(Field::from(key).unwrap(), String::from(value));
        }

        pub fn get(&self, key: &Field) -> Option<&String> {
            self.map.get(&key)
        }
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
        let passports = Passport::parse(data);
        let targets = vec![
            Field::BYR,
            Field::IYR,
            Field::EYR,
            Field::HGT,
            Field::HCL,
            Field::ECL,
            Field::PID,
        ];

        let mut total = 0;
        for passport in passports {
            let mut missing = false;

            for target in &targets {
                if let None = passport.get(target) {
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
        let passports = Passport::parse(data);
        let targets = vec![
            Field::BYR,
            Field::IYR,
            Field::EYR,
            Field::HGT,
            Field::HCL,
            Field::ECL,
            Field::PID,
        ];

        let mut total = 0;

        for passport in passports {
            let mut valid = true;

            for target in &targets {
                match passport.get(target) {
                    None => {
                        valid = false;
                    }
                    Some(value) => match target {
                        Field::BYR => {
                            let num_value = value.parse::<usize>().unwrap();
                            if num_value < 1920 || num_value > 2002 {
                                valid = false;
                            }
                        }
                        Field::IYR => {
                            let num_value = value.parse::<usize>().unwrap();
                            if num_value < 2010 || num_value > 2020 {
                                valid = false;
                            }
                        }
                        Field::EYR => {
                            let num_value = value.parse::<usize>().unwrap();
                            if num_value < 2020 || num_value > 2030 {
                                valid = false;
                            }
                        }
                        Field::HGT => {
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
                        Field::HCL => {
                            let cap = Regex::new(r"^#[0-9a-f]{6}$").unwrap();

                            if !cap.is_match(value) {
                                valid = false;
                            }
                        }
                        Field::ECL => match &value[..] {
                            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
                            _ => {
                                valid = false;
                            }
                        },
                        Field::PID => {
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
            println!("{}", part1(data));
        }

        #[test]
        fn d4part2test() {
            let data = util::lines_from_file("./data/day4.txt");
            println!("{}", part2(data));
        }
    }
}

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
mod day5 {
    pub fn parse_instructions(characters: &[char], target: &char) -> String {
        characters
            .into_iter()
            .map(|i| {
                if i == target {
                    String::from("1")
                } else {
                    String::from("0")
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn get_seat_ids(data: Vec<Vec<char>>) -> Vec<usize> {
        let mut seats = vec![];

        for pass in &data {
            let row = &pass.get(0..7).unwrap();
            let col = &pass.get(7..10).unwrap();

            let row_instructions = &parse_instructions(&row, &'B'); // Position increases with B
            let col_instructions = &parse_instructions(&col, &'R'); // Position increases with R

            let row_location = usize::from_str_radix(row_instructions, 2).unwrap();
            let col_location = usize::from_str_radix(col_instructions, 2).unwrap();

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

#[allow(dead_code)]
mod day6 {
    fn seperate_groups(data: Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
        let mut groups: Vec<Vec<Vec<char>>> = vec![vec![]];
        let mut index = 0;

        for row in data {
            if row.len() > 0 {
                groups[index].push(row);
            } else {
                groups.push(vec![]);
                index += 1
            }
        }

        groups
    }

    pub fn part1(data: Vec<Vec<char>>) -> usize {
        let groups = seperate_groups(data);
        let mut total = 0;

        for group in groups {
            let mut answered = vec![];

            for person in group {
                for answer in person {
                    if !answered.contains(&answer) {
                        answered.push(answer);
                    }
                }
            }

            total += answered.len();
        }

        total
    }

    pub fn part2(data: Vec<Vec<char>>) -> usize {
        let groups = seperate_groups(data);
        let mut total = 0;

        for group in groups {
            let mut answered: Vec<char> = vec![];

            // For each answer the first person in group gave...
            for answer in &group[0] {
                let mut in_all = true;

                for person in &group[1..] {
                    if !person.contains(&answer) {
                        in_all = false;
                    }
                }

                if in_all {
                    answered.push(answer.clone());
                }
            }

            total += answered.len();
        }

        total
    }

    #[cfg(test)]
    mod tests {
        use super::{super::util, *};

        #[test]
        fn d6part1test() {
            let data = util::grid_from_file("./data/day6.txt");
            assert_eq!(6799, part1(data));
        }

        #[test]
        fn d6part2test() {
            let data = util::grid_from_file("./data/day6.txt");
            assert_eq!(3354, part2(data));
        }
    }
}

#[allow(dead_code)]
mod day7 {
    use regex::Regex;
    use std::collections::HashMap;

    type Bag = String;

    #[derive(Debug)]
    struct Containable {
        bag: Bag,
        total: u32,
    }

    #[derive(Debug)]
    struct BagRules {
        pub bags: Vec<Bag>,
        pub containment_rules: HashMap<Bag, Vec<Containable>>,
    }

    impl BagRules {
        pub fn create_from_ruleset(rules: Vec<String>) -> BagRules {
            let line_regex: Regex =
                Regex::new(r"^(.*) bags contain ((\d.*? bags?)|no other bags).$").unwrap();
            let content_regex: Regex = Regex::new(r"^(\d) (.*) bags?$").unwrap();

            let mut bag_rules = BagRules {
                bags: Vec::new(),
                containment_rules: HashMap::new(),
            };

            for plain_rule in rules {
                let rule = line_regex.captures(&plain_rule).unwrap();

                let line_name = rule.get(1).map_or("", |m| m.as_str());
                let no_content = rule.get(2).map_or("", |m| m.as_str());
                let content = rule.get(3).map_or("", |m| m.as_str());

                let contained = {
                    if no_content == "no other bags" {
                        vec![]
                    } else {
                        let mut item_names: Vec<Containable> = vec![];

                        for containable in content.split(", ") {
                            let item = content_regex.captures(containable).unwrap();
                            let item_count = item.get(1).map_or("", |m| m.as_str());
                            let item_name = item.get(2).map_or("", |m| m.as_str());

                            item_names.push(Containable {
                                bag: String::from(item_name),
                                total: item_count.parse::<u32>().unwrap(),
                            });
                        }

                        item_names
                    }
                };

                bag_rules.bags.push(String::from(line_name));
                bag_rules
                    .containment_rules
                    .insert(String::from(line_name), contained);
            }

            bag_rules
        }

        pub fn bag_can_contain_bag(&self, bag: &Bag, target: &Bag) -> bool {
            for containable in self.containment_rules.get(bag).unwrap() {
                if &containable.bag == target {
                    return true;
                } else {
                    if self.bag_can_contain_bag(&containable.bag, &target) {
                        return true;
                    }
                }
            }

            false
        }

        pub fn sum_containment(&self, bag: &Bag) -> u32 {
            let mut total = 0;

            for containable in self.containment_rules.get(bag).unwrap() {
                println!("{:#?}", containable);
                total += containable.total;
                total += containable.total * self.sum_containment(&containable.bag);
            }

            total
        }
    }

    pub fn part1(rules: Vec<String>) -> u32 {
        let bag_rules = BagRules::create_from_ruleset(rules);
        let mut total = 0;

        for bag in &bag_rules.bags {
            if bag_rules.bag_can_contain_bag(bag, &String::from("shiny gold")) {
                total += 1;
            }
        }

        total
    }

    pub fn part2(rules: Vec<String>) -> u32 {
        let bag_rules = BagRules::create_from_ruleset(rules);
        bag_rules.sum_containment(&String::from("shiny gold"))
    }

    #[cfg(test)]
    mod tests {
        use super::{super::util, *};

        #[test]
        fn d7part1test() {
            let data = util::lines_from_file("./data/day7.txt");
            assert_eq!(119, part1(data));
        }

        #[test]
        fn d7part2test() {
            let data = util::lines_from_file("./data/day7.txt");
            assert_eq!(155802, part2(data));
        }
    }
}

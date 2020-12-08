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
    use super::*;
    use crate::util;

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

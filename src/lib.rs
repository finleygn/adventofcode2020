#[allow(dead_code)]
mod util {
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
        path::Path,
    };

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

use std::collections::HashSet;

pub fn twosum(data: &[isize], target: isize) -> bool {
    let mut seen = HashSet::new();

    for item in data {
        if seen.contains(&(target - item)) {
            return true;
        }
        seen.insert(item);
    }

    false
}

pub fn part1(data: &Vec<isize>) -> Option<isize> {
    for (index, value) in data[25..].iter().enumerate() {
        let available = &data[index..index + 25];

        if !twosum(&available, *value) {
            return Some(*value);
        }
    }

    None
}

pub fn part2(data: &Vec<isize>) -> Option<isize> {
    let target = part1(&data).unwrap();

    for (i, num1) in data.iter().enumerate() {
        let mut total = *num1;

        for (ii, num2) in data[i + 1..].iter().enumerate() {
            total += num2;

            if total > target {
                break;
            }

            if total == target {
                let addition_range = &data[i..i + ii + 2];
                return Some(
                    addition_range.iter().min().unwrap() + addition_range.iter().max().unwrap(),
                );
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn d9part1test() {
        let data = util::lines_from_file_as_isize("./data/day9.txt");
        assert_eq!(85848519, part1(&data).unwrap());
    }

    #[test]
    fn d9part2test() {
        let data = util::lines_from_file_as_isize("./data/day9.txt");
        assert_eq!(13414198, part2(&data).unwrap());
    }
}

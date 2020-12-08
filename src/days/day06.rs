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
    use super::*;
    use crate::util;

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

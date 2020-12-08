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
    use super::*;
    use crate::util;

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

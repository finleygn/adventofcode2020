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
        if !seats.contains(&seat) && seats.contains(&(seat + 1)) && seats.contains(&(seat - 1)) {
            return Some(seat);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

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

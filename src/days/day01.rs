// Probably should've done this https://en.wikipedia.org/wiki/3SUM

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
    use super::*;
    use crate::util;

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

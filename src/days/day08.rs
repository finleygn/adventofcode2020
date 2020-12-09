use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
enum Operation {
    JMP,
    ACC,
    NOP,
}

#[derive(Clone, Debug)]
struct Line {
    operation: Operation,
    value: isize,
}

#[derive(Clone, Debug)]
struct Program {
    lines: Vec<Line>,
    visited: Vec<isize>,
    location: isize,
    pub accumulator: isize,
}

impl Line {
    pub fn create_from_string(line: &String) -> Line {
        let line_regex = Regex::new(r"(\w{3}) ([+-])(\d+)").unwrap();
        let capture = line_regex.captures(line).unwrap();

        let operation = match capture.get(1).map_or("", |m| m.as_str()) {
            "jmp" => Operation::JMP,
            "acc" => Operation::ACC,
            _ => Operation::NOP,
        };
        let modifier = capture.get(2).map_or("", |m| m.as_str());
        let value = capture
            .get(3)
            .map_or("", |m| m.as_str())
            .parse::<isize>()
            .unwrap();

        Line {
            operation,
            value: if modifier == "+" { value } else { -value },
        }
    }
}

impl Program {
    pub fn new(data: Vec<String>) -> Program {
        let mut lines = vec![];

        for line in data {
            lines.push(Line::create_from_string(&line))
        }

        Program {
            lines,
            location: 0,
            accumulator: 0,
            visited: vec![],
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.location as usize >= self.lines.len() {
                break;
            }

            let line = &self.lines[self.location as usize];

            if self.visited.contains(&self.location) {
                break;
            } else {
                self.visited.push(self.location);
            }

            match line.operation {
                Operation::JMP => {
                    self.location += line.value;
                }
                Operation::ACC => {
                    self.accumulator += line.value;
                    self.location += 1;
                }
                _ => {
                    self.location += 1;
                }
            }
        }
    }
}

pub fn part1(data: Vec<String>) -> isize {
    let mut program = Program::new(data);
    program.run();

    program.accumulator
}

pub fn part2(data: Vec<String>) -> Option<isize> {
    let base_program = Program::new(data);

    for line in 0..base_program.lines.len() {
        let mut program = base_program.clone();

        if program.lines[line].operation == Operation::NOP {
            program.lines[line].operation = Operation::JMP;
        }

        if program.lines[line].operation == Operation::JMP {
            program.lines[line].operation = Operation::NOP;
        }

        program.run();

        if program.location >= program.lines.len() as isize {
            return Some(program.accumulator);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn d8part1test() {
        let data = util::lines_from_file("./data/day8.txt");
        assert_eq!(1928, part1(data));
    }
    #[test]
    fn d8part2test() {
        let data = util::lines_from_file("./data/day8.txt");
        assert_eq!(1319, part2(data).unwrap());
    }
}

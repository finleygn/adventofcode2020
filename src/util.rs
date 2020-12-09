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

pub fn lines_from_file_as_isize(filename: impl AsRef<Path>) -> Vec<isize> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| {
            l.expect("Could not parse line")
                .parse::<isize>()
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

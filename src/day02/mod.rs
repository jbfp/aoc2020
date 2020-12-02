use std::println;

use regex::Regex;

static FILE: &str = include_str!("../inputs/2.txt");

lazy_static! {
    static ref REGEX: Regex = Regex::new("^(\\d+)-(\\d+) (\\w): (\\w+)$").unwrap();
}

pub(crate) fn part1() {
    println!("day 2 part 1");

    let num_valid = FILE
        .lines()
        .filter(|s| {
            for cap in REGEX.captures_iter(s) {
                let min = cap[1].parse::<usize>().unwrap();
                let max = cap[2].parse::<usize>().unwrap();
                let test = cap[3].parse::<char>().unwrap();
                let input = cap[4].chars();
                let n = input.filter(|c| *c == test).count();

                if n >= min && n <= max {
                    return true;
                }
            }

            false
        })
        .count();

    println!("{} valid passwords", num_valid);
}

pub(crate) fn part2() {
    println!("day 2 part 2");

    let num_valid = FILE
        .lines()
        .filter(|s| {
            for cap in REGEX.captures_iter(s) {
                let first = cap[1].parse::<usize>().unwrap();
                let last = cap[2].parse::<usize>().unwrap();
                let test = cap[3].parse::<char>().unwrap() as u8;
                let input = cap[4].as_bytes();

                if (input[first - 1] == test) ^ (input[last - 1] == test) {
                    return true;
                }
            }

            false
        })
        .count();

    println!("{} valid passwords", num_valid);
}
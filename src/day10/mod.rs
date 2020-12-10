use itertools::Itertools;
use std::collections::HashMap;

static FILE: &str = include_str!("../inputs/10.txt");

lazy_static! {
    static ref LINES: Vec<i64> = FILE
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .sorted()
        .collect_vec();
}

pub(crate) fn part1() {
    println!("day 10 part 1");

    let mut ones = 0;
    let mut threes = 1;
    let mut current = 0;

    for &line in &*LINES {
        let diff = line - current;

        if diff == 1 {
            ones += 1
        } else if diff == 3 {
            threes += 1;
        }

        current = line;
    }

    println!("{} * {} = {}", ones, threes, ones * threes);
}

pub(crate) fn part2() {
    println!("day 10 part 2");

    let lines = &*LINES;
    let mut paths = HashMap::<i64, i64>::new();
    paths.insert(0, 1);

    for &line in lines {
        let sum = paths.get(&(line - 1)).unwrap_or(&0)
            + paths.get(&(line - 2)).unwrap_or(&0)
            + paths.get(&(line - 3)).unwrap_or(&0);

        paths.insert(line, sum);
    }

    let num_paths = paths[lines.last().unwrap()];
    println!("number of paths: {}", num_paths);
}

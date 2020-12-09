use itertools::{Itertools, MinMaxResult};
use std::cmp::Ordering;

static FILE: &str = include_str!("../inputs/9.txt");

lazy_static! {
    static ref LINES: Vec<usize> = FILE.lines().map(|s| s.parse().unwrap()).collect();
}

pub(crate) fn part1() {
    println!("day 9 part 1");

    const PREAMBLE: usize = 25;

    let lines = &*LINES;
    let tests = lines.iter().skip(PREAMBLE);
    let windows = lines.windows(PREAMBLE);

    let fail = tests
        .zip(windows)
        .filter_map(|(&test, window)| {
            for x in &window[..] {
                for y in &window[1..] {
                    if x + y == test {
                        return None;
                    }
                }
            }

            Some(test)
        })
        .next()
        .unwrap();

    println!("{}", fail);
}

pub(crate) fn part2() {
    println!("day 9 part 2");

    const INVALID: usize = 257342611;

    let lines = &*LINES;
    let mut start = 0;
    let mut end = 1;

    loop {
        let slice = &lines[start..=end];
        let sum = slice.iter().sum::<usize>();

        match sum.cmp(&INVALID) {
            Ordering::Greater => start += 1,
            Ordering::Less => end += 1,
            Ordering::Equal => {
                if let MinMaxResult::MinMax(x, y) = slice.iter().minmax() {
                    println!("{}", x + y);
                    return;
                }
            }
        }
    }
}

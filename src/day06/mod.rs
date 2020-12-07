use std::collections::HashSet;

static FILE: &str = include_str!("../inputs/6.txt");

lazy_static! {
    static ref FULL_SET: HashSet<char> = ('a'..='z').collect();
}

pub(crate) fn part1() {
    println!("day 6 part 1");

    let num_yes_answers = FILE
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| !char::is_ascii_whitespace(&c))
                .collect::<HashSet<_>>()
                .len()
        })
        .sum::<usize>();

    println!("# yes answers: {:#?}", num_yes_answers);
}

pub(crate) fn part2() {
    println!("day 6 part 2");

    let num_yes_answers = FILE
        .split("\n\n")
        .map(|group| {
            group
                .split_ascii_whitespace()
                .map(|person| {
                    person
                        .chars()
                        .filter(|c| !char::is_ascii_whitespace(&c))
                        .collect::<HashSet<_>>()
                })
                .fold(FULL_SET.clone(), |prev, cur| {
                    cur.intersection(&prev).copied().collect()
                })
                .len()
        })
        .sum::<usize>();

    println!("# yes answers: {}", num_yes_answers);
}

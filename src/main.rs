#![feature(str_split_once)]

#[macro_use]
extern crate lazy_static;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() {
    day01::part1();
    day01::part2();
    day02::part1();
    day02::part2();
    day03::part1();
    day03::part2();
    day04::part1();
    day04::part2();
    day05::part1();
    day05::part2();
    day06::part1();
    day06::part2();
    day07::part1();
    day07::part2();
    day08::part1();
    day08::part2();
}

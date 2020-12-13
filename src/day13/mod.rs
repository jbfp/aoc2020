use itertools::Itertools;

static FILE: &str = include_str!("../inputs/13.txt");

#[derive(Clone, Copy, Debug)]
struct Bus {
    id: usize,
    ttl: usize,
}

lazy_static! {
    static ref LINES: (&'static str, &'static str) = FILE.split_once('\n').unwrap();
    static ref TIMESTAMP: usize = LINES.0.parse().unwrap();
    static ref BUSES: Vec<Bus> = LINES
        .1
        .split(',')
        .filter(|&s| s != "x")
        .map(|s| s.parse().unwrap())
        .map(|id| Bus {
            id,
            ttl: id - (*TIMESTAMP % id)
        })
        .collect_vec();
}

pub(crate) fn part1() {
    println!("day 13 part 1");

    let bus = BUSES.iter().sorted_by_key(|bus| bus.ttl).next().unwrap();

    println!(
        "bus ID {} leaves in {} = {}",
        bus.id,
        bus.ttl,
        bus.id * bus.ttl
    );
}

pub(crate) fn part2() {
    println!("day 13 part 2");
}

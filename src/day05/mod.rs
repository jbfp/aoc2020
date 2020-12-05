use itertools::Itertools;

static FILE: &str = include_str!("../inputs/5.txt");

lazy_static! {
    static ref SEAT_IDS: Vec<usize> = FILE
        .lines()
        .map(|s| {
            let mut row = 0..128;
            let mut col = 0..8;

            for c in s.chars() {
                match c {
                    'F' => row = row.start..(row.end - (row.end - row.start) / 2),
                    'B' => row = (row.start + (row.end - row.start) / 2)..row.end,
                    'L' => col = col.start..(col.end - (col.end - col.start) / 2),
                    'R' => col = (col.start + (col.end - col.start) / 2)..col.end,
                    _ => {}
                }
            }

            row.start * 8 + col.start
        })
        .sorted()
        .collect();
}

pub(crate) fn part1() {
    println!("day 5 part 1");

    let highest_seat_id = SEAT_IDS.last().unwrap();

    println!("Highest seat ID: {:#?}", highest_seat_id);
}

pub(crate) fn part2() {
    println!("day 5 part 2");

    let lowest_seat_id = *SEAT_IDS.first().unwrap();
    let highest_seat_id = *SEAT_IDS.last().unwrap();

    for x in lowest_seat_id..highest_seat_id {
        if x != SEAT_IDS[x - lowest_seat_id] {
            println!("Your seat ID is {}", x);
            break;
        }
    }

}

static FILE: &str = include_str!("../inputs/1.txt");

lazy_static! {
    static ref INPUT: Vec<usize> = FILE.lines().map(|s| s.parse::<usize>().unwrap()).collect();
}

pub(crate) fn part1() {
    println!("day 1 part 1");

    let input = INPUT.to_vec();

    for x in &input {
        for y in &input {
            if x + y == 2020 {
                println!("{} * {} = {}", x, y, x * y);
                return;
            }
        }
    }
}

pub(crate) fn part2() {
    println!("day 1 part 2");

    let input = INPUT.to_vec();

    for x in &input {
        for y in &input {
            for z in &input {
                if x + y + z == 2020 {
                    println!("{} * {} * {} = {}", x, y, z, x * y * z);
                    return;
                }
            }
        }
    }
}

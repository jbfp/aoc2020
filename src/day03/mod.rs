static FILE: &str = include_str!("../inputs/3.txt");

lazy_static! {
    static ref MAP: Vec<&'static [u8]> = FILE.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
}

pub(crate) fn part1() {
    println!("day 3 part 1");
    println!("{} trees", count_trees(3, 1));
}

pub(crate) fn part2() {
    println!("day 3 part 2");

    let result = count_trees(1, 1)
        * count_trees(3, 1)
        * count_trees(5, 1)
        * count_trees(7, 1)
        * count_trees(1, 2);

    println!("{}", result);
}

fn count_trees(right: usize, down: usize) -> usize {
    let mut num_trees = 0;
    let mut col = 0;
    let mut row = 0;

    loop {
        col += right;
        row += down;

        if row >= MAP.len() {
            break;
        }

        let row_size = MAP[row].len();

        if col >= row_size {
            col %= row_size;
        }

        if MAP[row][col] == b'#' {
            num_trees += 1;
        }
    }

    num_trees
}

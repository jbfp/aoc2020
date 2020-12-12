static FILE: &str = include_str!("../inputs/12.txt");

lazy_static! {
    static ref LINES: Vec<(&'static str, i32)> = FILE
        .lines()
        .map(|line| line.split_at(1))
        .map(|(dir, n)| (dir, n.parse().unwrap()))
        .collect();
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    fn rotate_left(&self, degrees: i32) -> Direction {
        let times = degrees / 90;

        (0..times).fold(*self, |dir, _| match dir {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
        })
    }

    fn rotate_right(&self, degrees: i32) -> Direction {
        let times = degrees / 90;

        (0..times).fold(*self, |dir, _| match dir {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        })
    }
}

pub(crate) fn part1() {
    println!("day 12 part 1");

    let mut x = 0;
    let mut y = 0;
    let mut d = Direction::East;

    for &(dir, n) in &*LINES {
        match dir {
            "N" => y += n,
            "S" => y -= n,
            "E" => x += n,
            "W" => x -= n,
            "L" => {
                d = d.rotate_left(n);
            }
            "R" => {
                d = d.rotate_right(n);
            }
            "F" => match d {
                Direction::East => x += n,
                Direction::West => x -= n,
                Direction::North => y += n,
                Direction::South => y -= n,
            },
            _ => unreachable!(),
        }
    }

    println!("({:#?}) |{}| + |{}| = {}", d, x, y, x.abs() + y.abs());
}

pub(crate) fn part2() {
    println!("day 12 part 2");

    let mut x = 0;
    let mut y = 0;

    let mut wx = 10;
    let mut wy = 1;

    for &(action, n) in &*LINES {
        match action {
            "N" => wy += n,
            "S" => wy -= n,
            "E" => wx += n,
            "W" => wx -= n,
            "L" => {
                for _ in 0..n / 90 {
                    let old_wx = wx;
                    wx = -wy;
                    wy = old_wx;
                }
            }
            "R" => {
                for _ in 0..n / 90 {
                    let old_wx = wx;
                    wx = wy;
                    wy = -old_wx;
                }
            }
            "F" => {
                x += n * wx;
                y += n * wy;
            }
            _ => unreachable!(),
        }
    }

    println!("|{}| + |{}| = {}", x, y, x.abs() + y.abs());
}

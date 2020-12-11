use itertools::Itertools;

static FILE: &str = include_str!("../inputs/11.txt");

const FLOOR: char = '.';
const FREE: char = 'L';
const OCCUPIED: char = '#';

lazy_static! {
    static ref INITIAL_SEATS: Vec<Vec<char>> = FILE
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    static ref COL_SIZE: usize = INITIAL_SEATS.len();
    static ref ROW_SIZE: usize = INITIAL_SEATS[0].len();
}

pub(crate) fn part1() {
    println!("day 11 part 1");

    let initial_seats = INITIAL_SEATS.clone();
    let max_col = *COL_SIZE - 1;
    let max_row = *ROW_SIZE - 1;

    let mut previous_seats = initial_seats;
    let mut previous_occupied_seats = 0;

    loop {
        let mut seats = previous_seats.clone();

        for (i, row) in previous_seats.iter().enumerate() {
            for (j, &seat) in row.iter().enumerate() {
                if seat == FLOOR {
                    continue;
                }

                let adjacent_occupied = (i > 0 && j > 0 && previous_seats[i - 1][j - 1] == OCCUPIED) as usize + // top-left
                    (i > 0 && previous_seats[i - 1][j] == OCCUPIED) as usize + // above
                    (i > 0 && j < max_row && previous_seats[i - 1][j + 1] == OCCUPIED) as usize + // top-right
                    (j > 0 && previous_seats[i][j - 1] == OCCUPIED) as usize + // left
                    (j < max_row && previous_seats[i][j + 1] == OCCUPIED) as usize + // right
                    (i < max_col && j > 0 && previous_seats[i + 1][j - 1] == OCCUPIED) as usize + // bottom-left
                    (i < max_col && previous_seats[i + 1][j] == OCCUPIED) as usize + // below
                    (i < max_col && j < max_row && previous_seats[i + 1][j + 1] == OCCUPIED) as usize; // bottom-right;

                if seat == FREE && adjacent_occupied == 0 {
                    seats[i][j] = OCCUPIED;
                } else if seat == OCCUPIED && adjacent_occupied >= 4 {
                    seats[i][j] = FREE;
                }
            }
        }

        let occupied_seats = seats.iter().flatten().filter(|&&c| c == OCCUPIED).count();

        if previous_occupied_seats == occupied_seats {
            break;
        }

        previous_seats = seats;
        previous_occupied_seats = occupied_seats;
    }

    println!("num occupied seats: {}", previous_occupied_seats);
}

enum Direction {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

pub(crate) fn part2() {
    println!("day 11 part 2");

    let mut seats = Seats::new(INITIAL_SEATS.clone());

    while !seats.simulate() {}

    println!("num occupied seats: {}", seats.occupied);
}

struct Seats {
    seats: Vec<Vec<char>>,
    occupied: usize,
    top_row: usize,
    left_col: usize,
    right_col: usize,
    bottom_row: usize,
}

impl Seats {
    fn new(initial: Vec<Vec<char>>) -> Self {
        let num_rows = initial.len();
        let num_cols = initial[0].len();

        Self {
            seats: initial,
            occupied: 0,
            top_row: 0,
            left_col: 0,
            right_col: num_cols - 1,
            bottom_row: num_rows - 1,
        }
    }

    fn simulate(&mut self) -> bool {
        let mut seats = self.seats.clone();
        let mut occupied = self.occupied;

        for (i, row) in self.seats.iter().enumerate() {
            for (j, &seat) in row.iter().enumerate() {
                if seat == FLOOR {
                    continue;
                }

                let seek = |dir| self.seek(i, j, dir);

                let visible_occupied = seek(Direction::TopLeft)
                    + seek(Direction::Top)
                    + seek(Direction::TopRight)
                    + seek(Direction::Left)
                    + seek(Direction::Right)
                    + seek(Direction::BottomLeft)
                    + seek(Direction::Bottom)
                    + seek(Direction::BottomRight);

                if seat == FREE && visible_occupied == 0 {
                    seats[i][j] = OCCUPIED;
                    occupied += 1;
                } else if seat == OCCUPIED && visible_occupied >= 5 {
                    seats[i][j] = FREE;
                    occupied -= 1;
                }
            }
        }

        let done = occupied == self.occupied;
        self.seats = seats;
        self.occupied = occupied;
        done
    }

    fn seek(&self, row: usize, col: usize, dir: Direction) -> usize {
        match dir {
            Direction::TopLeft => {
                if row == self.top_row || col == self.left_col {
                    0
                } else {
                    match self.seats[row - 1][col - 1] {
                        OCCUPIED => 1,
                        FLOOR => self.seek(row - 1, col - 1, dir),
                        _ => 0,
                    }
                }
            }

            Direction::Top => {
                if row == self.top_row {
                    0
                } else {
                    match self.seats[row - 1][col] {
                        OCCUPIED => 1,
                        FLOOR => self.seek(row - 1, col, dir),
                        _ => 0,
                    }
                }
            }
            Direction::TopRight => {
                if row == self.top_row || col == self.right_col {
                    0
                } else {
                    match self.seats[row - 1][col + 1] {
                        OCCUPIED => 1,
                        FLOOR => self.seek(row - 1, col + 1, dir),
                        _ => 0,
                    }
                }
            }
            Direction::Left => {
                if col == self.left_col {
                    0
                } else {
                    match self.seats[row][col - 1] {
                        OCCUPIED => 1,
                        FLOOR => self.seek(row, col - 1, dir),
                        _ => 0,
                    }
                }
            }

            Direction::Right => {
                if col == self.right_col {
                    0
                } else {
                    match self.seats[row][col + 1] {
                        OCCUPIED => 1,
                        FLOOR => self.seek(row, col + 1, dir),
                        _ => 0,
                    }
                }
            }

            Direction::BottomLeft => {
                if row == self.bottom_row || col == self.left_col {
                    0
                } else {
                    match self.seats[row + 1][col - 1] {
                        OCCUPIED => 1,
                        FLOOR => self.seek(row + 1, col - 1, dir),
                        _ => 0,
                    }
                }
            }

            Direction::Bottom => {
                if row == self.bottom_row {
                    0
                } else {
                    match self.seats[row + 1][col] {
                        OCCUPIED => 1,
                        FLOOR => self.seek(row + 1, col, dir),
                        _ => 0,
                    }
                }
            }

            Direction::BottomRight => {
                if row == self.bottom_row || col == self.right_col {
                    0
                } else {
                    match self.seats[row + 1][col + 1] {
                        OCCUPIED => 1,
                        FLOOR => self.seek(row + 1, col + 1, dir),
                        _ => 0,
                    }
                }
            }
        }
    }
}

use std::collections::HashSet;

static FILE: &str = include_str!("../inputs/8.txt");

lazy_static! {
    static ref LINES: Vec<&'static str> = FILE.lines().collect();
}

pub(crate) fn part1() {
    println!("day 8 part 1");
    
    let mut visited = HashSet::new();
    let mut acc = 0;
    let mut pos = 0;

    loop {
        if !visited.insert(pos) {
            break;
        }

        let (instr, n) = LINES[pos as usize].split_once(' ').unwrap();
        let n = n.parse::<i32>().unwrap();

        match instr {
            "nop" => {
                pos += 1;
            }
            "acc" => {
                acc += n;
                pos += 1;
            }
            "jmp" => {
                pos += n;
            }
            _ => println!("unknown instruction {}", instr),
        }
    }

    println!("acc: {}", acc);
}

pub(crate) fn part2() {
    println!("day 8 part 2");
    println!("acc: {}", run(-1));
}

fn run(change: i32) -> i32 {
    let num_lines = LINES.len() as i32;
    let mut visited = HashSet::new();
    let mut acc = 0;
    let mut pos = 0;

    loop {
        if pos >= num_lines {
            break;
        }

        if !visited.insert(pos) {
            return run(change + 1);
        }

        let (instr, n) = LINES[pos as usize].split_once(' ').unwrap();
        let n = n.parse::<i32>().unwrap();
        
        match instr {
            "nop" => {
                if pos == change {
                    pos += n;
                } else {
                    pos += 1;
                }
            }
            "acc" => {
                acc += n;
                pos += 1;
            }
            "jmp" => {
                if pos == change {
                    pos += 1;
                } else {
                    pos += n;
                }
            }
            _ => println!("unknown instruction {}", instr),
        }
    }

    acc
}
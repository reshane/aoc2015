
pub fn solve() {
    let contents = std::fs::read_to_string("input/6.txt").expect("WHERE IS THE FILE");
    let start = std::time::Instant::now();
    println!("part 1: {}", solve_p1(&contents));
    println!("part 1: {}", solve_p2(&contents));
    println!("total execution time: {}ms", start.elapsed().as_millis());
}

use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    On,
    Off,
    Toggle,
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(raw: &str) -> Result<Operation, ()> {
        match &raw[5..=6] {
            "on" => Ok(Operation::On),
            "of" => Ok(Operation::Off),
            "e " => Ok(Operation::Toggle),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    x: (usize, usize),
    y: (usize, usize),
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(raw: &str) -> Result<Self, ()> {
        let dims = raw.split_whitespace()
            .filter(|tok| {
                tok.contains(",")
            })
            .filter_map(|rng_str| {
                if let Some((x,y)) = rng_str.split_once(",") {
                    return Some((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
                }
                None
            }).collect::<Vec<(usize,usize)>>();
        let x = (dims[0].0, dims[1].0);
        let y = (dims[0].1, dims[1].1);
        let op = Operation::from_str(raw).unwrap();
        Ok(Instruction {
            op,
            x,
            y,
        })
    }
}

fn solve_p1(contents: &str) -> i64 {
    let instructions = contents.lines()
        .filter_map(|line| {
            Instruction::from_str(line).ok()
        }).collect::<Vec<Instruction>>();
    let mut grid = vec![vec![0; 1000]; 1000];
    for ins in instructions.iter() {
        for x in ins.x.0..=ins.x.1 {
            for y in ins.y.0..=ins.y.1 {
                grid[y][x] = match ins.op {
                    Operation::On => 1,
                    Operation::Off => 0,
                    Operation::Toggle => {
                        if grid[y][x] == 1 {
                            0
                        } else {
                            1
                        }
                    }
                }
            }
        }
    }
    grid.iter()
        .map(|r| { r.iter().sum::<i64>() })
        .sum::<i64>()
}

#[test]
fn test_case_1() {
    let contents = std::fs::read_to_string("test/6.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(&contents);
    println!("{result}");
    assert!(result == 1000);
}

fn solve_p2(contents: &str) -> i64 {
    let instructions = contents.lines()
        .filter_map(|line| {
            Instruction::from_str(line).ok()
        }).collect::<Vec<Instruction>>();
    let mut grid = vec![vec![0; 1000]; 1000];
    for ins in instructions.iter() {
        for x in ins.x.0..=ins.x.1 {
            for y in ins.y.0..=ins.y.1 {
                match ins.op {
                    Operation::On => {
                        grid[y][x] += 1;
                    },
                    Operation::Off => {
                        grid[y][x] = std::cmp::max(
                            0,
                            grid[y][x] - 1
                        );
                    },
                    Operation::Toggle => {
                        grid[y][x] += 2;
                    }
                }
            }
        }
    }
    grid.iter()
        .map(|r| { r.iter().sum::<i64>() })
        .sum::<i64>()
}

#[test]
fn test_case_2() {
    let contents = std::fs::read_to_string("test/6.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(&contents);
    println!("{result}");
    assert!(result == 2000);
}

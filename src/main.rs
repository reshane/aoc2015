mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        3.. => println!("Too many args, pick a day"),
        2 => {
            if let Ok(day) = args[1].parse::<usize>() {
                match day {
                    1 => day1::solve(),
                    2 => day2::solve(),
                    3 => day3::solve(),
                    4 => day4::solve(),
                    5 => day5::solve(),
                    _ => println!("No implementation yet"),
                }
            } else {
                println!("Input a valid day");
            }
        },
        1 => solve(),
        _ => unreachable!("How did you do this"),
    }
}

pub fn solve() {
    let contents = std::fs::read_to_string("input/6.txt").expect("WHERE IS THE FILE");
    let start = std::time::Instant::now();
    println!("part 1: {}", solve_p1(&contents));
    println!("part 1: {}", solve_p2(&contents));
    println!("total execution time: {}ms", start.elapsed().as_millis());
}

fn solve_p1(_contents: &str) -> i64 {
    0
}

#[test]
fn test_case_1() {
    let contents = std::fs::read_to_string("test/6.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(&contents);
    println!("{result}");
    assert!(result == 0);
}

fn solve_p2(_contents: &str) -> i64 {
    0
}

#[test]
fn test_case_2() {
    let contents = std::fs::read_to_string("test/6.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(&contents);
    println!("{result}");
    assert!(result == 0);
}


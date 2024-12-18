use std::time::Instant;

pub fn solve() {
    let contents = std::fs::read_to_string("input/1.txt").expect("WHERE IS THE FILE");
    let start = Instant::now();
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
    println!("total execution time: {}ms", start.elapsed().as_millis());
}

fn solve_p1(contents: String) -> isize {
    let mut floor = 0;
    contents.chars()
        .for_each(|c| {
            match c {
                ')' => floor -= 1,
                '(' => floor += 1,
                _ => {},
            }
        });
    floor
}

#[test]
fn test_case_1a() {
    assert!(0 == solve_p1("(())".to_string()));
}

#[test]
fn test_case_1b() {
    assert!(3 == solve_p1("(((".to_string()));
}

fn solve_p2(contents: String) -> usize {
    let mut floor = 0;
    let mut idx = 0;
    for (i, c) in contents.chars().enumerate() {
        match c {
            ')' => floor -= 1,
            '(' => floor += 1,
            _ => {},
        }
        if floor < 0 {
            idx = i + 1;
            break;
        }
    }
    idx
}

#[test]
fn test_case_2a() {
    assert!(5 == solve_p2("()())".to_string()));
}

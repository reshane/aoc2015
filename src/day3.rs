use std::collections::HashSet;

pub fn solve() {
    let contents = std::fs::read_to_string("input/3.txt").expect("WHERE IS THE FILE");
    let start = std::time::Instant::now();
    println!("part 1: {}", solve_p1(&contents));
    println!("part 1: {}", solve_p2(&contents));
    println!("total execution time: {}ms", start.elapsed().as_millis());
}

fn solve_p1(contents: &str) -> i64 {
    let mut curr: (i64, i64) = (0,0);
    let mut visited = HashSet::<(i64,i64)>::new();
    visited.insert(curr);
    contents
        .chars()
        .for_each(|c| {
            match c {
                '^' => curr.1 += 1,
                'v' => curr.1 -= 1,
                '>' => curr.0 += 1,
                '<' => curr.0 -= 1,
                _ => {},
            }
            visited.insert(curr);
        });
    visited.len() as i64
}

#[test]
fn test_case_1() {
    let contents = std::fs::read_to_string("test/3.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(&contents);
    println!("{result}");
    assert!(result == 2);
}

fn solve_p2(contents: &str) -> i64 {
    let mut s: (i64, i64) = (0,0);
    let mut rs: (i64, i64) = (0,0);
    let mut visited = HashSet::<(i64,i64)>::new();
    visited.insert(s);
    contents
        .chars()
        .enumerate()
        .for_each(|(i,c)| {
            if i % 2 == 0 {
                match c {
                    '^' => s.1 += 1,
                    'v' => s.1 -= 1,
                    '>' => s.0 += 1,
                    '<' => s.0 -= 1,
                    _ => {},
                }
                visited.insert(s);
            } else {
                match c {
                    '^' => rs.1 += 1,
                    'v' => rs.1 -= 1,
                    '>' => rs.0 += 1,
                    '<' => rs.0 -= 1,
                    _ => {},
                }
                visited.insert(rs);
            }
        });
    visited.len() as i64
}

#[test]
fn test_case_2() {
    let contents = std::fs::read_to_string("test/3.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(&contents);
    println!("{result}");
    assert!(result == 11);
}


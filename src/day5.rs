
pub fn solve() {
    let contents = std::fs::read_to_string("input/5.txt").expect("WHERE IS THE FILE");
    let start = std::time::Instant::now();
    println!("part 1: {}", solve_p1(&contents));
    println!("part 1: {}", solve_p2(&contents));
    println!("total execution time: {}ms", start.elapsed().as_millis());
}

const VOWELS: &'static [char] = &[
    'a', 'e', 'i', 'o', 'u'
];

fn solve_p1(contents: &str) -> i64 {
    let nice = contents.lines()
        .filter(|line| {
            line.as_bytes()
                .windows(2)
                .any(|w| { w[0] == w[1] })
            && 
            line.as_bytes()
                .windows(2)
                .all(|w| {
                    !(w[0] == b'a' && w[1] == b'b') &&
                    !(w[0] == b'c' && w[1] == b'd') &&
                    !(w[0] == b'p' && w[1] == b'q') &&
                    !(w[0] == b'x' && w[1] == b'y')
                })
            &&
            line.chars()
                .fold(0, |acc, c| {
                    if VOWELS.contains(&c) {
                        return acc + 1;
                    }
                    return acc;
                }) > 2
        }).collect::<Vec<&str>>();
    nice.len() as i64
}

#[test]
fn test_case_1() {
    let contents = std::fs::read_to_string("test/5.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(&contents);
    println!("{result}");
    assert!(result == 2);
}

use std::collections::HashMap;

fn solve_p2(contents: &str) -> i64 {
    let nice = contents.lines()
        .filter(|line| {
            let mut pairs = HashMap::<(char, char), usize>::new();
            line.chars()
                .collect::<Vec<char>>()
                .windows(2)
                .enumerate()
                .any(|(i, w)| {
                    if let Some(idx) = pairs.get(&(w[0], w[1])) {
                        return *idx < i - 1;
                    }
                    pairs.insert((w[0], w[1]), i);
                    false
                })
            && 
            line.as_bytes()
                .windows(3)
                .any(|w| {
                    w[0] == w[2]
                })
        }).collect::<Vec<&str>>();
    nice.len() as i64
}

#[test]
fn test_case_2() {
    let contents = std::fs::read_to_string("test/5b.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(&contents);
    println!("{result}");
    assert!(result == 2);
}


pub fn solve() {
    let contents = std::fs::read_to_string("input/4.txt").expect("WHERE IS THE FILE");
    let start = std::time::Instant::now();
    println!("part 1: {}", solve_p1(&contents));
    println!("part 1: {}", solve_p2(&contents));
    println!("total execution time: {}ms", start.elapsed().as_millis());
}

fn solve_p1(contents: &str) -> i64 {
    let mut i = 0;
    let contents = contents.trim();
    let mut digest = md5::compute(format!("{contents}{i}"));
    while &format!("{:x}", digest)[0..5] != "00000" {
        i += 1;
        digest = md5::compute(format!("{contents}{i}"));
    }
    i
}

#[test]
#[cfg_attr(not(feature = "long_tests"), ignore)]
fn test_case_1() {
    let contents = std::fs::read_to_string("test/4.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(&contents);
    println!("{result}");
    assert!(result == 609043);
}

fn solve_p2(contents: &str) -> i64 {
    let mut i = 0;
    let contents = contents.trim();
    let mut digest = md5::compute(format!("{contents}{i}"));
    while &format!("{:x}", digest)[0..6] != "000000" {
        i += 1;
        digest = md5::compute(format!("{contents}{i}"));
    }
    i
}

#[test]
#[cfg_attr(not(feature = "long_tests"), ignore)]
fn test_case_2() {
    let contents = std::fs::read_to_string("test/4.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(&contents);
    println!("{result}");
    assert!(result == 6742839);
}

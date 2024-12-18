
pub fn solve() {
    let contents = std::fs::read_to_string("input/2.txt").expect("WHERE IS THE FILE");
    let start  = std::time::Instant::now();
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
    println!("total execution time: {}ms", start.elapsed().as_millis());
}

fn solve_p1(contents: String) -> i64 {
    contents
        .lines()
        .map(|line| {
            let mut dims: Vec<i64> = line.split("x")
                .filter_map(|d| d.parse::<i64>().ok() )
                .collect();
            dims.sort();
            dims
        })
        .fold(0, |acc, x| acc + 3 * x[0] * x[1]
                              + 2 * x[1] * x[2]
                              + 2 * x[0] * x[2])
}

#[test]
fn test_case_1() {
    let input = vec![
        "2x3x4",
    ];
    let result = solve_p1(input.join("\n"));
    println!("{result}");
    assert!(result == 58);
}

fn solve_p2(contents: String) -> i64 {
    contents
        .lines()
        .map(|line| {
            let mut dims: Vec<i64> = line.split("x")
                .filter_map(|d| d.parse::<i64>().ok() )
                .collect();
            dims.sort();
            dims
        })
        .fold(0, |acc, x| acc + 2 * x[0]
                              + 2 * x[1]
                              + x[0] * x[1] * x[2])
}

#[test]
fn test_case_2() {
    let input = vec![
        "1x1x10",
        "2x3x4",
    ];
    let result = solve_p2(input.join("\n"));
    println!("{result}");
    assert!(result == 48);
}

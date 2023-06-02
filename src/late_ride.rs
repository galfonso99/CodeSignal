fn solution(n: i32) -> i32 {
    format!("{}{}", n/60, n%60).chars().map(|c| c.to_digit(10).unwrap() as i32).sum()
}

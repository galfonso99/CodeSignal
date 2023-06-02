fn solution(n: i32) -> i32 {
    n.to_string().chars().filter_map(|c| c.to_digit(10)).sum::<u32>() as i32
}
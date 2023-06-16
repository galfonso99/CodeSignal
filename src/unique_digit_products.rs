use std::collections::HashSet;
fn solution(a: Vec<i32>) -> i32 {
    a.into_iter().map(|a| product(a)).collect::<HashSet<i32>>().len() as i32
}

fn product(x: i32) -> i32 {
    x.to_string().chars().filter_map(|c| c.to_digit(10)).product::<u32>() as i32
}
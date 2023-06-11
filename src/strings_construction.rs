fn solution(a: String, b: String) -> i32 {
    a.chars().collect::<std::collections::HashSet<char>>().iter().collect::<String>().chars()
    .map(|c| b.matches(c).count() / a.matches(c).count() ).min().unwrap() as i32
}
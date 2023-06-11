fn solution(s: String) -> bool {
    s.to_lowercase().chars().rev().collect::<String>() == s.to_lowercase()
}
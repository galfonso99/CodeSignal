/* Given two strings a and b, both consisting only of lowercase English letters, your task is to calculate how many strings equal to a can be constructed using only letters from the string b? Each letter can be used only once and in one string only. */

fn stringsConstruction(a: String, b: String) -> i32 {
    a.chars().collect::<std::collections::HashSet<char>>().iter().collect::<String>().chars()
    .map(|c| b.matches(c).count() / a.matches(c).count() ).min().unwrap() as i32
}
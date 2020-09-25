/* Check if a string is a palindrome
using Rust of course */


fn is_palindrome(string: String) -> bool {
    let half_len = string.len() / 2;
    string
        .chars()
        .take(half_len)
        .eq(string.chars().rev().take(half_len))
}

fn is_palindrome2(string: String) -> bool {
    return string.chars().rev().collect::<String>() == string;
}

fn is_palindrome3(string: String) -> bool {
    return string.chars().eq(string.chars().rev())
}


pub fn run() {
    let string = "racecar".to_string();
    println!("{}", is_palindrome2(string));
}
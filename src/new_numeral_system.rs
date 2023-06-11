fn solution(num: char) -> Vec<String> {
    (0..=(num as u8 - 65) / 2).map(|i| format!("{} + {}", (b'A' + i) as char, (num as u8 - i) as char)).collect()
}
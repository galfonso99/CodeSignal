fn solution(message: String) -> String {
    let mut s = String::new();
    message.chars().fold(0u32, |acc, x| {
        let a = (acc - ((acc / 26) * 26)) as u8;
        let b = if a > x as u8 - 97 {26+x as u8 - a} else {x as u8 - a};
        s.push(b as char);
        acc + b as u32 - 97
    });
    s
}
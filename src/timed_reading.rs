fn solution(max: i32, text: String) -> i32 {
    text.split(|c:char| !c.is_ascii_alphanumeric()).filter(|s| s.len() >0 && s.len() <= max as usize).count() as i32
}
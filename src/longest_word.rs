
fn solution(mut text: String) -> String {
    text.split_terminator(|c:char| !c.is_ascii_alphabetic()).max_by_key(|s| s.len()).unwrap().to_string()
}
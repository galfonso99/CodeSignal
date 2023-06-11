fn solution(note: String) -> String {
    note.chars().map(|c| {
        match c {
            '0'..='9' => (c as u8 + 49) as char,
            'a'..='j' => (c as u8 - 49) as char,
            _ => c
        }
    }).collect()
}
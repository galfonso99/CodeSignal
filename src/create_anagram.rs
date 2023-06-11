fn solution(s: String, mut t: String) -> i32 {
    for c in s.chars() {
        t= t.replacen(&c.to_string(), "", 1);
    }
    t.len() as i32
}
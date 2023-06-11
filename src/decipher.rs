fn solution(cipher: String) -> String {
    let mut s = String::new();
    let mut tmp = String::new();
    for c in cipher.chars() {
        tmp.push(c);
        if tmp.parse::<u8>().unwrap() > 96 {
            s.push(tmp.parse::<u8>().unwrap() as char);
            tmp.clear();
        }
    }
    s
}
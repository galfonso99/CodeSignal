fn solution(s: String) -> String {
    s.bytes().map(|c| (b'm'*2-c + 1) as char).collect()
}
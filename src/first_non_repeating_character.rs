fn solution(s: String) -> char {
    for c in s.chars() {
        if s.find(c) == s.rfind(c) {return c;}
    }
    '_'
}

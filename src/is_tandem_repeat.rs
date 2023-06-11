fn solution(mut s: String) -> bool {
    s.drain(..s.len()/2).collect::<String>() == s
}
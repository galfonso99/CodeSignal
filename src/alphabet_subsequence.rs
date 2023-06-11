fn solution(s: String) -> bool {
    s.chars().collect::<Vec<char>>().windows(2).all(|sl| sl[0] < sl[1])
}
fn solution(s1: i32, s2: i32) -> bool {
    s1.min(s2) < 5 && s1.max(s2) == 6 || s1.min(s2) < 7 && s1.min(s2) > 4 && s1.max(s2) == 7
}

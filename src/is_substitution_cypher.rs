fn solution(s1: String, s2: String) -> bool {
    use std::collections::HashSet;
    s1.chars().zip(s2.chars()).collect::<HashSet<(char,char)>>().len() ==
    s1.chars().collect::<HashSet<char>>().len() && s2.chars().collect::<HashSet<char>>().len() ==
    s1.chars().collect::<HashSet<char>>().len()
}
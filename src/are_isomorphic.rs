fn solution(a1: Vec<Vec<i32>>, a2: Vec<Vec<i32>>) -> bool {
    a1.len() == a2.len() && 
    (0..a1.len()).all(|i| a1[i].len() == a2[i].len())
}
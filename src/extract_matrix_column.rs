fn solution(m: Vec<Vec<i32>>, c: i32) -> Vec<i32> {
    (0..m.len()).map(|i| m[i][c as usize]).collect()
}
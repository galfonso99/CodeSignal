fn solution(m: Vec<Vec<i32>>, a: i32, b: i32) -> i32 {
    m[a as usize].iter().sum::<i32>() + (0..m.len()).map(|i| m[i][b as usize]).sum::<i32>() - m[a as usize][b as usize]
}
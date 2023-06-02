fn solution(a: i32, b: i32) -> i32 {
    (a..=b).map(|x| x.count_ones() as i32).sum()
}
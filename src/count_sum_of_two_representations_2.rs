fn solution(n: i32, l: i32, r: i32) -> i32 {
    (l..=r).filter(|&x| n - x >= x && n - x <= r).count() as i32
}
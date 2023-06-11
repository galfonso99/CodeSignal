fn solution(a: i32, b: i32, n: i32) -> i32 {
    (0..n).fold(0, |acc, x| (a+x) * (b+x) + acc)
}
fn solution(n: i32, m: i32) -> i32 {
    m + n + gcd( m, n) - 2
}
fn gcd(a: i32, b: i32) -> i32 {
    if a%b==0 {b} else {gcd(b, a%b)}
}
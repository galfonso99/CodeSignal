fn solution(k: i32) -> i32 {
    (1..=k).map(|n| if n%2==0 {n*n} else {-n*n}).sum()
}
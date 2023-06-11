fn solution(a: Vec<i32>) -> i32 {
    a.iter().take_while(|&x| x!=&0).sum()
}
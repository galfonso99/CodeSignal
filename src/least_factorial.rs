fn solution(n: i32) -> i32 {
    let mut res = 1;
    for i in 2.. {
        if res >= n {return res;}
        res *= i;
    }
    return res;
}
fn solution(mut n1: i32, mut n2: i32) -> i32 {
    let (mut res, mut cnt) = (0,1);
    while n1 != 0 || n2 != 0 {
        res += (n1 + n2) % 10 * cnt;
        cnt *= 10;
        n1 /= 10;
        n2 /= 10;
    }
    res
}


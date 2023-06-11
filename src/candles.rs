fn solution(mut solution: i32, mut parts: i32) -> i32 {
    let (mut left, mut cnt) = (0,0);
    while solution > 0 {
        left += solution; cnt+= solution; solution = 0;
        if left >= parts {
            solution = left / parts; left%=parts;
        }
    }
    cnt
}

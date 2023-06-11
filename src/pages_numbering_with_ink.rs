fn solution(mut cur: i32, mut n: i32) -> i32 {
    while n >= cur.to_string().len() as i32 {
        n -= cur.to_string().len() as i32;
        cur+=1;
    }
    cur-1
}

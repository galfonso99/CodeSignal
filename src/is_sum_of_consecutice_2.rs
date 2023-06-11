fn solution(n: i32) -> i32 {
    let mut cnt = 0;
    for i in 1..n {
        let mut acc = 0;
        for j in i..n {
            acc+=j;
            if acc >= n {
                if acc == n {cnt+=1;}
                break;
            }
            
        }
    }
    cnt
}

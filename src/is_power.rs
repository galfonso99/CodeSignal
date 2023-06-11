fn solution(n: i32) -> bool {
    for b in 2..10 {
        let mut a:i32 = 1;
        while a.pow(b) <= n {
            if a.pow(b) == n {return true};
            a+=1;
        }
    }
    false
    
}

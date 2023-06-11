fn solution(s: String, l: i32, r: i32) -> bool {
    if l <= s.len() as i32 && s.len() as i32 <= r {return true;}
    for n in 2..=s.len()/l as usize {
        let mut spc = true;
        let len = s.len()/n;
        for x in 1..n {
            spc = s.chars().nth(x*len+x-1).unwrap() == ' ';
            if !spc {break;}
        }
        spc = spc && l<=len as i32 && len as i32<=r;
        if spc {return true}
    }
    return false
}
fn solution(coms: String) -> i32 {
    let mut same = true;
    let mut cnt = 0;
    for c in coms.chars() {
        match c {
            'L' => if !same {cnt+=1; same = true;} else {same = false;}, 
            'R' => if !same {cnt+=1; same = true;} else {same = false;}, 
            'A' => if same {cnt+=1;}, 
            _ => {}
        };
    }
    cnt
}

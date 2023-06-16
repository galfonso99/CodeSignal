fn solution(dob: String) -> i32 {
    let date = interpret(dob);
    let before = date[0] < 3 && date[1] < 29;
    let mut cnt = if is_leap(date[2]) && before {1} else {0};
    if date[0] == 2 && date[1] == 29 {
        cnt = 0;
        for y in date[2].. {
            if is_leap(y) && cnt == 7 {return y - date[2];}
            if cnt > 7 {cnt-=7;};
            if is_leap(y) {cnt+=2;}
            else {cnt+=1}
        }
    }
    for y in date[2]+1.. {
        cnt+=1;
        if is_leap(y) && !before {cnt+=1;}
        if cnt == 7 {return y - date[2];}
        if cnt > 7 {cnt-=7;}
        if is_leap(y) && before {cnt+=1;}
    }
    cnt
}

fn interpret(date: String) -> Vec<i32> {
    date.split('-').map(|s| s.parse::<i32>().unwrap()).collect()
}

fn is_leap(y :i32) -> bool {
    y % 400 == 0 || (y%4==0 && y%100!=0)
}

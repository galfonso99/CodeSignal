fn solution(min1: i32, min2_10: i32, min11: i32, mut s: i32) -> i32 {
    let (mut min, mut cnt) = (0,0); 
    if min1 <= s {s-=min1; min+=1;} else {return 0;}
    while min2_10 < s && cnt < 9 {
        s-=min2_10;
        cnt+=1;
    }
    min+=cnt;
    if min > 9 {min+=s/min11;}
    min
}
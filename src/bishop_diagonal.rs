fn solution(mut b1: String, mut b2: String) -> Vec<String> {
    let mut original = vec![b1.to_string(),b2.to_string()];
    original.sort();
    let (mut y1, mut x1) = (b1.pop().unwrap().to_digit(10).unwrap() as i32, (b1.pop().unwrap() as u8 - 96) as i32 );
    let (mut y2, mut x2) = (b2.pop().unwrap().to_digit(10).unwrap() as i32, (b2.pop().unwrap() as u8 - 96) as i32 );
    if (x1-x2).abs() != (y1-y2).abs() {return original;}
    let (x,y) = (x1-x2, y1-y2);
    if x < 0 {
        if y < 0 {
            let d1 = x1.min(y1) - 1; 
            x1 -= d1;
            y1 -= d1;
            let d2 = 8 - x2.max(y2);
            x2 += d2;
            y2 += d2;
        } else {
            let d1 = (x1-1).min(8-y1);
            x1-=d1;
            y1+=d1;
            let d2 = (8-x2).min(y2-1);
            x2+=d2;
            y2-=d2;
        }
    }
    else {
        if y < 0 {
            let d1 = (8-x1).min(y1-1);
            x1+=d1;
            y1-=d1;
            let d2 = (x2-1).min(8-y2);
            x2-=d2;
            y2+=d2;
        } else {
            let d1 = 8 - x1.max(y1);
            x1 += d1;
            y1 += d1;
            let d2 = x2.min(y2) - 1;
            x2 -= d2;
            y2 -= d2;
        }
    }
    let mut ans = vec![convert(x1,y1), convert(x2,y2)];
    ans.sort();
    return ans;
}
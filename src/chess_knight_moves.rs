fn solution(mut cell: String) -> i32 {
    let (mut y, mut x) = (cell.pop().unwrap().to_digit(10).unwrap() as i32, (cell.pop().unwrap() as u8 - 96) as i32 );
    let mut cnt = 0;
    if y < 7 {
        if x > 1 {cnt +=1;}
        if x <8 {cnt +=1;}
    }
    if x < 7 {
        if y > 1 {cnt +=1;}
        if y <8 {cnt +=1;}
    }
    if y >2 {
        if x > 1 {cnt +=1;}
        if x <8 {cnt +=1;}
    }
    if x > 2 {
        if y > 1 {cnt +=1;}
        if y <8 {cnt +=1;}
    }
    cnt
}
fn solution(mut p: String) -> bool {
    let (mut y4, mut x4) = (p.pop().unwrap().to_digit(10).unwrap() as i32, (p.pop().unwrap() as u8 - 96) as i32 );
    p.pop();
    let (mut y3, mut x3) = (p.pop().unwrap().to_digit(10).unwrap() as i32, (p.pop().unwrap() as u8 - 96) as i32 );
    p.pop();
    let (mut y2, mut x2) = (p.pop().unwrap().to_digit(10).unwrap() as i32, (p.pop().unwrap() as u8 - 96) as i32 );
    p.pop();
    let (mut y1, mut x1) = (p.pop().unwrap().to_digit(10).unwrap() as i32, (p.pop().unwrap() as u8 - 96) as i32 );
    let mut cnt = 0;
    cnt += if is_white(x1,y1) {1} else {0};
    cnt += if is_white(x2,y2) {1} else {0};
    cnt += if is_white(x3,y3) {1} else {0};
    cnt += if is_white(x4,y4) {1} else {0};
    cnt % 2 == 0
}

fn is_white(x: i32, y:i32) -> bool {
    (x + y) % 2 == 1
}
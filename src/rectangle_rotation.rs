fn solution(a: i32, b: i32) -> i32 {
    let y = (a as f32 / (2.0f32.sqrt())).ceil() as i32;
    let x = (b as f32 / (2.0f32.sqrt())).ceil() as i32;
    let mut res = y*x+((y-1)*(x-1));
    if res%2==1 {res} else {res-1}
    
}
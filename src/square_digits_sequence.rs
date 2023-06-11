fn solution(a0: i32) -> i32 {
    let (mut temp, mut sum, mut cnt) = (0, a0, 1);
    let mut arr = vec![a0];
    while !arr.contains(&temp) {
        arr.push(temp);
        temp = 0;
        for c in sum.to_string().chars() {
            temp += (c.to_digit(10).unwrap() as i32).pow(2);
        }
        sum = temp;
        cnt+=1;
    } 
    cnt
}

fn solution(mut arr: Vec<i32>) -> i32 {
    let mut i = 1;
    while arr.len() > 1 {
        if i%2==1 {
            arr = arr.chunks(2).map(|s| s[0] + s[1]).collect();
        }
        else {
            arr = arr.chunks(2).map(|s| s[0] * s[1]).collect();
        }
        i+=1;
    }
    arr[0]
}
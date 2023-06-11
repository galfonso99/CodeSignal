fn solution(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    if len%2==0 {
        let mid = arr.drain(len/2-1..=len/2).sum();
        arr.insert(len/2-1, mid);
    }
    arr
}

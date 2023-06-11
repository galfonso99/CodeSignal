fn solution(arr: Vec<i32>) -> bool {
    let len = arr.len();
    let middle = if len%2==1 {arr[len/2]} else {arr[len/2] + arr[len/2 - 1]};
    arr.first().unwrap() == arr.last().unwrap() &&  arr.last().unwrap() == &middle
    
}

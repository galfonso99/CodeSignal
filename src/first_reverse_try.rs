fn solution(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    if len < 1 {return arr;}
    arr.swap(0, len-1);
    arr
}

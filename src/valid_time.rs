fn solution(t: String) -> bool {
    let arr = t.split(':').map(|e| e.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    if arr[0] < 24 && arr[1] < 60 {true} else {false}
}

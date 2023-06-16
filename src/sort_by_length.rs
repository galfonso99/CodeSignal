fn solution(mut arr: Vec<String>) -> Vec<String> {
    arr.sort_by(|a,b| a.len().cmp(&b.len()));
    arr
}
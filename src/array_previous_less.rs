fn solution(items: Vec<i32>) -> Vec<i32> {
    items.tp().into_iter().enumerate().map(|(i,x)| 
    items.clone().into_iter().take(i).rfind(|&a| a < x).unwrap_or(-1)).collect()
}
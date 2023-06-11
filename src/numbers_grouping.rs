fn solution(a: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut v:HashMap<i32,i32> = HashMap::new();
    a.iter().for_each(|&x| {v.insert(div_by_10_4(x),x);});
    (v.len() + a.len()) as i32
}

fn div_by_10_4 (n:i32) -> i32 {
    ((n-1) as f64 / 10_000f64) as i32
}
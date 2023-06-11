use std::collections::HashSet;
fn solution(div: Vec<i32>, k: i32) -> i32 {
    let mut h = HashSet::new();
    for x in 1..=k {
        let mut a :Vec<i32>= Vec::new();
        for i in div.iter() {
            if x % i == 0 {a.push(1);}
            else {a.push(0);}
        }
        h.insert(a);
    }
    h.len() as i32
}
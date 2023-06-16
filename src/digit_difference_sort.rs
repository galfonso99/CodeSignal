use std::cmp::Ordering;
fn solution(mut a: Vec<i32>) -> Vec<i32> {
    a.reverse();
    a.sort_by(|a,b| ord(a,b));
    a
}

fn diff(mut n: i32) -> i32 {
    let digits = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
    let max = digits.iter().max().unwrap();
    let min = digits.iter().min().unwrap();
    max-min
}


fn ord(a: &i32, b: &i32) -> Ordering {
    diff(*a).cmp(&diff(*b))
}
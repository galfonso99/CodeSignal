fn solution(a: i32) -> i32 {
    i32::from_str_radix(format!("{:b}", a).chars().rev().collect::<String>().as_str(), 2).unwrap()
}
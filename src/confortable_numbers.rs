fn solution(l: i32, r: i32) -> i32 {
    (l..r).map(|a| (a+1..=r).filter(|&x| comf(a,x)).count()).sum::<usize>() as i32
}

fn sum (n: i32) -> i32 {
    n.to_string().chars().filter_map(|c| c.to_digit(10)).sum::<u32>() as i32
}

fn comf (a: i32, b: i32) -> bool {
    b>= a-sum(a) && b<= a+sum(a) && b!=a && a>= b-sum(b) && a<= b+sum(b)
}
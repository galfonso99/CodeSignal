fn solution(n: i32) -> i32 {
    let mut a = (1..).scan(n, |st, x| {
        let sum = sum(*st);
        *st = *st - sum;
        Some(sum)
    }).take_while(|x| x>&0).collect::<Vec<i32>>();
    a.sort();
    *a.iter().max_by(|x,y| count(&a,*x).cmp(&count(&a,*y))).unwrap() as i32
}

fn sum (n:i32) -> i32 {
    n.to_string().chars().filter_map(|c| c.to_digit(10)).sum::<u32>() as i32
}

fn count(a:&[i32], n:&i32) -> i32 {
    a.to_vec().iter().filter(|&x| x == n).count() as i32
}
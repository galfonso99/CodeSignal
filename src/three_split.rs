fn solution(a: Vec<i32>) -> i32 {
    let sum = a.iter().sum();
    let mut first_sum = 0;
    let mut count = 0;
    for split1 in 0..a.len() - 2 {
        first_sum += a[split1];
        if first_sum * 3 != sum {
            continue;
        }
        let mut second_sum = 0;
        for split2 in split1 + 1..a.len() - 1 {
            second_sum += a[split2];
            if second_sum * 3 == sum {
                count += 1;
            }
        }
    }
    count
}
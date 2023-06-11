fn can_create(a: &str, b: &str, r: &str) -> bool {
    a.chars()
        .zip(b.chars())
        .zip(r.chars())
        .all(|((a, b), r)| a == r || b == r)
}

fn solution(inputArray: Vec<String>, result: String) -> i32 {
    inputArray
        .iter()
        .enumerate()
        .flat_map(|(i, a)| inputArray[(i + 1)..].iter().map(move |b| (a, b)))
        .filter(|(a, b)| can_create(a, b, &result))
        .count() as i32
}
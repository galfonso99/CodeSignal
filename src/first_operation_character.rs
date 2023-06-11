fn solution(s: String) -> i32 {
    use std::collections::HashMap;
    let prio = HashMap::from([('*', 3), ('+', 2)]);
    s.chars().map(|c| (1,1,0,c)).enumerate()
        .fold((1, 0, 0, ' '), |(multi, highest, high_ind, _), (ind, (_, _, _, c))| {
            match c {
                '(' => (multi * 2, highest, high_ind, c),
                ')' => (multi / 2, highest, high_ind, c),
                '*' | '+'  if prio.get(&c).unwrap() * multi > highest 
                        => (multi, prio.get(&c).unwrap() * multi, ind, c),
                _ => (multi, highest, high_ind, c)
            }
        }).2 as i32
}
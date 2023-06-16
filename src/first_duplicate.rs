fn solution(a: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut S:HashSet<i32> = HashSet::new();
    for x in &a {
        if S.contains(x) {
            return *x;
        }
        else {S.insert(*x);}
    }
    return -1;
}
fn solution(l: Vec<i32>, w: Vec<i32>, h: Vec<i32>) -> bool {
    let mut arr = vec![[0,0,0];l.len()];
    (0..l.len()).for_each(|i| {let mut tmp = [l[i], w[i], h[i]]; tmp.sort(); arr[i] = tmp;});
    arr.sort_by_key(|x| x[0]+x[1]+x[2]);
    arr.windows(2).all(|t| (0..t.len()).all(|i| t[1][i] > t[0][i]))
}
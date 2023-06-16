fn solution(mut m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    (0..m.len()/2).for_each(|i| {
        let bottom = m.len()-1-i;
        let tmp1 = m[bottom][bottom];
        let tmp2 = m[bottom][i];
        m[bottom][bottom] = m[i][i];
        m[i][i] = tmp1;
        m[bottom][i] = m[i][bottom];
        m[i][bottom] = tmp2;
    });
    m
    
}
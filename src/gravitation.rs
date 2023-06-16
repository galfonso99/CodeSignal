fn solution(rows: Vec<String>) -> Vec<i32> {
    let rws = rows.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let vec = (0..rws[0].len()).map(|i| {
        let mut tmp = Vec::new();
        for j in 0..rws.len() {
            tmp.push(rws[j][i]);
        }
        steps(tmp)
    }).collect::<Vec<i32>>();
    let min = vec.clone().into_iter().min().unwrap_or(0);
    vec.iter().enumerate().filter(|t| *t.1 == min).map(|t| t.0 as i32).collect::<Vec<i32>>()
}
fn steps(arr: Vec<char>) -> i32 {
    arr.into_iter().skip_while(|&c| c == '.').filter(|&c| c=='.').count() as i32
}
fn solution(matrix: Vec<Vec<i32>>) -> bool {
    let mut row_1 = (0..matrix.len()).map(|i| (i, matrix[i][0])).collect::<Vec<(usize,i32)>>();
    row_1.sort_by(|a, b| a.1.cmp(&b.1));
    if !asc(row_1.iter().map(|&(_,val)| val).collect::<Vec<i32>>()) {return false}
    let mut order = row_1.iter().map(|&(i,_)| i).collect::<Vec<usize>>();
    (1..matrix[0].len()).map(|i| order.iter().map(|&j| matrix[j][i]).collect::<Vec<i32>>())
    .all(|arr| asc(arr))
}

fn asc(arr: Vec<i32>) -> bool {
    for i in 0..arr.len()-1 {
        if arr[i] >= arr[i+1] {return false}
    }
    true
}
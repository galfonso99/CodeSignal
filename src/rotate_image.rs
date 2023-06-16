fn solution(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut tmp:i32;
    let n = a.len();
    for i in 0..n/2 {
        for j in i..n-i-1 {
            tmp = a[i][j];
            a[i][j] = a[n-j-1][i];
            a[n-j-1][i] = a[n-i-1][n-j-1];
            a[n-i-1][n-j-1] = a[j][n-i-1];
            a[j][n-i-1] = tmp;
        }
    }
    a
}
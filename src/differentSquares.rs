/* Given a rectangular matrix containing only digits, calculate the number of different 2 × 2 squares in it.

Example

For

matrix = [[1, 2, 1],
          [2, 2, 2],
          [2, 2, 2],
          [1, 2, 3],
          [2, 2, 1]]
the output should be
differentSquares(matrix) = 6. */

use std::collections::HashSet;
fn differentSquares(m: Vec<Vec<i32>>) -> i32 {
    let mut arr :Vec<Vec<i32>> = Vec::new();
    for i in 0..m.len()-1 {
        for j in 0..m[0].len()-1 {
            arr.push(vec![m[i][j], m[i][j+1], m[i+1][j], m[i+1][j+1]]);
        }
    }
    arr.iter().collect::<HashSet<_>>().iter().count() as i32
}
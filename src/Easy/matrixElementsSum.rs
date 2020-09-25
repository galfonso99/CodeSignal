/* After becoming famous, the CodeBots decided to move into a new building together. Each of the rooms has a different cost, and some of them are free, but there's a rumour that all the free rooms are haunted! Since the CodeBots are quite superstitious, they refuse to stay in any of the free rooms, or any of the rooms below any of the free rooms.

Given matrix, a rectangular matrix of integers, where each value represents the cost of the room, your task is to return the total sum of all rooms that are suitable for the CodeBots (ie: add up all the values that don't appear below a 0). 
Example

For matrix = [[0, 1, 1, 2], 
              [0, 5, 0, 0], 
              [2, 0, 3, 3]]
the output should be
matrixElementsSum(matrix) = 9. */

fn matrixElementsSum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for i in 0..matrix[0].len() {
        for j in 0..matrix.len() {
          if matrix[j][i] == 0 {break;}
          total += matrix[j][i]
        }
    }
    total
}

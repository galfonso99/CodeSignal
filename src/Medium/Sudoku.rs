/* Sudoku is a number-placement puzzle. The objective is to fill a 9 × 9 grid with digits so that each column, each row, and each of the nine 3 × 3 sub-grids that compose the grid contains all of the digits from 1 to 9.

This algorithm should check if the given grid of numbers represents a correct solution to Sudoku.

Example For
grid = [[1, 3, 2, 5, 4, 6, 9, 8, 7],
        [4, 6, 5, 8, 7, 9, 3, 2, 1],
        [7, 9, 8, 2, 1, 3, 6, 5, 4],
        [9, 2, 1, 4, 3, 5, 8, 7, 6],
        [3, 5, 4, 7, 6, 8, 2, 1, 9],
        [6, 8, 7, 1, 9, 2, 5, 4, 3],
        [5, 7, 6, 9, 8, 1, 4, 3, 2],
        [2, 4, 3, 6, 5, 7, 1, 9, 8],
        [8, 1, 9, 3, 2, 4, 7, 6, 5]] 
the output should be
sudoku(grid) = true;*/

//I purposely solved this is one line
use std::collections::HashSet;
fn sudoku(grid: Vec<Vec<i32>>) -> bool {
    grid.iter().all(|r| isUnique(r.to_vec())) &&
    (0..9).map(|i| (0..9).map(|j| grid[j][i]).collect::<Vec<i32>>()).all(|c| isUnique(c.to_vec())) &&
    (0..3).flat_map(|j| (0..3).map(|i| (0..3).flat_map(|y| (0..3).map(|x| grid[y+3*j][x+3*i])
    .collect::<Vec<i32>>()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>()).all(|g| isUnique(g.to_vec()))
}
fn isUnique(arr:Vec<i32>) -> bool {
    arr.len() == arr.into_iter().collect::<HashSet<i32>>().len()
}
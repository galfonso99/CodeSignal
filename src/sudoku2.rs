use std::collections::HashSet;
fn solution(grid: Vec<Vec<char>>) -> bool {
    grid.iter().all(|r| isUnique(r.to_vec())) &&
    (0..9).map(|i| (0..9).map(|j| grid[j][i]).collect::<Vec<char>>()).all(|c| isUnique(c.to_vec())) &&
    (0..3).flat_map(|j| (0..3).map(|i| (0..3).flat_map(|y| (0..3).map(|x| grid[y+3*j][x+3*i])
    .collect::<Vec<char>>()).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()).all(|g| isUnique(g.to_vec()))
}

fn isUnique(arr:Vec<char>) -> bool {
    arr.iter().filter(|&c| *c!='.').count() == arr.into_iter().collect::<HashSet<char>>().len() - 1
}
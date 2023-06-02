/* Given two cells on the standard chess board, determine whether they have the same color or not.

Example

For cell1 = "A1" and cell2 = "C3", the output should be
chessBoardCellColor(cell1, cell2) = true */

fn chessBoardCellColor(cell1: String, cell2: String) -> bool {
    let a = cell1.chars().collect::<Vec<char>>();
    let b = cell2.chars().collect::<Vec<char>>();
    (0..2).map(|i| a[i] as u8 - b[i] as u8).sum::<u8>() % 2 ==0
}

/* In the popular Minesweeper game you have a board with some mines and those cells that don't contain a mine have a number in it that indicates the total number of mines in the neighboring cells. Starting off with some arrangement of mines we want to create a Minesweeper game setup. */

fn minesweeper(mines: Vec<Vec<bool>>) -> Vec<Vec<i32>> {
    let (h, w) = (mines.len(), mines[0].len());
    let mut nums = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if mines[i][j] {
                let u = if i == 0 { 0 } else { i - 1 };
                let d = if i == h - 1 { i } else { i + 1 };
                let l = if j == 0 { 0 } else { j - 1 };
                let r = if j == w - 1 { j } else { j + 1 };
                for k in u..=d {
                    for l in l..=r {
                        nums[k][l] += 1;
                    }
                }
                nums[i][j] -= 1;
            }
        }
    }
    nums
}
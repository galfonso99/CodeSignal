fn solution(mut grid: Vec<Vec<i32>>, path: String) -> Vec<Vec<i32>> {
    // loop through path (map chars) and for each char match it to a direction and its corresponding function

    for dir in path.chars() {
        match dir {
            'R' => move_right(&mut grid),
            'L' => move_left(&mut grid),
            'U' => move_up(&mut grid),
            'D' => move_down(&mut grid),
            _ => (),
        }
    }
    grid
}

fn move_right(arr: &mut [Vec<i32>]) {
    for row in arr.into_iter() {
        row.reverse();
        let mut merged = false;
        let mut i = 0;
        for _ in 0..row.len() {
            let square = row[i];
            if square == 0 {
                row.remove(i);
                row.push(0);
            }
            else if i > 0 && square == row[i - 1] && !merged  {
                row[i - 1] = square * 2;
                row.remove(i);
                row.push(0);
                merged = true;
            } else {
                i += 1;
                merged = false;
            }
        }
        row.reverse();
    }
}

fn move_left(arr: &mut [Vec<i32>]) {
    for row in arr.iter_mut() {
        let mut merged = false;
        let mut i = 0;
        // let mut row = &mut arr[idx];
        for _ in 0..row.len() {
            let square = row[i];
            if square == 0 {
                row.remove(i);
                row.push(0);
            }
            else if i > 0 && square == row[i - 1] && !merged  {
                row[i - 1] = square * 2;
                row.remove(i);
                row.push(0);
                merged = true;
            } else {
                i += 1;
                merged = false;
            }
        }
    }
}

fn move_up(arr: &mut [Vec<i32>]) {
    for x in 0..arr[0].len() {
        let mut merged = false;
        let mut y = 0;

        for _ in 0..arr.len() {
            let square = arr[y][x];
            if square == 0 {
                shift(arr, x, y);
            }
            else if y > 0 && square == arr[y - 1][x] && !merged  {
                arr[y - 1][x] = square * 2;
                merged = true;
                shift(arr, x, y);
            }
            else {
                y += 1;
                merged = false;
            }
        }
    }
}

fn move_down(arr: &mut [Vec<i32>]) {
    arr.reverse();
    for x in 0..arr[0].len() {
        let mut merged = false;
        let mut y = 0;

        for _ in 0..arr.len() {
            let square = arr[y][x];
            if square == 0 {
                shift(arr, x, y);
            }
            else if y > 0 && square == arr[y - 1][x] && !merged  {
                arr[y - 1][x] = square * 2;
                merged = true;
                shift(arr, x, y);
            }
            else {
                y += 1;
                merged = false;
            }
        }
    }
    arr.reverse();
}

fn shift(arr: &mut [Vec<i32>], column: usize, row: usize) {
    for i in row..arr.len()-1 {
        arr[i][column] = arr[i+1][column];
    }
    arr[arr.len()-1][column] = 0;
}
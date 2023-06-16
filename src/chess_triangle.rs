pub fn solution(n: i32, m: i32) -> i32 {
    if n + m < 5 || n < 2 || m < 2 {
        return 0;
    }
    let mut total = 0;
    for i in 0..m as i32 {
        for j in 0..n as i32 {
            total += check_square((i, j), (m as i32, n as i32));
        }
    }
    total
}

fn check_square(pos: (i32, i32), board: (i32, i32)) -> i32 {
    let mut count_for_square = 0;
    let knight_pos = pos;
    for kn_move in get_knight_moves(pos, board) {
        for bs_move in get_bishop_moves(kn_move, knight_pos, board) {
            let rook_pos = bs_move;
            if rook_takes(rook_pos, knight_pos) {
                count_for_square += 1;
            }
        }
        for rk_move in get_rook_moves(kn_move, knight_pos, board) {
            let bishop_pos = rk_move;
            if bishop_takes(bishop_pos, knight_pos) {
                count_for_square += 1;
            }
        }
    }
    count_for_square
}

fn get_knight_moves(pos: (i32, i32), board: (i32, i32)) -> Vec<(i32, i32)> {
    let mut moves: Vec<(i32, i32)> = Vec::new();
    let hor = vec![2, 1, -1, -2, -2, -1, 1, 2];
    let vert = vec![1, 2, 2, 1, -1, -2, -2, -1];

    for i in 0..8 {
        let x = pos.0 as i32 + hor[i];
        let y = pos.1 as i32 + vert[i];
        // count valid moves
        if (x >= 0 && y >= 0 && x < board.0 && y < board.1) {
            moves.push((x as i32, y as i32));
        }
    }
    moves
}

fn get_rook_moves(pos: (i32, i32), knight: (i32, i32), board: (i32, i32)) -> Vec<(i32, i32)> {
    let mut moves: Vec<(i32, i32)> = Vec::new();
    for i in knight.0 - 3..=knight.0 + 3 {
        if i == pos.0 {
            continue;
        }
        if fits_in_board((i, pos.1), board) {
            moves.push((i, pos.1));
        }
    }
    for j in knight.1 - 3..=knight.1 + 3 {
        if j == pos.1 {
            continue;
        }
        if fits_in_board((pos.0, j), board) {
            moves.push((pos.0, j));
        }
        
    }
    moves
}

fn get_bishop_moves(pos: (i32, i32), knight: (i32, i32), board: (i32, i32)) -> Vec<(i32, i32)> {
    let mut moves: Vec<(i32, i32)> = Vec::new();
    let mut push_move = |x: (i32, i32)| {
        if fits_in_board(x, board) && fits_significant_area(x, knight) {
            moves.push(x);
        };
    };
    
    for i in 1..=knight.0 + 3 {
        let up_left = (pos.0 - i, pos.1 - i);
        let up_right = (pos.0 + i, pos.1 - i);
        let down_right = (pos.0 + i, pos.1 + i);
        let down_left = (pos.0 - i, pos.1 + i);
        push_move(up_left);
        push_move(up_right);
        push_move(down_right);
        push_move(down_left);
        
    }
    moves
}

fn rook_takes(rook: (i32, i32), knight: (i32, i32)) -> bool {
    rook.0 == knight.0 || rook.1 == knight.1
}

fn bishop_takes(bishop: (i32, i32), knight: (i32, i32)) -> bool {
    bishop.0 - knight.0 == bishop.1 - knight.1 || bishop.0 - knight.0 == knight.1 - bishop.1
}

fn fits_in_board(pos: (i32, i32), board: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < board.0 && pos.1 >= 0 && pos.1 < board.1
}

// Significant area is the area with which the triangle can occur outside the triangle is impossible
fn fits_significant_area(pos: (i32, i32), knight_pos: (i32, i32)) -> bool {
    pos.0 >= knight_pos.0 - 3 && pos.0 <= knight_pos.0 + 3 && pos.1 >= knight_pos.1 - 3 && pos.1 <= knight_pos.1 + 3
}
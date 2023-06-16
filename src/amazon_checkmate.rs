pub fn solution(king: String, amazon: String) -> Vec<i32> {
    let mut on_check: Vec<Vec<Option<bool>>> = vec![vec! {Some(false); 8}; 8];
    let queen = get_position(amazon);
    let (y, x) = queen;
    let king_pos = get_position(king);
    let king = (king_pos.0 as usize, king_pos.1 as usize);

    knight_moves(&mut on_check, queen);
    rook_moves(&mut on_check, queen, king_pos);
    bishop_moves(&mut on_check, queen, king_pos);
    get_white_king_moves(&mut on_check, (king.0 as i32, king.1 as i32));
    // Set queen square to Unavailable aka None
    on_check[y as usize][x as usize] = None;
    // Set king square to Unavailable aka None
    on_check[king_pos.0 as usize][king_pos.1 as usize] = None;

    let (checkmate, check, stalemate, safe) = get_king_status(&on_check, queen, king_pos);

    [checkmate, check, stalemate, safe].to_vec()
}

fn get_position(mut s: String) -> (i32, i32) {
    (
        7 - (s.pop().unwrap() as u8 - 49) as i32,
        (s.pop().unwrap() as u8 - 97) as i32,
    )
}

fn knight_moves(mut arr: &mut [Vec<Option<bool>>], pos: (i32, i32)) {
    let hor = vec![2, 1, -1, -2, -2, -1, 1, 2];
    let vert = vec![1, 2, 2, 1, -1, -2, -2, -1];

    for i in 0..8 {
        let y = pos.0 as i32 + vert[i];
        let x = pos.1 as i32 + hor[i];
        // count valid moves
        if within_board(x, y) {
            arr[y as usize][x as usize] = Some(true);
        }
    }
}
fn rook_moves(mut arr: &mut [Vec<Option<bool>>], pos: (i32, i32), king: (i32, i32)) {
    for i in 0..8 {
        if !king_is_blocking((pos.0, i), pos, king) {
            arr[pos.0 as usize][i as usize] = Some(true);
        }
        if !king_is_blocking((i, pos.1), pos, king) {
            arr[i as usize][pos.1 as usize] = Some(true);
        }
    }
}

fn bishop_moves(mut arr: &mut [Vec<Option<bool>>], queen: (i32, i32), king: (i32, i32)) {
    for i in 1..8 {
        for dir in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
            let pos = (queen.0 + i * dir.0, queen.1 + i * dir.1);
            if within_board(pos.0, pos.1) && !king_is_blocking(pos, queen, king) {
                arr[pos.0 as usize][pos.1 as usize] = Some(true);
            }
        }
    }
}

fn safe_move(
    arr: &[Vec<Option<bool>>],
    pos: (i32, i32),
    queen: (i32, i32),
    king: (i32, i32),
) -> bool {
    (pos.0 - 1..=pos.0 + 1)
        .flat_map(|i| {
            (pos.1 - 1..=pos.1 + 1).flat_map(move |j| {
                if (i, j) == pos || !within_board(i, j) {
                    None
                } else if (i, j) == queen && !((i - king.0).abs() <= 1 && (j - king.1).abs() <= 1) {
                    Some(false)
                } else {
                    arr[i as usize][j as usize]
                }
            })
        })
        .any(|in_check| !in_check)
}

fn get_king_status(
    arr: &[Vec<Option<bool>>],
    queen: (i32, i32),
    king: (i32, i32),
) -> (i32, i32, i32, i32) {
    let (mut checkmate, mut check, mut stalemate, mut safe) = (0, 0, 0, 0);

    for i in 0..8 {
        for j in 0..8 {
            match arr[i as usize][j as usize] {
                Some(true) => {
                    if safe_move(arr, (i, j), queen, king) {
                        check += 1;
                    } else {
                        checkmate += 1;
                    }
                }
                Some(false) => {
                    if safe_move(arr, (i, j), queen, king) {
                        safe += 1;
                    } else {
                        stalemate += 1;
                    }
                }
                None => {}
            }
        }
    }
    (checkmate, check, stalemate, safe)
}

fn get_white_king_moves(mut arr: &mut [Vec<Option<bool>>], king: (i32, i32)) {
    for i in king.0 - 1..=king.0 + 1 {
        for j in king.1 - 1..=king.1 + 1 {
            if within_board(i, j) {
                // arr[i as usize][j as usize] = Some(true);
                arr[i as usize][j as usize] = None;
            }
        }
    }
}

fn within_board(x: i32, y: i32) -> bool {
    x >= 0 && x < 8 && y >= 0 && y < 8
}

fn king_is_blocking(pos: (i32, i32), queen: (i32, i32), king: (i32, i32)) -> bool {
    let (x, y) = pos;
    let (qx, qy) = queen;
    let (kx, ky) = king;

    x == qx && x == kx && ((qy < ky && ky < y) || (y < ky && ky < qy))
        || y == qy && y == ky && ((qx < kx && kx < x) || (x < kx && kx < qx))
        || (x - qx).abs() == (y - qy).abs()
            && (x - kx).abs() == (y - ky).abs()
            && ((qx < kx && kx < x) || (x < kx && kx < qx))
}


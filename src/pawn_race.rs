fn solution(white: String, black: String, toMove: char) -> String {
    let (wy, wx) = parse_position(white);
    let (mut by, bx) = parse_position(black);
    let white = "white".to_string();
    let black = "black".to_string();
    
    let (first, second) = {
        match toMove {
            'w' => ("white".to_string(), "black".to_string()),
            'b' => ("black".to_string(), "white".to_string()),
            _ => ("error".to_string(), "error".to_string())
        }
    };

    let fastest_pawn = || {
        let w_delta = if wy == 1 {7 - wy - 1} else {7 - wy};
        let b_delta = if by == 6 {by - 1} else {by};
        if b_delta - w_delta <= -1 {
            return black.clone();
        }
        if b_delta - w_delta == 0 {
            return first.clone();
        }
        return white.clone();
    };
    
    if wx == bx {
        if by - wy > 0 {
            return "draw".to_string(); 
        } else {
            return fastest_pawn();
        }
    };
    if (wx-bx).abs() > 1 {
        return fastest_pawn();
    };

    if (wx-bx).abs() == 1 {
        if (by - wy) <= 0 {
            return fastest_pawn();
        } else if wy != 1 && by != 6 {
            if (by - wy) % 2 == 1 {
                return first;
            }
            if (by - wy) % 2 == 0 {
                return second;
            }
        } else if wy == 1 && by == 6 {
            return second;
        } else if by-wy == 1 {
            return first;
        } else if wy == 1 {
            if first == white && by == 3 {
                return black;
            } else if first == black && by == 4 {
                return black;
            } else {
                return white;
            }
        } else if by == 6 {
            if first == black && wy == 4 {
                return white;
            } else if first == white && wy == 3 {
                return white;
            } else {
                return black;
            }
        } 
    }
    first
}


fn parse_position(mut s: String) -> (i32, i32) {
    (
        (s.pop().unwrap() as u8 - 49) as i32,
        (s.pop().unwrap() as u8 - 97) as i32,
    )
}
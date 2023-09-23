#![allow(clippy::needless_return)]

fn main() {
    // let sol = solution();
    // dbg!(sol);
}

use std::fmt::Display;

fn solution(pieces: Vec<Vec<Vec<char>>>) -> i32 {
    let mut field = Field::new(20, 10);
    for (i, piece) in pieces.iter().enumerate() {
        handle_rotations(&piece, &mut field);
        let (row, column) = field.optimal_piece_pos;
        insert_optimal_piece(&mut field, i + 1, row, column);
        clear_lines(&mut field);
        field.optimal_piece = (vec![vec!['.']], 0);
    }
    field.lines as i32
}

fn handle_rotations(piece: &Piece, field: &mut Field) {
    let is_symmetrical = is_symmetrical(piece);
    let nr_of_rotations = if is_symmetrical { 1 } else { 3 }; //3 rotations bc 1st is given
    let mut rotations = vec![piece.to_vec()];
    handle_positions(piece, field);
    for _ in 0..nr_of_rotations {
        let prev_rotation = &rotations[rotations.len() - 1];
        let new_rotation = rotate_90_degrees(prev_rotation);
        rotations.push(new_rotation);
        let curr_rotation = &rotations[rotations.len() - 1];
        handle_positions(curr_rotation, field);
    }
}

fn handle_positions(piece: &Piece, field: &mut Field) {
    // Upper bound should be width - piece_width + 1
    let positions_count = field.dimensions.1 - piece[0].len() + 1;
    for i in 0..positions_count {
        handle_falling_piece(piece, field, i);
    }
}

fn handle_falling_piece(piece: &Piece, field: &mut Field, column: usize) {
    let layout = &field.positions;
    let piece_bottom = &piece[piece.len() - 1];
    let mut starting_row_index: usize = layout.len() - 1;
    let upper_column = column + piece[0].len();
    'outer: for i in piece.len()..layout.len() {
        for j in column..upper_column {
            let overlap = layout[i][j] != 0 && piece_bottom[j - column] == '#';
            if overlap {
                starting_row_index = i - 1;
                break 'outer;
            }
        }
    }
    let mut lowest_fitting_row = layout.len() - 1;
    'outer: for i in (piece.len() - 1..=starting_row_index).rev() {
        for (offset, k) in (0..piece.len()).rev().enumerate() {
            for j in column..upper_column {
                let collision = layout[i - offset][j] != 0 && piece[k][j - column] == '#';
                if collision {
                    continue 'outer;
                }
            }
        }
        lowest_fitting_row = i;
        break;
    }
    calculate_score(piece, field, lowest_fitting_row, column);
}

fn calculate_score(piece: &Piece, field: &mut Field, row: usize, column: usize) {
    let piece_length = piece.len();
    let top_row = row + 1 - piece_length;
    // Count row will count the blocks in the field and on the piece and add them
    let count_row = |piece_index: usize, field_index: usize| {
        let mut count = 0;
        let field_width = field.dimensions.1;
        for i in 0..field_width {
            let field_has_block: bool = field.positions[field_index][i] != 0;
            let piece_has_block = i >= column && i < column + piece[0].len() && piece[piece_index][i - column] == '#';
            if field_has_block || piece_has_block {
                count += 1;
            }
        }
        count
    };
    let mut total_count = 0;
    for (i, j) in (top_row..=row).enumerate() {
        total_count += count_row(i, j);
    }
    // If total_count is bigger than previous highest replace it in field.optimal
    if total_count > field.optimal_piece.1 {
        field.optimal_piece = (piece.to_owned(), total_count);
        field.optimal_piece_pos = (row, column);
    }
}

fn insert_optimal_piece(field: &mut Field, piece_nr: usize, row: usize, column: usize) {
    let optimal_piece = &field.optimal_piece.0;
    let piece_length = optimal_piece.len();
    let upper_column = column + optimal_piece[0].len();
    let top_row = row + 1 - piece_length;
    for i in top_row..=row {
        for j in column..upper_column {
            // let collision = layout[i][j] != 0 && optimal_piece[k][j - column] == '#';
            if optimal_piece[i - top_row][j - column] == '#' {
                field.positions[i][j] = piece_nr;
            }
        }
    }
}

fn clear_lines(field: &mut Field) {
    let mut line_count = 0;
    let mut lines = vec![];
    'outer: for i in (0..field.positions.len()).rev() {
        for j in 0..field.positions[0].len() {
            if field.positions[i][j] == 0 {
                continue 'outer;
            }
        }
        lines.push(i);
        line_count += 1;
    }
    if line_count == 0 {
        return;
    }
    field.lines += line_count;
    let y = line_count;
    let mut i = 0;
    let mut n = 0;
    while i < lines.len() {
        let mut h = 0;
        let mut j = 0;
        while i < lines.len() {
            j += 1;
            if h == lines.len() - 1 || lines[h] - 1 != lines[h + 1] {
                break;
            }
            h += 1;
        }
        let l = if lines[i] + n < j { 0 } else { lines[i] + n - j };
        for k in (n..=l).rev() {
            for m in 0..field.positions[0].len() {
                field.positions[k + j][m] = field.positions[k][m];
            }
        }
        i += 1;
        n += j;
        if n == y {
            break;
        }
    }
    for i in 0..y {
        for j in 0..field.positions[0].len() {
            field.positions[i][j] = 0;
        }
    }
}

fn rotate_90_degrees(piece: &Piece) -> Piece {
    let len = piece.len();
    let width = piece[0].len();
    let mut rotation = vec![vec!['_'; len]; width];
    for i in 0..len {
        for j in 0..width {
            let part = piece[i][j];
            let i_offset = len - i - 1;
            rotation[j][i_offset] = part;
        }
    }
    rotation
}

fn is_symmetrical(piece: &Piece) -> bool {
    let rotated_piece = rotate_90_degrees(&rotate_90_degrees(piece));
    if piece.len() != rotated_piece.len() || piece[0].len() != rotated_piece[0].len() {
        return false;
    }
    piece.eq(&rotated_piece)
}
struct Field {
    positions: Vec<Vec<usize>>,
    lines: usize,
    dimensions: (usize, usize),
    optimal_piece: (Vec<Vec<char>>, usize),
    optimal_piece_pos: (usize, usize),
}
impl Field {
    fn new(height: usize, width: usize) -> Self {
        Self {
            positions: vec![vec![0; width]; height],
            lines: 0,
            dimensions: (height, width),
            optimal_piece: (vec![vec![]], 0),
            optimal_piece_pos: (0, 0),
        }
    }
}
type Piece = Vec<Vec<char>>;

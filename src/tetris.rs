#![allow(clippy::needless_return)]

fn main() {
    let sol = solution(
        // vec![
        //     vec![vec!['#', '#', '#', '#']],
        //     vec![vec!['#', '#', '#', '#']],
        //     vec![vec!['#', '#'], vec!['#', '#']],
        // ],
        vec![
            vec![vec!['.', '#', '#'], vec!['#', '#', '.']],
            vec![vec!['.', '#', '.'], vec!['#', '#', '#']],
            vec![vec!['#', '#', '.'], vec!['.', '#', '#']],
            vec![vec!['.', '#', '.'], vec!['#', '#', '#']],
            vec![vec!['#', '#', '#', '#']],
            vec![vec!['#', '.', '.'], vec!['#', '#', '#']],
            vec![vec!['#', '#'], vec!['#', '#']],
            vec![vec!['#', '#', '#'], vec!['.', '.', '#']],
            vec![vec!['.', '#', '#'], vec!['#', '#', '.']],
            vec![vec!['.', '#', '.'], vec!['#', '#', '#']],
            vec![vec!['#', '#', '.'], vec!['.', '#', '#']],
            vec![vec!['.', '#', '.'], vec!['#', '#', '#']],
            vec![vec!['#', '#', '#', '#']],
            vec![vec!['#', '.', '.'], vec!['#', '#', '#']],
            vec![vec!['#', '#'], vec!['#', '#']],
            vec![vec!['#', '#', '#'], vec!['.', '.', '#']],
        ],
    );
    dbg!(sol);
}

use std::fmt::Display;

fn solution(pieces: Vec<Vec<Vec<char>>>) -> i32 {
    let mut field = Field::new(20, 10);
    for (i, piece) in pieces.iter().enumerate() {
        // let piece: Piece = Piece::from(piece);
        handle_rotations(&piece, &mut field);
        // After the optimal_piece is found insert it into the field
        let (row, column) = field.optimal_piece_pos;
        insert_optimal_piece(&mut field, i + 1, row, column);
        // println!("row {}, column {}, score {}", row, column, field.optimal_piece.1);
        // pretty_print(&field.optimal_piece.0);
        // Here you should check for clompleted lines and remove them and shift the blocks down
        clear_lines(&mut field);
        // Restart optimal Piece
        field.optimal_piece = (vec![vec!['.']], 0);
        pretty_print(&piece, 0);
        println!("");
        pretty_print(&field.positions, 13);
        println!("Lines: {}", field.lines);
        print!("\n");
        // TODO: Debugging
        // println!("Piece nr {} is symmetrical {}", i, is_symmetrical(piece));
        // dbg!();
    }
    field.lines as i32
}

fn handle_rotations(piece: &Piece, field: &mut Field) {
    let is_symmetrical = is_symmetrical(piece);
    let mut curr_piece;
    let nr_of_rotations = if is_symmetrical { 1 } else { 3 }; //3 rotations bc 1st is given
                                                              // First piece position doesnt need to be rotated
    handle_positions(piece, field);
    for _ in 0..nr_of_rotations {
        curr_piece = rotate_90_degrees(piece);
        handle_positions(&curr_piece, field);
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
    /*  Find the starting row starting from top until collision and go one above
     *  From the starting row start testing the rows of the piece
     *  If conflict change the starting row to one above the previous and start the test all over
     *  Once you found the right row for the piece, calculate the blocks of the rows and save it if
     *  its higher than the previous highest
     *  Once you're done with all the positions the one with the highest value gets inserted to the
     *  field
     */

    let layout = &field.positions;
    let piece_bottom = &piece[piece.len() - 1];
    let mut starting_row_index: usize = layout.len() - 1;
    let upper_column = column + piece[0].len();
    for i in piece.len()..layout.len() {
        for j in column..upper_column {
            // TODO: Keep in mind that I have to test for hitting the top of the field
            let overlap = layout[i][j] != 0 && piece_bottom[j - column] == '#';
            let overflow_field = i == 0;
            if overlap && !overflow_field {
                starting_row_index = i - 1;
                break;
            }
        }
    }
    // let starting_row = &layout[starting_row_index];
    // TODO: Here you also have to check that the piece doesnt overflow the top of the field

    // Find the row for which the whole piece actually fits (lowest_fitting_row)
    let mut lowest_fitting_row = layout.len() - 1;
    'outer: for i in (piece.len() - 1..=starting_row_index).rev() {
        // let condition = piece.eq(&vec![vec!['#', '#'], vec!['#', '.'], vec!['#', '.']]) && column == 3;

        // if condition {
        //     print!("{} \n", i);
        // }
        for (offset, k) in (0..piece.len()).rev().enumerate() {
            for j in column..upper_column {
                let collision = layout[i - offset][j] != 0 && piece[k][j - column] == '#';
                // if condition {println!("At field [{}][{}] and piece[{}][{}] it is: {}", i-offset,j, k, j - column, collision);}
                if collision {
                    continue 'outer;
                }
            }
        }
        lowest_fitting_row = i;
        break;
    }
    // if piece.eq(&vec![vec!['#', '#'], vec!['#', '.'], vec!['#', '.']]) && column == 3 {
    //     // pretty_print(piece);
    //     println!("Starting row {}, lowest fitting {}, column {}", starting_row_index, lowest_fitting_row, column);
    // }
    // println!("row {}", lowest_fitting_row);
    // Calculate the effective row score for the given piece and store it if higher than current
    calculate_score(piece, field, lowest_fitting_row, column);
}

fn calculate_score(piece: &Piece, field: &mut Field, row: usize, column: usize) {
    let piece_length = piece.len();
    let top_row = row + 1 - piece_length;
    // Count row will count the blocks in the field and on the piece and add them
    let count_row = |piece_index: usize, field_index: usize| {
        // TODO: Well actually given the index you need to subtract the top row from the index so
        // that the initial index of the piece is zero
        //  from 7 - 9 len is 3 7 - 7 = 0    9 - 7 = 2
        let mut count = 0;
        // let piece_index = piece_index - top_row;
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
    // TODO: Figure out this range its a bit weird
    for (i, j) in (top_row..=row).enumerate() {
        total_count += count_row(i, j);
    }
    // If total_count is bigger than previous highest replace it in field.optimal
    if total_count > field.optimal_piece.1 {
        field.optimal_piece = (piece.to_owned(), total_count);
        field.optimal_piece_pos = (row, column);
    }
    // if piece.eq(&vec![vec!['#'], vec!['#'], vec!['#'], vec!['#']]) && column == 0 {
    //     println!("Score {}, row {}, column {}", total_count, row, column);
    //     pretty_print(piece);
    //     println!("Optimal Count {}", field.optimal_piece.1);
    // }
}

fn insert_optimal_piece(field: &mut Field, piece_nr: usize, row: usize, column: usize) {
    let optimal_piece = &field.optimal_piece.0;
    let piece_length = optimal_piece.len();
    let upper_column = column + optimal_piece[0].len();
    let top_row = row + 1 - piece_length;
    // let layout = &field.positions;
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
    // I need to keep track of which lines get
    // completed to know which ones to remove and then after add the zeroes at the top
    // Keep an array [usize; 20] of completed lines and then for each index
    //
    // Forget about the SHIT above just store an array of indices of completed lines
    // Then process the first comp line from the bottom (meaning highest value) and continue as
    // long as the numbers are in sequence then break process the next line and continue as long as
    // sequenced
    // TODO: Keep in mind that yuu mind need to make some individual pieces fall as far as they can
    // beyond just the amount of deleted lines
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
    // dbg!(field.lines, lines);
    field.lines += line_count;
    let y = line_count;
    let mut i = 0;
    let mut j = 0;
    let mut n = 0;
    // TODO: This logic going on here is completely screwed up in case of bug check these nested loops
    // for logical bugs
    while i < lines.len() {
        while i < lines.len() {
            j += 1;
            if i == lines.len() - 1 || lines[i] - 1 != lines[i + 1] {
                break;
            }
            i += 1;
        }
        // let x = i + 1 - field.positions.len();
        // let line_index = lines[i];
        let x = i + 1 - j;
        let l = if lines[x] + n < j { 0 } else { lines[x] + n - j };
        for k in (n..=l).rev() {
            for m in 0..field.positions[0].len() {
                field.positions[k + j][m] = field.positions[k][m];
            }
        }
        i += 1;
        n += j;
    }
    for i in 0..y {
        for j in 0..field.positions[0].len() {
            field.positions[i][j] = 0;
        }
    }
    // for i in 0..line_indices.len() {}
    // for i in (y..field.positions.len()).rev() {
    //     for j in 0..field.positions[0].len() {
    //         field.positions[i][j] = field.positions[i - y][j];
    //     }
    // }
}

fn rotate_90_degrees(piece: &Piece) -> Piece {
    // let shape = piece.shape;
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

fn pretty_print<T: Display>(piece: &Vec<Vec<T>>, start: usize) {
    for i in start..piece.len() {
        for j in 0..piece[i].len() {
            print!("{:3}", piece[i][j]);
        }
        print!("\n");
    }
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

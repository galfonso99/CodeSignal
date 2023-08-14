#![allow(clippy::needless_return)]

// mod adjacent_product;
// mod befunge_93;
// mod palindrome;
// mod pipes_game;
// mod shape_area;

fn main() {
    solution(vec![vec![vec!['c']]]);
}

fn solution(pieces: Vec<Vec<Vec<char>>>) -> i32 {
    // Handle pieces this time inside of solution fn
    // Meaning loop through the pieces and apply the necessary Functions
    let mut field = Field::new(20, 10);
    for piece in pieces {
        let piece: Piece = Piece::from(piece);
        handle_rotations(&piece, &field);
    }
    3
}

fn handle_rotations(piece: &Piece, field: &Field) {
    let mut is_symmetrical = is_symmetrical(piece);
    let mut curr_piece = piece.to_vec();
    // Optinal stores the number os blocks in the rows of the piece and the Piece itself
    let mut optimal = (vec![vec!['c']; 0], 0);
    let nr_of_rotations = if is_symmetrical {2} else {4};
    for i in 0..nr_of_rotations {
        handle_positions(piece, field);
        curr_piece = rotate_90_degrees(&curr_piece);
    }

}

fn handle_positions(piece: &Piece, field: &Field) {
    // Upper bound should be width - piece_width + 1
    for i in 0..piece[0].len() {
        handle_falling_piece()
    }
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

fn is_symmetrical (piece: &Piece) -> bool {
    let rotated_piece = rotate_90_degrees( &rotate_90_degrees(piece) );
    if piece.len() != rotated_piece.len() || piece[0].len() != rotated_piece[0].len() {
        return false; 
    }
    // compare the two
    // for i in 0..piece.len() {
    //     for j in 0..piece[0].len() {
    //         if piece[i][j] != rotated_piece[i][j] {
    //             return false;
    //         }
    //     }
    // }
    // true
    piece.eq(&rotated_piece)
}

fn calculate_score(piece: &Piece, field: &Field) -> u32 {
    8
}

struct Field {
    positions: Vec<Vec<u32>>,
    lines: u32,
    dimensions: (u32, u32),
}

impl Field {
    fn new(height: u32, width: u32) -> Self {
        Self {
            positions: vec![vec![]],
            lines: 0,
            dimensions: (height, width),
        }
    }
}

type Piece = Vec<Vec<char>>;

// struct Piece {
//     shape: Vec<Vec<char>>,
//     symmetrical: bool,
//
// }
//
// impl From<Vec<Vec<char>>> for Piece {
//     fn from(shape: Vec<Vec<char>>) -> Self {
//         Self { shape, symmetrical: false }
//
//     }
// }

/*
Let's play Tetris! But first we need to define the rules, especially since they probably differ from the way you've played Tetris before.

There is an empty field with 20 rows and 10 columns, which is initially empty. Random pieces appear on the field, each composed of four square blocks. You can't change the piece's shape, but you can rotate it 90 degree clockwise (possibly several times) and choose which columns it will appear within. Once you've rotated the piece and have set its starting position, it appears at the topmost row where you placed it and falls down until it can't fall any further. The objective of the game is to create horizontal lines composed of 10 blocks. When such a line is created, it disappears, and all lines above the deleted one move down. The player receives 1 point for each deleted row.

Your task is to implement an algorithm that places each new piece optimally. The piece is considered to be placed optimally if:

    The total number of blocks in the rows this piece will occupy after falling down is maximized;
    Among all positions with that value maximized, this position requires the least number of rotations;
    Among all positions that require the minimum number of rotations, this one is the leftmost one (i.e. the leftmost block's position is as far to the left as possible).

The piece can't leave the field. It is guaranteed that it is always possible to place the Tetris piece in the field.
*/

// Functions
// rotate the pieces 90 degrees
// calculate the scores for all the possible positions for a giuen piece for a given rotation (the
// Actually the way the scores work is that whatever the rows this piece occupies after falling
// down will be maximized in terms of all of the blocks meaning it is closest to being filled
// Prioritizing the lower rows first
// Keep a record of the highest score along with the placement and the calculate all the
// positions
// and rotations
// Make sure to start the calculations from the left and with the least amount of rotations
// Keep a field object that keep tracks of the positions and the the current score (rows deleted)
// Some of the pieces mighr be symmetrical so only require 2 rotations
// THIS IS ACTUALLY WRONG I THINK  scores are based on how many horizontal blocks are covered  (rows) in horizontal order rather
// than vertical order

#![allow(clippy::needless_return)]

mod adjacent_product;
mod befunge_93;
mod palindrome;
mod pipes_game;
mod shape_area;

fn main() {
    //palindrome::run();
    //adjacent_product::run();
    //shape_area::run();
}

fn solution(pieces: Vec<Vec<Vec<char>>>) -> i32 {
    // Do something here
    

}

// functions
// rotate the pieces 90 degrees
// calculate the scores for all the possible positions for a giuen piece for a given rotation (the
// scores are based on how many horizontal blocks are covered  (rows) in horizontal order rather
// than vertical order )
// Actually the way the scores work is that whatever the rows this piece occupies after falling
// down will be maximized in terms of all of the blocks meaning it is closest to being filled
// Prioritizing the lower rows first
// Keep a record of the highest score along with the placement and the calculate all the positions
// and rotations
// Make sure to start the calculations from the left and with the least amount of rotations
// Keep a field object that keep tracks of the positions and the the current score (rows deleted)
//

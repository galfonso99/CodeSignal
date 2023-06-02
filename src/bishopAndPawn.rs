/* Given the positions of a white bishop and a black pawn on the standard chess board, determine whether the bishop can capture the pawn in one move.

The bishop has no restrictions in distance for each move, but is limited to diagonal movement. Check out the example below to see how it can move: */

fn bishopAndPawn(b: String, p: String) -> bool {
    (b.as_bytes()[0] as i32 - p.as_bytes()[0] as i32).abs() 
    == (b.as_bytes()[1] as i32 - p.as_bytes()[1] as i32).abs()
}
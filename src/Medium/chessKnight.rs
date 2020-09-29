/* Given a position of a knight on the standard chessboard, find the number of different moves the knight can perform.

The knight can move to a square that is two squares horizontally and one square vertically, or two squares vertically and one square horizontally away from it. The complete move therefore looks like the letter L. Check out the image below to see all valid moves for a knight piece that is placed on one of the central squares. */

fn chessKnightc(mut cell: String) -> i32 {
    let (mut y, mut x) = (cell.pop().unwrap().to_digit(10).unwrap() as i32, (cell.pop().unwrap() as u8 - 96) as i32 );
    let mut cnt = 0;
    if y+2 <=8 {
        if x-1 > 0 {cnt +=1;}
        if x+1 <9 {cnt +=1;}
    }
    if x+2 <9 {
        if y-1 > 0 {cnt +=1;}
        if y+1 <9 {cnt +=1;}
    }
    if y-2 >0 {
        if x-1 > 0 {cnt +=1;}
        if x+1 <9 {cnt +=1;}
    }
    if x-2 >0 {
        if y-1 > 0 {cnt +=1;}
        if y+1 <9 {cnt +=1;}
    }
    cnt
}
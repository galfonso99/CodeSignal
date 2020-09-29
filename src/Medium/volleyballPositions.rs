/* You are watching a volleyball tournament, but you missed the beginning of the very first game of your favorite team. Now you're curious about how the coach arranged the players on the field at the start of the game.

The team you favor plays in the following formation:

0 3 0
4 0 2
0 6 0
5 0 1
where positive numbers represent positions occupied by players. After the team gains the serve, its members rotate one position in a clockwise direction, so the player in position 2 moves to position 1, the player in position 3 moves to position 2, and so on, with the player in position 1 moving to position 6.

Here's how the players change their positions:



Given the current formation of the team and the number of times k it gained the serve, find the initial position of each player in it. */

fn volleyballPositions(mut f: Vec<Vec<String>>, k: i32) -> Vec<Vec<String>> {
    let mut players = vec![&f[0][1], &f[1][2], &f[3][2], &f[2][1], &f[3][0], &f[1][0]];   //take players
    players.rotate_left(k as usize %6);    //Rotate players
    let mut start = f.clone();      //Mutable clone
    start[0][1] = players[0].to_owned();       //Position players correctly
    start[1][2] = players[1].to_owned();       //Without looping
    start[3][2] = players[2].to_owned();
    start[2][1] = players[3].to_owned();
    start[3][0] = players[4].to_owned();
    start[1][0] = players[5].to_owned();
    start
}
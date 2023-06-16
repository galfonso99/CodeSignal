fn solution(mut f: Vec<Vec<String>>, k: i32) -> Vec<Vec<String>> {
    let mut players = vec![&f[0][1], &f[1][2], &f[3][2], &f[2][1], &f[3][0], &f[1][0]];   //take players
    players.rotate_left(k as usize %6);    //Rotate players
    let mut start = f.clone();      //Mutable clone
    start[0][1] = players[0].to_string();       //Position players correctly
    start[1][2] = players[1].to_owned();       //Without looping
    start[3][2] = players[2].to_owned();
    start[2][1] = players[3].to_owned();
    start[3][0] = players[4].to_owned();
    start[1][0] = players[5].to_owned();
    start
}
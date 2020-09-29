/* Elections are in progress!

Given an array of the numbers of votes given to each of the candidates so far, and an integer k equal to the number of voters who haven't cast their vote yet, find the number of candidates who still have a chance to win the election.

The winner of the election must secure strictly more votes than any other candidate. If two or more candidates receive the same (maximum) number of votes, assume there is no winner at all. */

fn electionsWinners(v: Vec<i32>, k: i32) -> i32 {
    let max = v.iter().max().unwrap();
    if k==0 && v.iter().filter(|&x| x == max).count() < 2 {1}
    else {v.iter().filter(|&x| x+k > *max).count() as i32}
}
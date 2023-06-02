/* Given a sequence of integers as an array, determine whether it is possible to obtain a strictly increasing sequence by removing no more than one element from the array.

Note: sequence a0, a1, ..., an is considered to be a strictly increasing if a0 < a1 < ... < an. Sequence containing only one element is also considered to be strictly increasing. */

fn almostIncreasingSequence(sequence: Vec<i32>) -> bool {
    let c2 = sequence.windows(2).filter(|v| v[0] >= v[1]).count();
    let c3 = sequence.windows(3).filter(|v| v[0] >= v[2]).count();
    c2 <= 1 && c3 <= 1
}
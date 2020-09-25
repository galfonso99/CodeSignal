/* Given a string, find out if its characters can be rearranged to form a palindrome.

Example

For inputString = "aabb", the output should be
palindromeRearranging(inputString) = true.

We can rearrange "aabb" to make "abba", which is a palindrome */

use std::collections::HashSet;
use std::iter::FromIterator;
fn palindromeRearranging(s: String) -> bool {
    let set:HashSet<char> = HashSet::from_iter(s.chars());
    set.iter().map(|x| s.chars().filter(|c| c==x).count() % 2).sum::<usize>() <= 1
}

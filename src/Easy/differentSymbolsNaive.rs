/* Given a string, find the number of different characters in it.

Example

For s = "cabca", the output should be
differentSymbolsNaive(s) = 3.

There are 3 different characters a, b and c. */

use std::collections::HashSet;
fn differentSymbolsNaive(s: String) -> i32 {
    s.chars().collect::<HashSet<_>>().len() as i32
}
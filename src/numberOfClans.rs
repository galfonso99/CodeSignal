/* Let's call two integers A and B friends if each integer from the array divisors is either a divisor of both A and B or neither A nor B. If two integers are friends, they are said to be in the same clan. How many clans are the integers from 1 to k, inclusive, broken into? */

use std::collections::HashSet;
fn numberOfClans(div: Vec<i32>, k: i32) -> i32 {
    let mut h = HashSet::new();
    for x in 1..=k {
        let mut a :Vec<i32>= Vec::new();
        for i in div.iter() {
            if x % i == 0 {a.push(1);}
            else {a.push(0);}
        }
        h.insert(a);
    }
    h.len() as i32
}
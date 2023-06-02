/* Miss X has only two combs in her possession, both of which are old and miss a tooth or two. She also has many purses of different length, in which she carries the combs. The only way they fit is horizontally and without overlapping. Given teeth' positions on both combs, find the minimum length of the purse she needs to take them with her.

It is guaranteed that there is at least one tooth at each end of the comb.
It is also guaranteed that the total length of two strings is smaller than 32.
Note, that the combs can not be rotated/reversed. */

fn combs(c1: String, c2: String) -> i32 {
    let min = if c1.len() <= c2.len() {c1.clone()} else {c2.clone()};
    let max = if c1.len() > c2.len() {c1.clone()} else {c2.clone()};
    let mut right = 0;
    let mut left = 0;
    for x in 1..=max.len() {
        let mut fits = true;
        let mut mx = max.chars().skip(x);
        let mut mn = min.chars();
        for n in 0..max.len() - x {
            let a = mx.next();
            let b = mn.next();
            fits = a!=Some('*') || b!=Some('*');
            if !fits {break;}
        }
        if fits {
            right = if x+min.len() < max.len() {max.len()} else {x + min.len()};
            break;
        }
    }
    for x in 1..=min.len() {
        let mut fits = true;
        let mut mx = max.chars();
        let mut mn = min.chars().skip(x);
        for n in 0..min.len() - x {
            let a = mx.next();
            let b = mn.next();
            fits = a!=Some('*') || b!=Some('*');
            if !fits {break;}
        }
        if fits {
            left = x + max.len();
            break;
        }
    }
    left.min(right) as i32
}

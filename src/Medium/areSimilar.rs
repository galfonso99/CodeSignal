/* Two arrays are called similar if one can be obtained from another by swapping at most one pair of elements in one of the arrays.

Given two arrays a and b, check whether they are similar. */

fn areSimilar(a: Vec<i32>, b: Vec<i32>) -> bool {
    let max = std::cmp::min(a.len(),b.len());
    let mut stack = Vec::new();
    for i in 0..max {
        if a[i] != b[i] {
            stack.push(i);
        }
    }
    if stack.len() == 0 {return true;} else if stack.len() != 2 {return false;}
    let j = stack.pop().unwrap();
    let i = stack.pop().unwrap();
    (a[i] == b[j] && b[i] == a[j])
}

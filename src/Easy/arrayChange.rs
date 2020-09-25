/* You are given an array of integers. On each move you are allowed to increase exactly one of its element by one. Find the minimal number of moves required to obtain a strictly increasing sequence from the input.

Example

For inputArray = [1, 1, 1], the output should be
arrayChange(inputArray) = 3. */

fn arrayChange(a: Vec<i32>) -> i32 {
    let (mut inc, mut cnt) = (0,0);
    for i in 0..a.len()-1 {
        if a[i+1] <= a[i]+inc {
            inc += a[i]+1-a[i+1];
            cnt += inc;
        }
        else {
            inc = 0;
        }
    }
    cnt
}

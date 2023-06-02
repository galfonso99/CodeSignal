/* Find the number of ways to express n as sum of some (at least two) consecutive positive integers.

Example

For n = 9, the output should be
isSumOfConsecutive2(n) = 2. 
There are two ways to represent n = 9: 2 + 3 + 4 = 9 and 4 + 5 = 9.*/

fn isSumOfConsecutive2(n: i32) -> i32 {
    let mut cnt = 0;
    for i in 1..n {
        let mut acc = 0;
        for j in i..n {
            acc+=j;
            if acc >= n {
                if acc == n {cnt+=1;}
                break;
            }
        }
    }
    cnt
}
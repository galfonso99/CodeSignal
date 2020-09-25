/* A string is said to be beautiful if each letter in the string appears at most as many times as the previous letter in the alphabet within the string; ie: b occurs no more times than a; c occurs no more times than b; etc.

Given a string, check whether it is beautiful.

Example

For inputString = "bbbaacdafe", the output should be isBeautifulString(inputString) = true. */

fn isBeautifulString(inp: String) -> bool {
    let mut occurs = vec![0; 26];
    inp.bytes().for_each(|b| occurs[(b-b'a') as usize] += 1);
    occurs.windows(2).all(|x| x[0] >= x[1])
}
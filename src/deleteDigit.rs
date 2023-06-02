/* Given some integer, find the maximal number you can obtain by deleting exactly one digit of the given number.

Example

For n = 152, the output should be
deleteDigit(n) = 52;
For n = 1001, the output should be
deleteDigit(n) = 101. */

fn deleteDigit(n: i32) -> i32 {
    let str = n.to_string();
    (0..str.len()).map(|i| {
        format!("{}{}", &str[..i], &str[(i + 1)..]).parse::<i32>().unwrap()
    }).max().unwrap_or(0)
}
/* Ticket numbers usually consist of an even number of digits. A ticket number is considered lucky if the sum of the first half of the digits is equal to the sum of the second half.

Given a ticket number n, determine if it's lucky or not.

Example

For n = 1230, the output should be
isLucky(n) = true;
For n = 239017, the output should be
isLucky(n) = false. */

fn isLucky(n: i32) -> bool {
    let number = n.to_string();
    let half = number.len() / 2;
    number.chars().take(half).map(|s| s.to_digit(10).unwrap()).sum::<u32>()
    .eq(&number.chars().rev().take(half).map(|x| x.to_digit(10).unwrap()).sum::<u32>())
}

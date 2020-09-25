/* Let's define digit degree of some positive integer as the number of times we need to replace this number with the sum of its digits until we get to a one digit number.

Given an integer, find its digit degree.

Example

For n = 5, the output should be
digitDegree(n) = 0;
For n = 100, the output should be
digitDegree(n) = 1. */

fn digitDegree(n: i32) -> i32 {
    let mut st = n.to_string();
    let mut dg = 0;
    while st.len() > 1 {
        st = st.chars().fold(0, |acc, c| acc + c.to_digit(10).unwrap()).to_string();
        dg += 1;
    }
    dg
}
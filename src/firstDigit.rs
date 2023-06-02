/* Find the leftmost digit that occurs in a given string.

Example

For inputString = "var_1__Int", the output should be
firstDigit(inputString) = '1'; */

fn firstDigit(s: String) -> char {
    s.chars().filter(|c| (*c as u8) > 47 && (*c as u8) <= 57).next().unwrap()
}
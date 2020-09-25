/* Given a string, output its longest prefix which contains only digits.

Example

For inputString = "123aa1", the output should be
longestDigitsPrefix(inputString) = "123". */

fn longestDigitsPrefix(s: String) -> String {
    s.chars().take_while(|c| c.is_digit(10)).collect()
}
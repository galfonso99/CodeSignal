/* Given two strings, find the number of common characters between them.

Example

For s1 = "aabcc" and s2 = "adcaa", the output should be
commonCharacterCount(s1, s2) = 3.

Strings have 3 common characters - 2 "a"s and 1 "c". */

fn commonCharacterCount(s1: String, mut s2: String) -> i32 {
    let mut counter = 0;
    for c in s1.chars() {
        for ch in s2.chars() {
            if c == ch {
                counter += 1;
                s2 = s2.replacen(ch, "", 1);
                break;
            }
        }
    }
    counter
}


/* Given a string, find the shortest possible string which can be achieved by adding characters to the end of initial string to make it a palindrome.

Example

For st = "abcdc", the output should be
buildPalindrome(st) = "abcdcba". */

fn buildPalindrome(mut st: String) -> String {
    let len = st.len();
    for c in st.clone().chars() {
        if is_palin(st.clone()) {break;}
        st.insert(len, c);
    }
    st
}

fn is_palin(s: String) -> bool {
    s.chars().rev().collect::<String>() == s
}
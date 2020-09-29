/* You are given two strings s and t of the same length, consisting of uppercase English letters. Your task is to find the minimum number of "replacement operations" needed to get some anagram of the string t from the string s. A replacement operation is performed by picking exactly one character from the string s and replacing it by some other character. */

fn createAnagram(s: String, mut t: String) -> i32 {
    for c in s.chars() {
        t= t.replacen(&c.to_string(), "", 1);
    }
    t.len() as i32
}
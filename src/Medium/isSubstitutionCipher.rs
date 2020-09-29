/* A ciphertext alphabet is obtained from the plaintext alphabet by means of rearranging some characters. For example "bacdef...xyz" will be a simple ciphertext alphabet where a and b are rearranged.

A substitution cipher is a method of encoding where each letter of the plaintext alphabet is replaced with the corresponding (i.e. having the same index) letter of some ciphertext alphabet.

Given two strings, check whether it is possible to obtain them from each other using some (possibly, different) substitution ciphers. */

fn isSubstitutionCipher(s1: String, s2: String) -> bool {
    use std::collections::HashSet;
    s1.chars().zip(s2.chars()).collect::<HashSet<(char,char)>>().len() ==
    s1.chars().collect::<HashSet<char>>().len() && s2.chars().collect::<HashSet<char>>().len() ==
    s1.chars().collect::<HashSet<char>>().len()
}
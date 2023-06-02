/* Given an array of strings, return another array containing all of its longest strings.

Example

For inputArray = ["aba", "aa", "ad", "vcd", "aba"], the output should be
allLongestStrings(inputArray) = ["aba", "vcd", "aba"]. */

fn allLongestStrings(inputArray: Vec<String>) -> Vec<String> {
    let max: usize = inputArray.iter().map(|x| x.len()).max().unwrap();
    inputArray.into_iter().filter(|x| x.len() == max).collect()
}
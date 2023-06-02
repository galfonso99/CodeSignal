/* Given an array of integers, replace all the occurrences of elemToReplace with substitutionElem.

Example

For inputArray = [1, 2, 1], elemToReplace = 1, and substitutionElem = 3, the output should be
arrayReplace(inputArray, elemToReplace, substitutionElem) = [3, 2, 3]. */

fn arrayReplace(input: Vec<i32>, r: i32, s: i32) -> Vec<i32> {
    input.iter().map(|&x| if x == r {s} else {x}).collect()
}
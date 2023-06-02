/* Given an array of integers, find the maximal absolute difference between any two of its adjacent elements.

Example

For inputArray = [2, 4, 1, 0], the output should be
arrayMaximalAdjacentDifference(inputArray) = 3 */

fn arrayMaximalAdjacentDifference(arr: Vec<i32>) -> i32 {
    arr.windows(2).map(|tup| (tup[0]-tup[1]).abs()).max().unwrap()
}

/* Given array of integers, find the maximal possible sum of some of its k consecutive elements.

Example

For inputArray = [2, 3, 5, 1, 6] and k = 2, the output should be
arrayMaxConsecutiveSum(inputArray, k) = 8 */

fn arrayMaxConsecutiveSum(a: Vec<i32>, k: i32) -> i32 {
    a.windows(k as usize).map(|slice| slice.iter().sum()).max().unwrap()
}
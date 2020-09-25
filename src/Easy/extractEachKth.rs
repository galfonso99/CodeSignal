/* Given array of integers, remove each kth element from it.

Example

For inputArray = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] and k = 3, the output should be
extractEachKth(inputArray, k) = [1, 2, 4, 5, 7, 8, 10]. */

fn extractEachKth(arr: Vec<i32>, k: i32) -> Vec<i32> {
    arr.into_iter().enumerate().filter(|(i,j)| (i+1) % k as usize > 0).map(|(i,j)| j).collect::<Vec<i32>>()
}

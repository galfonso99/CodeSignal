/* Given array of integers, for each position i, search among the previous positions for the last (from the left) position that contains a smaller value. Store this value at position i in the answer. If no such value can be found, store -1 instead.

Example

For items = [3, 5, 2, 4, 5], the output should be
arrayPreviousLess(items) = [-1, 3, -1, 2, 4]. */

fn arrayPreviousLess(items: Vec<i32>) -> Vec<i32> {
    items.clone().into_iter().enumerate().map(|(i,x)| 
    items.clone().into_iter().take(i).rfind(|&a| a < x).unwrap_or(-1)).collect()
}
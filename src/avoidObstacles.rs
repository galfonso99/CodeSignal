/* You are given an array of integers representing coordinates of obstacles situated on a straight line.

Assume that you are jumping from the point with coordinate 0 to the right. You are allowed only to make jumps of the same length represented by some integer.

Find the minimal length of the jump enough to avoid all the obstacles.

Example

For inputArray = [5, 3, 6, 7, 9], the output should be
avoidObstacles(inputArray) = 4. */

fn avoidObstacles(a: Vec<i32>) -> i32 {
    (2..).find(|x| a.iter().all(|i| i % x != 0)).unwrap()
}

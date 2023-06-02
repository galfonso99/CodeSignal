/* You are given an array of up to four non-negative integers, each less than 256.

Your task is to pack these integers into one number M in the following way:

The first element of the array occupies the first 8 bits of M;
The second element occupies next 8 bits, and so on.
Return the obtained integer M.

Note: the phrase "first bits of M" refers to the least significant bits of M - the right-most bits of an integer. For further clarification see the following example.

Example

For a = [24, 85, 0], the output should be
arrayPacking(a) = 21784. */

fn arrayPacking(a: Vec<i32>) -> i32 {
    i32::from_str_radix(a.iter().rev().map(|i| format!("{:0>8b}", i)).collect::<Vec<String>>().iter().flat_map(|e| e.chars()).collect::<String>().as_str(), 2).unwrap()
}
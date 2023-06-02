/* Given an array of 2^k integers (for some integer k), perform the following operations until the array contains only one element:

On the 1st, 3rd, 5th, etc. iterations (1-based) replace each pair of consecutive elements with their sum;
On the 2nd, 4th, 6th, etc. iterations replace each pair of consecutive elements with their product.
After the algorithm has finished, there will be a single element left in the array. Return that element. */

fn arrayConversion(mut arr: Vec<i32>) -> i32 {
    let mut i = 1;
    while arr.len() > 1 {
        if i%2==1 {
            arr = arr.chunks(2).map(|s| s[0] + s[1]).collect();
        }
        else {
            arr = arr.chunks(2).map(|s| s[0] * s[1]).collect();
        }
        i+=1;
    }
    arr[0]
}

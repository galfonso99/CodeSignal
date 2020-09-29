/* Given an integer product, find the smallest positive (i.e. greater than 0) integer the product of whose digits is equal to product. If there is no such integer, return -1 instead.

Example

For product = 12, the output should be
digitsProduct(product) = 26; 

Guaranteed constraints:
0 ≤ product ≤ 600.
*/

fn digitsProduct(product: i32) -> i32 {
    (1..999999).find(|c|
        c.to_string().chars().filter_map(|ch| ch.to_digit(10)).product::<u32>() == product as u32
    )
    .unwrap_or(-1)
}
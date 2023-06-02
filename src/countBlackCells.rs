/* Imagine a white rectangular grid of n rows and m columns divided into two parts by a diagonal line running from the upper left to the lower right corner. Now let's paint the grid in two colors according to the following rules:

A cell is painted black if it has at least one point in common with the diagonal;
Otherwise, a cell is painted white.
Count the number of cells painted black. */

fn countBlackCells(n: i32, m: i32) -> i32 {
    m + n + gcd( m, n) - 2
}
fn gcd(a: i32, b: i32) -> i32 {
    if a%b==0 {b} else {gcd(b, a%b)}
}
/* Two two-dimensional arrays are isomorphic if they have the same number of rows and each pair of respective rows contains the same number of elements.

Given two two-dimensional arrays, check if they are isomorphic. */

fn areIsomorphic(a1: Vec<Vec<i32>>, a2: Vec<Vec<i32>>) -> bool {
    a1.len() == a2.len() && 
    (0..a1.len()).all(|i| a1[i].len() == a2[i].len())
}
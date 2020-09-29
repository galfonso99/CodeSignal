/* Consider a (2k+1) × (2k+1) square subarray of an integer integers matrix. Let's call the union of the square's two longest diagonals, middle column and middle row a star. Given the coordinates of the star's center in the matrix and its width, rotate it 45 · t degrees clockwise preserving position of all matrix elements that do not belong to the star. 

For

matrix = [[1, 0, 0, 2, 0, 0, 3],
          [0, 1, 0, 2, 0, 3, 0],
          [0, 0, 1, 2, 3, 0, 0],
          [8, 8, 8, 9, 4, 4, 4],
          [0, 0, 7, 6, 5, 0, 0],
          [0, 7, 0, 6, 0, 5, 0],
          [7, 0, 0, 6, 0, 0, 5]]
width = 7, center = [3, 3], and t = 1, the output should be

starRotation(matrix, width, center, t) = 
[[8, 0, 0, 1, 0, 0, 2],
[0, 8, 0, 1, 0, 2, 0],
[0, 0, 8, 1, 2, 0, 0],
[7, 7, 7, 9, 3, 3, 3],
[0, 0, 6, 5, 4, 0, 0],
[0, 6, 0, 5, 0, 4, 0],
[6, 0, 0, 5, 0, 0, 4]]
                  
*/

fn starRotation(mut m: Vec<Vec<i32>>, w: i32, center: Vec<i32>, t: i32) -> Vec<Vec<i32>> {
    let c = (center[0] as usize, center[1] as usize);
    for _ in 0..(t as usize%8) {
        for i in 1..=( w/2) as usize {
            let tmp = m[c.0-i][c.1];
            m[c.0-i][c.1] = m[c.0-i][c.1-i];
            m[c.0-i][c.1-i] = m[c.0][c.1-i];
            m[c.0][c.1-i] = m[c.0+i][c.1-i];
            m[c.0+i][c.1-i] = m[c.0+i][c.1];
            m[c.0+i][c.1] = m[c.0+i][c.1+i];
            m[c.0+i][c.1+i] = m[c.0][c.1+i];
            m[c.0][c.1+i] = m[c.0-i][c.1+i];
            m[c.0-i][c.1+i] = tmp;
        }
    }
    m
}

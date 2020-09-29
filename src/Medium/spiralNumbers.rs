/* Construct a square matrix with a size N × N containing integers from 1 to N * N in a spiral order, starting from top-left and in clockwise direction.

Example

For n = 3, the output should be

spiralNumbers(n) = [[1, 2, 3],
                    [8, 9, 4],
                    [7, 6, 5]] */

fn spiralNumbers(m: i32) -> Vec<Vec<i32>> {
    //n=north, w=west, e=east, s=south
    let (mut n, mut w, mut e, mut s) = (0_usize,0_usize,m as usize,m as usize);
    let mut value = 1;
    let mut spiral:Vec<Vec<i32>> = vec![vec![0;e];e];
    while w < e && n<s {
        for i in w..e {
            spiral[n][i] = value;
            value+=1;
        }
        n+=1;
        for i in n..s {
            spiral[i][e-1] = value;
            value+=1;
        }
        e-=1;
        for i in (w..e).rev() {
            spiral[s-1][i] = value;
            value+=1;
        }
        s-=1;
        for i in (n..s).rev() {
            spiral[i][w] = value;
            value+=1;
        }
        w+=1;
        
        
    }
    spiral
}

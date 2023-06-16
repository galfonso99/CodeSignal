fn solution(mut m: Vec<Vec<i32>>, w: i32, center: Vec<i32>, t: i32) -> Vec<Vec<i32>> {
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
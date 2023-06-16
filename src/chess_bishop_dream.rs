fn solution(boardSize: Vec<i32>, initPosition: Vec<i32>, mut initDirection: Vec<i32>, k: i32) -> Vec<i32> 
{
    let sx = boardSize[0];
    let sy = boardSize[1];
    let mut mx = k % (sx * 2);
    let mut my = k % (sy * 2);
    
    let mut fx = initPosition[0];
    let mut fy = initPosition[1];
    
    walk_this_way(&mut fx, initDirection[0], mx, sx);
    walk_this_way(&mut fy, initDirection[1], my, sy);
    
    vec![fx, fy]
}
fn walk_this_way(pos: &mut i32, mut idir: i32, mut m: i32, size: i32)
{
    while m > 0
    {
        *pos += idir;
        if *pos >= size || *pos < 0
        {
            idir = -idir;
            *pos += idir;
        }
        m -= 1;
    }
}
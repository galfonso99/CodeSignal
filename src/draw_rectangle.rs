fn solution(mut m: Vec<Vec<char>>, pts: Vec<i32>) -> Vec<Vec<char>> {
    m[pts[1] as usize].iter_mut().enumerate().for_each(|(i,c)| {
        if (i as i32) == pts[0] || (i as i32) == pts[2] {*c = '*';}
        else if (i as i32) > pts[0] && (i as i32) < pts[2] {*c = '-';}
    });
    m[pts[3] as usize].iter_mut().enumerate().for_each(|(i,c)| {
        if (i as i32) == pts[0] || (i as i32) == pts[2] {*c = '*';}
        else if (i as i32) > pts[0] && (i as i32) < pts[2] {*c = '-';}
    });
    ((pts[1] as usize + 1)..pts[3] as usize).for_each(|i| {
        m[i][pts[0] as usize] = '|';
        m[i][pts[2] as usize] = '|';
    });
    m
}
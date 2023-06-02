/* You are implementing a command-line version of the Paint app. Since the command line doesn't support colors, you are using different characters to represent pixels. Your current goal is to support rectangle x1 y1 x2 y2 operation, which draws a rectangle that has an upper left corner at (x1, y1) and a lower right corner at (x2, y2). Here the x-axis points from left to right, and the y-axis points from top to bottom.

Given the initial canvas state and the array that represents the coordinates of the two corners, return the canvas state after the operation is applied. For the details about how rectangles are painted, see the example. */

fn drawRectangle(mut m: Vec<Vec<char>>, pts: Vec<i32>) -> Vec<Vec<char>> {
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
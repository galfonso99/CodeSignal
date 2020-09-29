/* A rectangle with sides equal to even integers a and b is drawn on the Cartesian plane. Its center (the intersection point of its diagonals) coincides with the point (0, 0), but the sides of the rectangle are not parallel to the axes; instead, they are forming 45 degree angles with the axes.

How many points with integer coordinates are located inside the given rectangle (including on its sides)? */

fn rectangleRotation(a: i32, b: i32) -> i32 {
    let y = (a as f32 / (2.0f32.sqrt())).ceil() as i32;
    let x = (b as f32 / (2.0f32.sqrt())).ceil() as i32;
    let mut res = y*x+((y-1)*(x-1));
    if res%2==1 {res} else {res-1}
    
}
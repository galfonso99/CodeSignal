/* Get the area of an nth-interesting polygon */


fn shape_area(n: i32) -> i32 {
    (1..n).map(|x| x*4).sum::<i32>()+1
}


pub fn run() {
    println!("{}", shape_area(4));
}
fn solution(mut s: Vec<i32>) -> Vec<i32> {
    let i = s.iter().position(|&x| x==(s.iter().sum::<i32>()-x)).unwrap();
    s.remove(i);
    s.sort();
    s
}
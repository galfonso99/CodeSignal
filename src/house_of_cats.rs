fn solution(l: i32) -> Vec<i32> {
    (0..=l/2).filter(|&x| (l-x*2)%4 == 0).collect()
}
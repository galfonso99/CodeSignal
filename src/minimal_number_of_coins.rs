fn solution(coins: Vec<i32>, mut price: i32) -> i32 {
    let mut cnt = 0;
    while price > 0 {
        let coin = *coins.iter().rev().find(|&&x| x <= price).unwrap();
        price -= coin;
        cnt+=1;
    }
    cnt
}
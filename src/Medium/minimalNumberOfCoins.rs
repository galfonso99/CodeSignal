/* You find yourself in Bananaland trying to buy a banana. You are super rich so you have an unlimited supply of banana-coins, but you are trying to use as few coins as possible.

The coin values available in Bananaland are stored in a sorted array coins. coins[0] = 1, and for each i (0 < i < coins.length) coins[i] is divisible by coins[i - 1]. Find the minimal number of banana-coins you'll have to spend to buy a banana given the banana's price. */

fn minimalNumberOfCoins(coins: Vec<i32>, mut price: i32) -> i32 {
    let mut cnt = 0;
    while price > 0 {
        let coin = *coins.iter().rev().find(|&&x| x <= price).unwrap();
        price -= coin;
        cnt+=1;
    }
    cnt
}
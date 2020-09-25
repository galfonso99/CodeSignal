/* You have deposited a specific amount of money into your bank account. Each year your balance increases at the same growth rate. With the assumption that you don't make any additional deposits, find out how long it would take for your balance to pass a specific threshold.

Example

For deposit = 100, rate = 20, and threshold = 170, the output should be
depositProfit(deposit, rate, threshold) = 3 */

fn depositProfit(deposit: i32, rate: i32, ths: i32) -> i32 {
    ( (ths as f32 / deposit as f32 ).log10() / (1.0+(rate as f32 / 100.0)).log10() ).ceil() as i32
}

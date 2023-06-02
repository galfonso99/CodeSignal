/* Consider a sequence of numbers a0, a1, ..., an, in which an element is equal to the sum of squared digits of the previous element. The sequence ends once an element that has already been in the sequence appears again.

Given the first element a0, find the length of the sequence. */

fn squareDigitsSequence(a0: i32) -> i32 {
    let (mut temp, mut sum, mut cnt) = (0, a0, 1);
    let mut arr = vec![a0];
    while !arr.contains(&temp) {
        arr.push(temp);
        temp = 0;
        for c in sum.to_string().chars() {
            temp += (c.to_digit(10).unwrap() as i32).pow(2);
        }
        sum = temp;
        cnt+=1;
    } 
    cnt
}

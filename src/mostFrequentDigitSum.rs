/* A step(x) operation works like this: it changes a number x into x - s(x), where s(x) is the sum of x's digits. You like applying functions to numbers, so given the number n, you decide to build a decreasing sequence of numbers: n, step(n), step(step(n)), etc., with 0 as the last element.

Building a single sequence isn't enough for you, so you replace all elements of the sequence with the sums of their digits (s(x)). Now you're curious as to which number appears in the new sequence most often. If there are several answers, return the maximal one. */

fn mostFrequentDigitSum(n: i32) -> i32 {
    let mut a = (1..).scan(n, |st, x| {
        let sum = sum(*st);
        *st = *st - sum;
        Some(sum)
    }).take_while(|x| x>&0).collect::<Vec<i32>>();
    a.sort();
    *a.iter().max_by(|x,y| count(&a,*x).cmp(&count(&a,*y))).unwrap() as i32
}

fn sum (n:i32) -> i32 {
    n.to_string().chars().filter_map(|c| c.to_digit(10)).sum::<u32>() as i32
}

fn count(a:&[i32], n:&i32) -> i32 {
    a.to_vec().iter().filter(|&x| x == n).count() as i32
}
fn solution(mut n: i32) -> i32 {
    let (mut carry, mut f) = (0,1);
    while n >= 10 {
        if (n % 10) + carry < 5 {carry = 0;} else {carry = 1;}
        n/=10; f*=10;
    }
    (n+carry) * f
}

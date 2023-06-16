fn solution(part: String, total: String) -> Vec<i32> {
    let prt = to_secs(part);
    let tot = to_secs(total);
    let gcd = gcd(prt,tot);
    vec![prt / gcd, tot / gcd]
}
fn gcd (a:i32, b:i32) -> i32 {
    if b==0 {return a}
    gcd(b,a%b)
}

fn to_secs(st: String) -> i32 {
    let mut time = st.split(':').map(|s| s.parse::<i32>().ok().unwrap());
    time.next().unwrap() * 3600 + time.next().unwrap() * 60 + time.next().unwrap()
}
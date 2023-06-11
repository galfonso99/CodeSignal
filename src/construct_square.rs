use std::collections::HashMap;


fn gen_squares() -> impl Iterator<Item=i32> {
    (0..).map(|c| c*c)
}

fn digit_len(d: i32) -> usize {
    d.to_string().len()
}

fn equal_amount(d: i32, s: &str) -> bool {
    fn inner(s: &str) -> HashMap<char, usize> {
        s.chars().fold(HashMap::new(), |mut e, c| {*e.entry(c).or_insert(0) += 1; e})
    }
    
    let ad = inner(&d.to_string());
    let mut sd = inner(s);
    
    for e in ad.values() {
        match sd.values_mut().find(|v| v == &e) {
            Some(mut x) => *x = 0, //avoid duplicates
            None => return false,
        }
    }
    
    true
}

fn solution(s: String) -> i32 {
    let len = s.len();
    
    gen_squares()
        .skip_while(|d| digit_len(*d) < len)
        .take_while(|d| digit_len(*d) == len)
        .filter(|d| equal_amount(*d, &s))
        .last().unwrap_or(-1)
}

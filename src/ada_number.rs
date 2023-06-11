fn solution(mut s: String) -> bool {
    if s.replace("_", "").parse::<u128>().is_ok() {true}
    else if s.matches('#').count() == 2 {
        let radix = s.chars().take_while(|&c| c!='#').filter(|&c| c!='_').collect::<String>().parse::<i32>();
        let digits = s.chars().skip_while(|&c| c!='#').skip(1)
        .take_while(|&c| c!='#').filter(|&c| c!='_').collect::<String>();
        if radix.is_ok() && !digits.is_empty() && radix.as_ref().unwrap() > &1 && radix.as_ref().unwrap() < &17 {
            digits.chars().all(|d| d.to_digit(*radix.as_ref().unwrap() as u32).is_some())
        }
        else {false}
    }
    else {false}
}
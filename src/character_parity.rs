fn solution(c: char) -> String {
    if c.to_digit(10).is_some() {
        if c.to_digit(10).unwrap() % 2 == 0{
            "even".to_string()
        }
        else {"odd".to_string()}
    }
    else {"not a digit".to_string()}
}
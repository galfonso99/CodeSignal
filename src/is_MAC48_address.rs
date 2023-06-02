fn solution(s: String) -> bool {
    let split = s.split('-').collect::<Vec<&str>>();    
    split.len()==6 && split.into_iter().all(|s| s.len() == 2 && s.chars().all(|x|x.is_digit(16)))
}

fn solution(str: String) -> i32 {
    let mut in_string = false;
    str.split(|c: char| !c.is_alphanumeric() && c != '"').filter(|&s| !s.is_empty() &&
        match s {
            "\"" => {in_string = !in_string; return !in_string;},  
            str if str.starts_with('"') && str.ends_with('"') => true,
            str if str.contains('"') => {in_string = !in_string; return !in_string;},
            s if in_string => false,
            s => true
        }
    ).count() as i32
}
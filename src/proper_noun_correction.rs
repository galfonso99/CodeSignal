fn solution(noun: String) -> String {
    noun.chars().enumerate().map(|(i,c)| if i==0 {c.to_ascii_uppercase()} else {c.to_ascii_lowercase()}).collect()
}

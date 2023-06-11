fn solution(tag: String) -> String {
    format!("</{}>", tag.split(|c:char| !c.is_ascii_alphanumeric()).skip(1).next().unwrap().to_string())
}
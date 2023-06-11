fn solution(s: String) -> String {
    let mut char = s.chars().nth(0).unwrap();
    let mut count = 0;
    let mut string = "".to_string();
    for c in s.chars() {
        if c == char {
            count += 1;
        } else {
            if count > 1 {
                string.push_str(count.to_string().as_str());
            }
            string.push(char);
            count = 1;
            char = c;
        }
    }
    if count > 1 {
        string.push_str(count.to_string().as_str());
    }
    string.push(char);
    return string;
}

fn solutio2(s: String) -> String {
    let mut v: Vec<String> = vec![];
    let mut len = 0;
    let mut it = s.chars().peekable(); 
    while let Some(c) = it.next() {
        len += 1;
        let next = it.peek();
        println!("Found: {}, next is {:?}", c, next);
        if next.is_none() || next != Some(&c) {
            let part: String = 
                if len == 1 { c.to_string() }
                else { format!("{}{}", len, c) };
            println!("Added {}", part);
            v.push(part);
            len = 0
        }
    }
    v.concat()
}
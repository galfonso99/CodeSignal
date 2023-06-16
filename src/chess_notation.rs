/*
Split by '/' then reverse the order of columns , then expand each column so that the string is padded with spaces
    equal to the number in the string so RP4pr becomes RP____pr
then for each character in the column push it into the equivalent row so the 4th char in the col would go into row 4
Then shrink each string back to using numbers so RP____pr would become RP4pr again
Then just join each string with '/' in between
*/

fn solution(notation: String) -> String {
    let mut rotated: Vec<String> = vec!["".to_string(); 8]; 
    for (i, line) in notation.rsplit('/').enumerate() {
        let mut expanded = expand(line);
        for (ind, char) in expanded.chars().enumerate() {
            rotated[ind].push(char);
        }
    }
    for s in rotated.iter_mut() {
        *s = condense(s);
    }
    rotated.join("/")
}

fn expand (sr: &str) -> String {
    let mut temp = String::new();
    for (ind, c) in sr.chars().enumerate() {
        match c.is_numeric() {
            true => temp.push_str(&" ".to_string()
                    .repeat(c.to_digit(10).unwrap() as usize)),
            false => temp.push(c)
        }
    }
    return temp;
}

fn condense (str: &str) -> String {
    let mut temp = String::new();
    let mut cnt = 0;
    for (ind, c) in str.chars().enumerate() {
        match c {
            ' ' => cnt += 1,
            _ => {
                if cnt > 0 {
                    temp.push(char::from_digit(cnt,10).unwrap());
                    cnt = 0;
                }
                temp.push(c);
            }
        }
    }
    if cnt > 0 {temp.push(char::from_digit(cnt,10).unwrap());}
    return temp;
}
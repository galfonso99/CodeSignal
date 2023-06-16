fn solution(crypt: Vec<String>, solution: Vec<Vec<char>>) -> bool {
    use std::collections::HashMap;
    let dic = solution.iter().map(|sl| (sl[0], sl[1])).collect::<HashMap<char,char>>();
    let convert = |s:&String| {
        let tmp = s.chars().filter_map(|c| dic.get(&c)).collect::<String>();
        let ans = if !tmp.starts_with('0') || tmp=="0" {tmp} else {"NAN".to_string()};
        ans.parse::<i64>().unwrap_or(-1001)
    };
    convert(&crypt[0]) + convert(&crypt[1]) == convert(&crypt[2])
}

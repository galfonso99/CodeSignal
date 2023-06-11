fn solution(n: i32) -> bool {
    n.to_string().chars().skip_while(|&c| c != '0').any(|c| c!='0')
}
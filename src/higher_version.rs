fn solution(v1: String, v2: String) -> bool {
    let tup = v1.split('.').zip(v2.split('.')).map(|(a,b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
    .find(|t| t.0!=t.1).unwrap_or((0,1));
    tup.0 > tup.1
}
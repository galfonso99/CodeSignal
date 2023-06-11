fn solution(mut shoes: Vec<Vec<i32>>) -> bool {
    use std::collections::HashMap;
    let mut a = HashMap::new();
    shoes.iter().for_each(|sl| {
        let x = if sl[0]==0 {-1} else {1};
        *a.entry(sl[1]).or_insert(0) += x;
    });
    a.iter().all(|t| t.1 == &0)
    
}
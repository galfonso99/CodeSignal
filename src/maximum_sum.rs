fn solution(mut a: Vec<i32>, q: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    //Spread the ranges so [0,3] becomes [0,1,2,3]
    let indices = q.iter().flat_map(|rng| (rng[0] as usize..=rng[1] as usize)).collect::<Vec<usize>>();
    let mut map: HashMap<usize,u32> = HashMap::new();
    //Count the indices to then sort by most common
    for i in indices {
        *map.entry(i).or_insert(0)+=1;
    }
    let mut arr = map.into_iter().collect::<Vec<(usize,u32)>>();
    //Sort indices
    arr.sort_by(|a, b| b.1.cmp(&a.1));
    //Sort the values by highest to lowest
    a.sort_by(|a, b| b.cmp(&a));
    let mut result = 0;
    let mut i = 0;
    //For each value (desc order) multiply times the count of frequent indices (sorted)
    for (_,m) in arr {
        result += a[i] * m as i32;
        i+=1;
    }
    result
}
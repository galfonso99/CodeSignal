fn solution(evi: Vec<Vec<i32>>) -> bool {
    (0..evi[0].len()).map(|i| (0..evi.len()).map(|j| evi[j][i]).collect::<Vec<i32>>()).all(|x| consistent(x) )
}

fn consistent (mut vec: Vec<i32>) -> bool {
    vec = vec.into_iter().filter(|&x| x!=0).collect::<Vec<i32>>();
    vec.iter().sum::<i32>().abs() == vec.len() as i32
}
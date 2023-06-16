fn solution(size: i32, nono: Vec<Vec<String>>) -> bool {
    let extra = nono.len() - size as usize;
    nono.iter().skip(extra).all(|a| stripe(a.to_vec(), extra)) &&
    (0..nono[0].len()).map(|i| (0..nono.len()).map(|j| (&nono[j][i]).to_string()).collect::<Vec<String>>())
    .skip(extra).all(|a| stripe(a.to_vec(), extra))
}

fn stripe (run: Vec<String>, extra: usize) -> bool {
    let (nums, colors) = run.split_at(extra);
    let nums_arr = nums.into_iter().skip_while(|s| *s=="-").filter_map(|s| s.parse::<u32>().ok()).collect::<Vec<u32>>();
    let colors_arr = colors.split(|s| s == ".").filter(|arr| !arr.is_empty()).map(|ar| ar.to_owned()).collect::<Vec<Vec<String>>>();
    if nums_arr.len() != colors_arr.len() {return false;}
    (0..nums_arr.len()).all(|i| nums_arr[i] == colors_arr[i].len() as u32)
}
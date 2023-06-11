fn solution(n: i32) -> Vec<i32> {
    let mut divs = Vec::new();
    let mut weak = Vec::new();
    for i in 1..=n {
        let mut cnt = 0;
        for j in 1..=i {
            if i%j==0 {cnt+=1;}
        } 
        divs.push(cnt);
        weak.push(weakness(cnt, &divs));
        
    }
    let max :i32= *weak.iter().max().unwrap();
    let cnt = weak.iter().filter(|&x| x==&max).count() as i32;
    vec![max, cnt]
}
fn weakness(n: i32, slc: &[i32]) -> i32 {
    slc.iter().filter(|&x| x>&n).count() as i32
}


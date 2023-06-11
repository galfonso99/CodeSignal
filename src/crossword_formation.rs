fn count_cw(a: &Vec<String>) -> i32 {
    let mut cnt = 0;
    for i0 in 0 .. a[0].len() - 2 {
        for i1 in 0 .. a[1].len() - 2 {
            if a[0].as_bytes()[i0] != a[1].as_bytes()[i1] { continue }
            for j0 in i0 + 2 .. a[0].len() {
                for j2 in 0 .. a[2].len() - 2 {
                    if a[0].as_bytes()[j0] != a[2].as_bytes()[j2] { continue }
                    for k1 in i1 + 2 .. a[1].len() {
                        for k3 in 0 .. a[3].len() - 2 {
                            if a[1].as_bytes()[k1] != a[3].as_bytes()[k3] { continue }
                            let l2 = j2 + k1 - i1;
                            let l3 = k3 + j0 - i0;
                            if l2 >= a[2].len() { continue }
                            if l3 >= a[3].len() { continue }
                            if a[3].as_bytes()[k3 + j0 - i0] == a[2].as_bytes()[j2 + k1 - i1] { cnt += 1; }
                        }
                    }
                }
            }
        }
    }
    cnt
}

fn permut(n: usize, mut a: &mut Vec<String>) -> i32 {
    let mut cnt = 0_i32;
    if n == 1 {
        cnt += count_cw(a);
    } else {
        for i in 0 .. n-1 {
            cnt += permut(n-1, &mut a);
            if n % 2 == 0 {
                a.swap(i, n-1);
            } else {
                a.swap(0, n-1);
            }
        }
        cnt += permut(n-1, &mut a);
    }
    cnt
}

fn solution(words: Vec<String>) -> i32 {
    permut(words.len(), &mut words.clone())
}

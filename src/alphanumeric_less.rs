fn solution(s1: String, s2: String) -> bool { 
    let (mut i, mut j) = (0, 0);
    let (il, jl) = (s1.len(), s2.len());
    let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
    let mut zeroes = (0, true, true);
    
    while i < il && j < jl { 
        if s1[i] < 0x40 && s2[i] < 0x40 { // Number
            if zeroes.0 == 0 {
                zeroes.1 = true;
                zeroes.2 = true;
            }
            let (mut x, mut y) = (s1[i] as u32 - 0x30, s2[j] as u32 - 0x30);
            while i + 1 < il && s1[i + 1] < 0x40 { 
                if zeroes.1 && s1[i + 1] == 0x30 {  zeroes.0 += 1; } 
                else { zeroes.1 = false; }
                x = x * 10 + s1[i + 1] as u32 - 0x30;
                i += 1;
            }
            while j + 1 < jl && s2[j + 1] < 0x40 {
                if zeroes.2 && s2[j + 1] == 0x30 { zeroes.0 -= 1; } 
                else { zeroes.2 = false; }
                y = y * 10 + s2[j + 1] as u32 - 0x30;
                j += 1;
            }
            
            if x != y { return x < y;  }
        } 
        else if s1[i] != s2[j] {  return s1[i] < s2[j]; }
        i += 1;
        j += 1;
    }
    if (i == il) != (j == jl) { // If one is at the end but the other isn't, shortest wins
        i == il
    } else {
        zeroes.0 > 0 
    }
}

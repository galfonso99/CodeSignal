fn solution(mut table: Vec<String>, coords: Vec<Vec<i32>>) -> Vec<String> {
    let x1 = coords[0][0].min(coords[1][0]) as usize;
    let x2 = coords[0][0].max(coords[1][0]) as usize;
    let y1 = coords[0][1].min(coords[1][1]) as usize;
    let y2 = coords[0][1].max(coords[1][1]) as usize;

    let r1 = table.iter().enumerate().filter(|(_, v)| &v[..1] == "+").map(|(i, _)| i).skip(x1).next().unwrap();
    let r2 = table.iter().enumerate().filter(|(_, v)| &v[..1] == "+").map(|(i, _)| i).skip(x2+1).next().unwrap();
    let c1 = table[0].rmatch_indices('+').map(|(i, _)| i).rev().skip(y1).next().unwrap();
    let c2 = table[0].rmatch_indices('+').map(|(i, _)| i).rev().skip(y2+1).next().unwrap();

    let content_row = |s: &String| (&s[..=c1]).to_string() + &s[c1+1..c2].replace('|', " ") + &s[c2..];
    let horizontal_line = |s: &String, f: &str| (&s[..=c1]).to_string() + &f.repeat(c2 - c1 - 1) + &s[c2..];
    for i in r1+1..r2 {
        match &table[i][..1] == "+" {    
            true => table[i] = horizontal_line(&table[i], " "),   // inner line
            false => table[i] = content_row(&table[i]),   // content row
        }
        if c1 == 0 { table[i] = (&table[i][..c1]).to_string() + &"|" + &table[i][c1+1..] }
        if c2 + 1 == table[i].len() { table[i] = (&table[i][..c2]).to_string() + &"|" + &table[i][c2+1..] }
    }
    if r1 == 0 { table[r1] = horizontal_line(&table[r1], "-") }
    if r2 + 1 == table.len() { table[r2] = horizontal_line(&table[r2], "-") }
    
    table
}

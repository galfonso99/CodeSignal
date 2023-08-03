fn solution(program: Vec<String>) -> String {
    assert!(program[0].is_ascii());
    let height = program.len() as i32;
    let width = program[0].len() as i32;
    let mut string_res = String::new();
    let mut stack: Vec<i32> = Vec::new();
    let mut string_mode = false;
    let (mut i, mut j) = (0, 0);
    let mut moving_hor = true;
    let mut vert_dir: i32 = 1;
    let mut hor_dir: i32 = 1;
    let mut counter = 0;

    'outer: loop  {
        loop {
            i = ((i + height) % height) as i32;
            j = ((j + width) % width) as i32;
            let (i_usize, j_usize) = (i as usize, j as usize);
            let char = program[i_usize][j_usize..j_usize+1].chars().next().unwrap();
            match char {
                '\"' => {
                    string_mode = !string_mode;
                },
                ch if string_mode => {
                    stack.push((ch as u8) as i32);
                },
                '>' => {
                    hor_dir = 1;
                    moving_hor = true;
                }
                '<' => {
                    hor_dir = -1;
                    moving_hor = true;
                }
                'v' => {
                    vert_dir = 1;
                    moving_hor = false;
                }
                '^' => {
                    vert_dir = -1;
                    moving_hor = false;
                }
                '#' => {
                    if moving_hor {
                        j += hor_dir
                    } else {
                        i += vert_dir;
                        break;
                    }
                }
                '_' => {
                    let val = stack.pop().unwrap_or(0);
                    if val == 0 {
                        hor_dir = 1;
                        moving_hor = true;
                    } else {
                        hor_dir = -1;
                        moving_hor = true;
                    }
                }
                '|' => {
                    let val = stack.pop().unwrap_or(0);
                    if val == 0 {
                        vert_dir = 1;
                        moving_hor = false;
                    } else {
                        vert_dir = -1;
                        moving_hor = false;
                    }
                }
                '+' => {
                    let (a, b) = pop_two(&mut stack);
                    stack.push(a + b);
                }
                '-' => {
                    let (a, b) = pop_two(&mut stack);
                    stack.push(b - a);
                },
                '*' => {
                    let (a, b) = pop_two(&mut stack);
                    stack.push(a * b);
                },
                '/' => {
                    let (a, b) = pop_two(&mut stack);
                    stack.push(b / a);
                }, 
                '%' => {
                    let (a, b) = pop_two(&mut stack);
                    stack.push(b % a);
                },
                '!' => {
                    let val = stack.pop().unwrap_or(0);
                    stack.push((val == 0) as i32);
                }
                '`' => {
                    let (a, b) = pop_two(&mut stack);
                    stack.push((b > a) as i32);
                },
                ':' => {
                    let val = stack.pop().unwrap_or(0);
                    stack.push(val);
                    stack.push(val);
                },
                '\\' => {
                    let (a, b) = pop_two(&mut stack);
                    stack.push(a);
                    stack.push(b);
                },
                '$' => {
                    stack.pop();
                },
                '.' => {
                    let val = stack.pop().unwrap_or(0);
                    string_res.push_str(format!("{val} ").as_str());
                    if string_res.len() >= 100 {
                        string_res = string_res[0..100].to_string();
                        break 'outer;
                    }
                },
                ',' => {
                    let val = stack.pop().unwrap_or(0) as u32;
                    let ch = char::from_u32(val).unwrap_or('_');
                    string_res.push(ch);
                    if string_res.len() == 100 {break 'outer;}
                },
                '0'..='9' => { stack.push((char as u8 - b'0') as i32);},
                ' ' => (),
                '@' => {break 'outer;},
                _ => (),
            }
            counter += 1;
            if counter == 100000 {break 'outer;}
            if !moving_hor {break;}
            j += hor_dir;
        }
        i += vert_dir;
    }
    string_res
}

pub fn pop_two(stack: &mut Vec<i32>) -> (i32, i32) {
    let a = stack.pop().unwrap_or(0);
    let b = stack.pop().unwrap_or(0);
    (a,b)
}

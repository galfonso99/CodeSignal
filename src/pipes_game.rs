pub fn solution(state: Vec<String>) -> i32 {
    let mut locations = std::collections::HashMap::new();
    let mut paths = std::collections::HashMap::new();
    let mut crosses = std::collections::HashMap::new();
    let mut full_pipes = 0;
    let mut negative_if_incorrect: i32 = 1;

    struct Pipe<'a> {
        dir: &'a str,
        pos: (usize, usize),
    }

    let is_inside = |(x, y): (i32, i32)| {
        x < state[0].len() as i32 && x >= 0 && y < state.len() as i32 && y >= 0
    };
    for i in 0..state.len() {
        for j in 0..state[0].len() {
            let ch = char_from_slice(&state[i][j..j + 1]);
            if let 'a'..='z' = ch {
                locations.insert(ch, (j, i));
            }
        }
    }
    for (key, &(x, y)) in locations.iter() {
        let up_inside = is_inside((x as i32, y as i32 - 1));
        if up_inside {
            let up_pos = (x, y - 1);
            let up_char = char_from_slice(&state[y - 1][x..x + 1]);
            let next_dir = next_direction(up_pos, up_char, "up");
            if let Some(dir) = next_dir {
                paths.insert((key, "up"), Some(Pipe { dir, pos: up_pos }));
                full_pipes += 1;
                if up_char == '7' {
                    crosses.insert(up_pos, 1);
                }
            }
        }
        let right_inside = is_inside((x as i32 + 1, y as i32));
        if right_inside {
            let right_pos = (x + 1, y);
            let right_char = char_from_slice(&state[y][x + 1..x + 2]);
            let next_dir = next_direction(right_pos, right_char, "right");
            if let Some(dir) = next_dir {
                paths.insert( (key, "right"), Some(Pipe { dir, pos: right_pos }));
                full_pipes += 1;
                if right_char == '7' {
                    crosses.insert(right_pos, 1);
                }
            }
        }
        let down_inside = is_inside((x as i32, y as i32 + 1));
        if down_inside {
            let down_pos = (x, y + 1);
            let down_char = char_from_slice(&state[y + 1][x..x + 1]);
            let next_dir = next_direction(down_pos, down_char, "down");
            if let Some(dir) = next_dir {
                paths.insert((key, "down"), Some(Pipe { dir, pos: down_pos }));
                full_pipes += 1;
                if down_char == '7' {
                    crosses.insert(down_pos, 1);
                }
            }
        }
        let left_inside = is_inside((x as i32 - 1, y as i32));
        if left_inside {
            let left_pos = (x - 1, y);
            let left_char = char_from_slice(&state[y][x - 1..x]);
            let next_dir = next_direction(left_pos, left_char, "left");
            if let Some(dir) = next_dir {
                paths.insert((key, "left"), Some(Pipe { dir, pos: left_pos }));
                full_pipes += 1;
                if left_char == '7' {
                    crosses.insert(left_pos, 1);
                }
            }
        }
    }

    'outer: loop {
        let mut temp_full_pipes = 0;
        let mut functioning_pipes = 0;
        for (&(&key, dir), pipe) in paths.iter_mut() {
            if pipe.is_none() {continue;}
            let curr_pipe = pipe.as_ref().unwrap();
            let (x, y) = next_pos(curr_pipe.pos, curr_pipe.dir);
            let pos_inside = is_inside((x, y));
            if pos_inside {
                let next_char = char_from_slice(&state[y as usize][x as usize..x as usize + 1]);
                if reached_destination(key, next_char) {
                    *pipe = None;
                    continue;
                }
                let next_dir = next_direction(curr_pipe.pos, next_char, curr_pipe.dir);
                if let Some(dir) = next_dir {
                    *pipe = Some(Pipe { dir, pos: (x as usize, y as usize) });
                    temp_full_pipes += 1;
                    functioning_pipes += 1;
                    if next_char == '7' {
                        if let std::collections::hash_map::Entry::Vacant(e) = crosses.entry((x as usize, y as usize)) {
                            e.insert(1);
                        } else {
                            temp_full_pipes -= 1;
                        }
                    }
                } else {
                    negative_if_incorrect = -1;
                    break 'outer;
                }
            } else {
                negative_if_incorrect = -1;
                break 'outer;
            }
        }
        if functioning_pipes == 0 {
            break;
        }
        full_pipes += temp_full_pipes;
    }
    full_pipes * negative_if_incorrect
}
fn char_from_slice(slice: &str) -> char {
    slice.chars().next().unwrap()
}

fn reached_destination(key: char, next_char: char) -> bool {
    key.to_ascii_uppercase() == next_char
}

fn next_pos((x, y): (usize, usize), dir: &str) -> (i32, i32) {
    match dir {
        "up" => (x as i32, y as i32 - 1),
        "right" => (x as i32 + 1, y as i32),
        "down" => (x as i32, y as i32 + 1),
        "left" => (x as i32 - 1, y as i32),
        _ => (x as i32 + 1, y as i32)
    }
}

fn next_direction(pos: (usize, usize), char: char, dir: &str) -> Option<&str> {
    match dir {
        "up" => match char {
            '1' | '7' => Some("up"),
            '3' => Some("right"),
            '4' => Some("left"),
            _ => None,
        },
        "right" => match char {
            '2' | '7' => Some("right"),
            '4' => Some("down"),
            '5' => Some("up"),
            _ => None,
        },
        "down" => match char {
            '1' | '7' => Some("down"),
            '5' => Some("left"),
            '6' => Some("right"),
            _ => None,
        },
        "left" => match char {
            '2' | '7' => Some("left"),
            '3' => Some("down"),
            '6' => Some("up"),
            _ => None,
        },
        _ => None,
    }
}

// on first O(N) iteration find the position of the starting points and also all that pipes that connect to it (up, right, down, left)

// Rephrasing the second paragraph. With the location of the sources, find which paths are valid if any (up, right, down, left) for each source. Then for each path of each source try to step in the direction expected (only one), if you see a connection increment the full pipes count by 1, else drop that path and set negativeIfIncorrect to -1. Then at the end multiply result by negative_if_incorrect
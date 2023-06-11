fn solution(tree: String) -> Vec<i32> {
    let mut curr_depth: i32 = 0;
    let mut max_depth: i32 = 0;
    let values_and_depth = tree.rsplit(' ').flat_map(|s| {
        curr_depth += s.bytes().map(|byte| (byte, 0)).fold((b'$', 0),|(_, depth), (byte, _)| 
            match byte {
                b')' =>  (byte, depth+1),
                b'(' =>  (byte, depth-1),
                _ => (byte, depth)
            }
        ).1;
        max_depth = max_depth.max(curr_depth);
        if let Ok(val) = s.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<isize>() {
            Some((val, curr_depth+1, max_depth))
        }
        else { None }
    }).collect::<Vec<(isize, i32, i32)>>();
    max_depth = values_and_depth.last().unwrap().2;
    values_and_depth.iter().filter(|(_, depth, _)| *depth == max_depth).map(|(val, _, _)| *val as i32)
    .rev().collect()
}


// Most Important
// Doing it all in one iterator cant work
// Just find the max beforehand
// Then for each if curr_depth equal max and there's a number store that number in VecDeque at front
// Return Vec::from(vec_deque)


// Store an iter of tuples of (val, depth) where depth is how deep it was found
// Then using that iter find max_by_key using tuple.1 (depth) and store the max
// Then using the max filter the stored iter for tuples with max depth only
// Then map tp keep only the values then reverse then collect into Vec

// Might need to find the total deepest depth of the tree beforehand
// parsing the string from the right count how many closed but unopened parenthesis there are 
//  (substracting if you find a "(" and adding id you find a ")")
// If you encounter a value when ")" count is at the highest add that value to the array
// This is where knowing the depth beforehand would be useful

// To find the depth run through the string backwards and count the running total of 
// unopened closed parenthesis and keep only the hightest
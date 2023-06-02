/* You found two items in a treasure chest! The first item weighs weight1 and is worth value1, and the second item weighs weight2 and is worth value2. What is the total maximum value of the items you can take with you, assuming that your max weight capacity is maxW and you can't come back for the items later?

Note that there are only two items and you can't bring more than one item of each type, i.e. you can't take two first items or two second items */

fn knapsackLight(v1: i32, w1: i32, v2: i32, w2: i32, maxW: i32) -> i32 {
    if w1 + w2 <= maxW {return v1 + v2;}
    let mut st:Vec<i32> = Vec::new();
    if w1 <= maxW {st.push(v1);}
    if w2 <= maxW {st.push(v2);}
    if !st.is_empty() {return *st.iter().max().unwrap();}
    else {0}
}

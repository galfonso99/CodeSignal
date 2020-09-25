/* Some people are standing in a row in a park. There are trees between them which cannot be moved. Your task is to rearrange the people by their heights in a non-descending order without moving the trees. People can be very tall!

Example

For a = [-1, 150, 190, 170, -1, -1, 160, 180], the output should be
sortByHeight(a) = [-1, 150, 160, 170, -1, -1, 180, 190]. */

fn sortByHeight(a: Vec<i32>) -> Vec<i32> {
    let mut positive = a.iter().filter(|&&x| x > 0).cloned().collect::<Vec<i32>>();
    positive.sort();
    let mut iter = positive.iter();
    a.iter().map(|x| if x > &0 {iter.next().unwrap()} else {x}).cloned().collect()
}
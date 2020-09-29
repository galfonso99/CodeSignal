/* You are given an array of desired filenames in the order of their creation. Since two files cannot have equal names, the one which comes later will have an addition to its name in a form of (k), where k is the smallest positive integer such that the obtained name is not used yet.

Return an array of names that will be given to the files.

Example

For names = ["doc", "doc", "image", "doc(1)", "doc"], the output should be
fileNaming(names) = ["doc", "doc(1)", "image", "doc(1)(1)", "doc(2)"]. */

fn fileNaming(n: Vec<String>) -> Vec<String> {
    let mut arr = Vec::new();
    for ele in n.into_iter() {
        if !arr.contains(&ele) {arr.push(ele)}
        else {
            let mut cnt = 1;
            while arr.contains( &format!("{}({})", ele, cnt) ) {
                cnt += 1;
            }
            arr.push(format!("{}({})", ele, cnt));
        }
    }
    arr
}
/* Yesterday you found some shoes in the back of your closet. Each shoe is described by two values:

type indicates if it's a left or a right shoe;
size is the size of the shoe.
Your task is to check whether it is possible to pair the shoes you found in such a way that each pair consists of a right and a left shoe of an equal size. */

fn pairOfShoes(mut shoes: Vec<Vec<i32>>) -> bool {
    use std::collections::HashMap;
    let mut a = HashMap::new();
    shoes.iter().for_each(|sl| {
        let x = if sl[0]==0 {-1} else {1};
        *a.entry(sl[1]).or_insert(0) += x;
    });
    a.iter().all(|t| t.1 == &0)
    
}
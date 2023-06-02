/* There are some people and cats in a house. You are given the number of legs they have all together. Your task is to return an array containing every possible number of people that could be in the house sorted in ascending order. It's guaranteed that each person has 2 legs and each cat has 4 legs. */

fn houseOfCats(l: i32) -> Vec<i32> {
    (0..=l/2).filter(|&x| (l-x*2)%4 == 0).collect()
}
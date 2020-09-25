/* Given a rectangular matrix of characters, add a border of asterisks(*) to it.

Example

For

picture = ["abc",
           "ded"]
the output should be

addBorder(picture) = ["*****",
                      "*abc*",
                      "*ded*",
                      "*****"] */
                      
fn addBorder(mut picture: Vec<String>) -> Vec<String> {
    let l = picture[0].len()+2;
    picture = picture.iter().map(|x| ["*", x,"*"].concat()).collect::<Vec<_>>();
    picture.insert(0, (0..l).map(|_| "*").collect::<String>());
    picture.push((0..l).map(|_| "*").collect::<String>());
    picture
}

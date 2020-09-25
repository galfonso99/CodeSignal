/* Last night you partied a little too hard. Now there's a black and white photo of you that's about to go viral! You can't let this ruin your reputation, so you want to apply the box blur algorithm to the photo to hide its content.

The pixels in the input image are represented as integers. The algorithm distorts the input image in the following way: Every pixel x in the output image has a value equal to the average value of the pixel values from the 3 × 3 square that has its center at x, including x itself. All the pixels on the border of x are then removed.

Return the blurred image as an integer, with the fractions rounded down. */

/* For

image = [[7, 4, 0, 1], 
         [5, 6, 2, 2], 
         [6, 10, 7, 8], 
         [1, 4, 2, 0]]
the output should be

boxBlur(image) = [[5, 4], 
                  [4, 4]] */

fn boxBlur(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = image[0].len();
    let m = image.len();
    let mut blur = Vec::new();
    for j in 0..=m-3 {
        let mut arr = Vec::new();
        for i in 0..=n-3 {
            let mut acc = 0;
            for y in 0..3 {
                for x in 0..3 {
                    acc += image[y+j][x+i];
                }
            }
            arr.push(acc/9 as i32);
            acc = 0;
        };
        blur.push(arr.clone());
        arr.clear();
    }
    return blur;
}
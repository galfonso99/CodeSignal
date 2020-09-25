/* Call two arms equally strong if the heaviest weights they each are able to lift are equal.

Call two people equally strong if their strongest arms are equally strong (the strongest arm can be both the right and the left), and so are their weakest arms.

Given your and your friend's arms' lifting capabilities find out if you two are equally strong. */

use std::cmp::max;
use std::cmp::min;
fn areEquallyStrong(l1: i32, r1: i32, l2: i32, r2: i32) -> bool {
    max(l1,r1) == max(l2,r2) && min(l1,r1) == min(l2,r2)
}

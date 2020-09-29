/* Consider a string containing only letters and whitespaces. It is allowed to replace some (possibly, none) whitespaces with newline symbols to obtain a multiline text. Call a multiline text beautiful if and only if each of its lines (i.e. substrings delimited by a newline character) contains an equal number of characters (only letters and whitespaces should be taken into account when counting the total while newline characters shouldn't). Call the length of the line the text width.

Given a string and some integers l and r (l ≤ r), check if it's possible to obtain a beautiful text from the string with a text width that's within the range [l, r].

Example

For inputString = "Look at this example of a correct text", l = 5, and r = 15, the output should be
beautifulText(inputString, l, r) = true. */

fn beautifulText(s: String, l: i32, r: i32) -> bool {
    if l <= s.len() as i32 && s.len() as i32 <= r {return true;}
    for n in 2..=s.len()/l as usize {
        let mut spc = true;
        let len = s.len()/n;
        for x in 1..n {
            spc = s.chars().nth(x*len+x-1).unwrap() == ' ';
            if !spc {break;}
        }
        spc = spc && l<=len as i32 && len as i32<=r;
        if spc {return true}
    }
    return false
}